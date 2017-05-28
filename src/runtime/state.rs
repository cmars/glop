extern crate serde_json;
extern crate spoolq;

use std;
use std::collections::{HashMap, HashSet};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

use super::*;
use self::context::Context;
use self::transaction::Transaction;
use self::value::{Message, Value};

pub trait Storage {
    fn load(&mut self) -> Result<(i32, HashMap<String, Value>)>;
    fn save(&mut self, seq: i32, vars: HashMap<String, Value>) -> Result<()>;

    fn next_messages(&mut self,
                     filters: &HashSet<MessageFilter>)
                     -> Result<HashMap<String, Message>>;
    fn push_msg(&mut self, msg: Message) -> Result<()>;

    fn vars(&self) -> &HashMap<String, Value>;
    fn seq(&self) -> i32;

    fn workspace(&self) -> &str;
}

pub trait Outbox {
    fn send_msg(&self, msg: Message) -> Result<()>;
}

pub struct State<S: Storage> {
    name: String,
    storage: S,
    outbox: Box<Outbox + Send + 'static>,
}

impl<S: Storage> State<S> {
    pub fn new(name: &str, storage: S) -> State<S> {
        State {
            name: name.to_string(),
            storage: storage,
            outbox: Box::new(UndeliverableOutbox) as Box<Outbox + Send>,
        }
    }

    pub fn new_outbox(name: &str, storage: S, outbox: Box<Outbox + Send + 'static>) -> State<S> {
        State {
            name: name.to_string(),
            storage: storage,
            outbox: outbox,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn mut_storage(&mut self) -> &mut S {
        &mut self.storage
    }

    pub fn eval(&mut self, m: Match) -> Result<Option<Transaction>> {
        debug!("State.eval: {:?}", &m);
        let (seq, vars) = self.storage.load()?;
        let msgs = self.storage.next_messages(&m.filters())?;
        let ctx = Context::new(&self.name, vars, msgs, self.storage.workspace());
        let txn = Transaction::new(m, seq, ctx);
        if txn.eval() {
            debug!("State.eval: MATCHED");
            Ok(Some(txn))
        } else {
            // Re-enqueue messages that didn't match.
            debug!("State.eval: MISSED");
            let mut ctx = txn.ctx.lock().unwrap();
            for (_topic, msg) in ctx.msgs.drain() {
                self.storage.push_msg(msg)?;
            }
            Ok(None)
        }
    }

    pub fn commit(&mut self, txn: &mut Transaction) -> Result<i32> {
        debug!("State.commit: BEGIN transaction seq={}", txn.seq);
        let mut txn = txn;
        let mut vars = self.storage.vars().clone();
        let mut self_msgs = Vec::new();
        let actions = txn.apply()?;
        let matched_topics = txn.matched_topics();
        for action in actions {
            debug!(target: "State.commit", "action {:?}", action);
            match &action {
                &Action::SetVar(ref k, ref v) => {
                    k.set(&mut vars, Value::Str(v.to_string()));
                }
                &Action::UnsetVar(ref k) => {
                    k.unset(&mut vars);
                }
                &Action::SendMsg { ref dst_remote,
                                   ref dst_agent,
                                   ref topic,
                                   ref in_reply_to,
                                   ref contents } => {
                    let msg = Message::new(topic, contents.clone())
                        .src_agent(&self.name)
                        .src_role(txn.m.acting_role.clone())
                        .dst_agent(dst_agent)
                        .dst_remote(dst_remote.clone())
                        .in_reply_to(in_reply_to.clone());
                    debug!("send {:?}", msg);
                    if dst_agent == "self" {
                        self_msgs.push(msg);
                    } else {
                        self.outbox.send_msg(msg)?;
                    }
                }
                _ => return Err(Error::UnsupportedAction),
            };
        }
        let msgs = txn.with_context(|ctx| ctx.msgs.clone());
        for (topic, msg) in msgs {
            if !matched_topics.contains(&topic) {
                self.storage.push_msg(msg)?;
            }
        }
        for msg in self_msgs {
            self.storage.push_msg(msg)?;
        }
        self.storage.save(txn.seq, vars)?;
        debug!("State.commit: OK transaction seq={}", txn.seq);
        Ok(txn.seq)
    }

    pub fn rollback(&mut self, txn: Transaction) -> Result<()> {
        let mut txn = txn;
        let msgs = txn.with_context(|ctx| ctx.msgs.clone());
        for (_topic, msg) in msgs {
            self.storage.push_msg(msg)?;
        }
        Ok(())
    }
}

pub struct UndeliverableOutbox;

impl Outbox for UndeliverableOutbox {
    fn send_msg(&self, msg: Message) -> Result<()> {
        Err(Error::UndeliverableMessage(msg.dst_agent.to_string()))
    }
}

pub struct MemStorage {
    seq: i32,
    vars: HashMap<String, Value>,
    msgs: HashMap<MessageFilter, Vec<Message>>,
    workspace: String,
}

impl MemStorage {
    pub fn new() -> MemStorage {
        MemStorage {
            seq: 0,
            vars: HashMap::new(),
            msgs: HashMap::new(),
            workspace: std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

impl Storage for MemStorage {
    fn load(&mut self) -> Result<(i32, HashMap<String, Value>)> {
        Ok((self.seq, self.vars.clone()))
    }

    fn save(&mut self, seq: i32, vars: HashMap<String, Value>) -> Result<()> {
        if seq < self.seq {
            return Err(error::Error::InvalidArgument("stale transaction".to_string()));
        }
        debug!(target: "MemStorage.save", "vars before={:?} after={:?}", &self.vars, &vars);
        self.vars = vars;
        self.seq = seq + 1;
        Ok(())
    }

    fn next_messages(&mut self,
                     filters: &HashSet<MessageFilter>)
                     -> Result<HashMap<String, Message>> {
        let mut next: HashMap<String, Message> = HashMap::new();
        for (k, v) in &mut self.msgs {
            if !filters.contains(k) {
                continue;
            }
            match v.pop() {
                Some(msg) => {
                    next.insert(k.topic.to_string(), msg.clone());
                }
                None => (),
            }
        }
        Ok(next)
    }

    fn push_msg(&mut self, msg: Message) -> Result<()> {
        let k = MessageFilter {
            topic: msg.topic.to_string(),
            src_role: msg.src_role.clone(),
        };
        if let Some(v) = self.msgs.get_mut(&k) {
            v.push(msg);
            return Ok(());
        }
        self.msgs.insert(k, vec![msg]);
        Ok(())
    }

    fn vars(&self) -> &HashMap<String, Value> {
        &self.vars
    }

    fn seq(&self) -> i32 {
        self.seq
    }

    fn workspace(&self) -> &str {
        &self.workspace
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct DurableCheckpoint {
    seq: i32,
    vars: HashMap<String, Value>,
}

pub struct DurableStorage {
    checkpoint_path: String,
    checkpoint: DurableCheckpoint,
    topics_path: String,
    topics: HashMap<String, spoolq::Queue<Message>>,
    workspace: String,
}

impl DurableStorage {
    pub fn new(path: &str) -> Result<DurableStorage> {
        let checkpoint_path = std::path::PathBuf::from(path)
            .join("checkpoint.json")
            .to_str()
            .unwrap()
            .to_string();
        let topics_path = std::path::PathBuf::from(path)
            .join("topics")
            .to_str()
            .unwrap()
            .to_string();
        std::fs::DirBuilder::new().recursive(true)
            .mode(0o700)
            .create(&topics_path)
            .map_err(error::Error::IO)?;
        let workspace = std::path::PathBuf::from(path)
            .join("workspace")
            .to_str()
            .unwrap()
            .to_string();
        std::fs::DirBuilder::new().recursive(true)
            .mode(0o700)
            .create(&workspace)
            .map_err(error::Error::IO)?;
        DurableStorage::recover_all(&topics_path)?;
        Ok(DurableStorage {
            checkpoint: DurableCheckpoint {
                seq: 0,
                vars: HashMap::new(),
            },
            checkpoint_path: checkpoint_path,
            topics: HashMap::new(),
            topics_path: topics_path,
            workspace: workspace,
        })
    }

    fn recover_all(path: &str) -> Result<()> {
        let dirh = std::fs::read_dir(path)?;
        for maybe_dirent in dirh {
            match maybe_dirent {
                Ok(dirent) => {
                    if let Ok(ftype) = dirent.file_type() {
                        if ftype.is_dir() {
                            let topic_path = dirent.path().to_str().unwrap().to_string();
                            match DurableStorage::recover_topic(&topic_path) {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!("failed to recover topic queue {}: {}", &topic_path, e)
                                }
                            }
                        }
                    }
                }
                Err(e) => warn!("error recovering topic queues: {}", e),
            }
        }
        Ok(())
    }

    fn recover_topic(path: &str) -> Result<()> {
        let q = spoolq::Queue::<Message>::new(path)?;
        q.recover()?;
        Ok(())
    }

    fn new_queue(&self, topic: &str) -> Result<spoolq::Queue<Message>> {
        let q_path = std::path::PathBuf::from(&self.topics_path)
            .join(topic)
            .to_str()
            .unwrap()
            .to_string();
        spoolq::Queue::<Message>::new(&q_path).map_err(error::Error::IO)
    }
}

impl Storage for DurableStorage {
    fn load(&mut self) -> Result<(i32, HashMap<String, Value>)> {
        debug!("DurableStorage load: path={}", &self.checkpoint_path);
        if !std::path::Path::new(&self.checkpoint_path).exists() {
            self.checkpoint = DurableCheckpoint {
                seq: 0,
                vars: HashMap::new(),
            };
            debug!("DurableStorage.load: no checkpoint file! vars={:?}",
                   &self.checkpoint.vars);
            return Ok((self.checkpoint.seq, self.checkpoint.vars.clone()));
        }
        {
            let chk_file = std::fs::OpenOptions::new().read(true)
                .open(&self.checkpoint_path)?;
            self.checkpoint = serde_json::from_reader(chk_file).map_err(to_ioerror)
                .map_err(error::Error::IO)?;
            debug!("DurableStorage.load: loaded checkpoint: {:?}",
                   &self.checkpoint);
            Ok((self.checkpoint.seq, self.checkpoint.vars.clone()))
        }
    }

    fn save(&mut self, seq: i32, vars: HashMap<String, Value>) -> Result<()> {
        if seq < self.checkpoint.seq {
            return Err(error::Error::InvalidArgument("stale transaction".to_string()));
        }
        debug!("DurableStorage.save: saving vars before={:?} after={:?}",
               &self.checkpoint.vars,
               &vars);
        let chk = DurableCheckpoint {
            vars: vars,
            seq: seq + 1,
        };
        {
            let mut chk_file = std::fs::OpenOptions::new().write(true)
                .mode(0o600)
                .create(true)
                .truncate(true)
                .open(&self.checkpoint_path)?;
            serde_json::to_writer(&mut chk_file, &chk).map_err(to_ioerror)
                .map_err(error::Error::IO)?;
        }
        if !std::path::Path::new(&self.checkpoint_path).exists() {
            panic!("where is the checkpoint file? {}", &self.checkpoint_path);
        }
        for q in self.topics.values_mut() {
            q.flush()?;
        }
        self.checkpoint = chk;
        Ok(())
    }

    fn next_messages(&mut self,
                     filters: &HashSet<MessageFilter>)
                     -> Result<HashMap<String, Message>> {
        let mut next: HashMap<String, Message> = HashMap::new();
        for k in filters {
            if !self.topics.contains_key(&k.topic) {
                let q = self.new_queue(&k.topic)?;
                self.topics.insert(k.topic.to_string(), q);
            }
            let q = self.topics.get_mut(&k.topic).unwrap();
            let maybe_msg = q.pop().map_err(error::Error::IO)?;
            if let Some(msg) = maybe_msg {
                next.insert(k.topic.to_string(), msg);
            }
        }
        Ok(next)
    }

    fn push_msg(&mut self, msg: Message) -> Result<()> {
        if !self.topics.contains_key(&msg.topic) {
            let q = self.new_queue(&msg.topic)?;
            self.topics.insert(msg.topic.to_string(), q);
        }
        let q = self.topics.get_mut(&msg.topic).unwrap();
        q.push(msg).map_err(error::Error::IO)
    }

    fn vars(&self) -> &HashMap<String, Value> {
        &self.checkpoint.vars
    }

    fn seq(&self) -> i32 {
        self.checkpoint.seq
    }

    fn workspace(&self) -> &str {
        &self.workspace
    }
}

fn to_ioerror<E: std::error::Error>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.description())
}
