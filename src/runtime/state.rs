extern crate serde_json;
extern crate spoolq;

use std;
use std::collections::{HashMap, HashSet};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

use super::*;
use self::context::Context;
use self::transaction::Transaction;
use self::value::{Obj, Value};

pub trait Storage {
    fn load(&mut self) -> Result<(i32, HashMap<String, Value>)>;
    fn save(&mut self, seq: i32, vars: HashMap<String, Value>) -> Result<()>;

    fn next_messages(&mut self, topics: &HashSet<String>) -> Result<HashMap<String, Obj>>;
    fn push_msg(&mut self, topic: &str, msg: Obj) -> Result<()>;

    fn vars(&self) -> &HashMap<String, Value>;
    fn seq(&self) -> i32;
}

pub struct State<S: Storage> {
    storage: S,
}

impl<S: Storage> State<S> {
    pub fn new(storage: S) -> State<S> {
        State { storage: storage }
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
        let msgs = self.storage.next_messages(&m.msg_topics)?;
        let ctx = Context::new(vars, msgs);
        let txn = Transaction::new(m, seq, ctx);
        if txn.eval() {
            debug!("State.eval: MATCHED");
            Ok(Some(txn))
        } else {
            debug!("State.eval: MISSED");
            Ok(None)
        }
    }

    pub fn commit(&mut self, txn: &mut Transaction) -> Result<i32> {
        debug!("State.commit: BEGIN transaction seq={}", txn.seq);
        let mut txn = txn;
        let mut vars = self.storage.vars().clone();
        let mut popped = HashSet::new();
        let mut self_msgs = HashMap::new();
        let actions = txn.apply()?;
        for action in actions {
            debug!(target: "State.commit", "action {:?}", action);
            match &action {
                &Action::SetVar(ref k, ref v) => {
                    k.set(&mut vars, Value::Str(v.to_string()));
                }
                &Action::UnsetVar(ref k) => {
                    k.unset(&mut vars);
                }
                &Action::PopMsg(ref topic) => {
                    popped.insert(topic.to_string());
                }
                &Action::SendMsg { ref dst, ref topic, ref contents } => {
                    if dst != "self" {
                        return Err(Error::UndeliverableMessage(dst.to_string()));
                    }
                    self_msgs.insert(topic.to_string(), contents.clone());
                }
                _ => return Err(Error::UnsupportedAction),
            };
        }
        let msgs = txn.with_context(|ctx| ctx.msgs.clone());
        for (topic, msg) in msgs {
            if !popped.contains(&topic) {
                self.storage.push_msg(&topic, msg)?;
            }
        }
        for (topic, msg) in self_msgs {
            self.storage.push_msg(&topic, msg)?;
        }
        self.storage.save(txn.seq, vars)?;
        debug!("State.commit: OK transaction seq={}", txn.seq);
        Ok(txn.seq)
    }

    pub fn rollback(&mut self, txn: Transaction) -> Result<()> {
        let mut txn = txn;
        let msgs = txn.with_context(|ctx| ctx.msgs.clone());
        for (topic, msg) in msgs {
            self.storage.push_msg(&topic, msg)?;
        }
        Ok(())
    }
}

pub struct MemStorage {
    seq: i32,
    vars: HashMap<String, Value>,
    msgs: HashMap<String, Vec<Obj>>,
}

impl MemStorage {
    pub fn new() -> MemStorage {
        MemStorage {
            seq: 0,
            vars: HashMap::new(),
            msgs: HashMap::new(),
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

    fn next_messages(&mut self, topics: &HashSet<String>) -> Result<HashMap<String, Obj>> {
        let mut next: HashMap<String, Obj> = HashMap::new();
        for (k, v) in &mut self.msgs {
            if !topics.contains(k) {
                continue;
            }
            match v.pop() {
                Some(msg) => {
                    next.insert(k.to_string(), msg.clone());
                }
                None => (),
            }
        }
        Ok(next)
    }

    fn push_msg(&mut self, topic: &str, msg: Obj) -> Result<()> {
        match self.msgs.get_mut(topic) {
            Some(v) => {
                v.push(msg);
                return Ok(());
            }
            _ => {}
        }
        self.msgs.insert(topic.to_string(), vec![msg]);
        Ok(())
    }

    fn vars(&self) -> &HashMap<String, Value> {
        &self.vars
    }

    fn seq(&self) -> i32 {
        self.seq
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
    topics: HashMap<String, spoolq::Queue<Obj>>,
}

impl DurableStorage {
    pub fn new(path: &str) -> Result<DurableStorage> {
        let checkpoint_path =
            std::path::PathBuf::from(path).join("checkpoint.json").to_str().unwrap().to_string();
        let topics_path =
            std::path::PathBuf::from(path).join("topics").to_str().unwrap().to_string();
        std::fs::DirBuilder::new().recursive(true)
            .mode(0o700)
            .create(&topics_path)
            .map_err(error::Error::IO)?;
        Ok(DurableStorage {
            checkpoint: DurableCheckpoint {
                seq: 0,
                vars: HashMap::new(),
            },
            checkpoint_path: checkpoint_path,
            topics: HashMap::new(),
            topics_path: topics_path,
        })
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
            debug!("DurableStorage.load: no checkpoint file! vars={:?}", &self.checkpoint.vars);
            return Ok((self.checkpoint.seq, self.checkpoint.vars.clone()));
        }
        {
            let chk_file = std::fs::OpenOptions::new().read(true)
                .open(&self.checkpoint_path)?;
            self.checkpoint = serde_json::from_reader(chk_file).map_err(to_ioerror)
                .map_err(error::Error::IO)?;
            debug!("DurableStorage.load: loaded checkpoint: {:?}", &self.checkpoint);
            Ok((self.checkpoint.seq, self.checkpoint.vars.clone()))
        }
    }

    fn save(&mut self, seq: i32, vars: HashMap<String, Value>) -> Result<()> {
        if seq < self.checkpoint.seq {
            return Err(error::Error::InvalidArgument("stale transaction".to_string()));
        }
        debug!("DurableStorage.save: saving vars before={:?} after={:?}", &self.checkpoint.vars, &vars);
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
        self.checkpoint = chk;
        Ok(())
    }

    fn next_messages(&mut self, topics: &HashSet<String>) -> Result<HashMap<String, Obj>> {
        let mut next: HashMap<String, Obj> = HashMap::new();
        for k in topics {
            let q = match self.topics.get_mut(k) {
                Some(q) => q,
                None => continue,
            };
            let maybe_msg = q.pop().map_err(error::Error::IO)?;
            match maybe_msg {
                Some(msg) => {
                    next.insert(k.to_string(), msg);
                }
                None => {}
            };
        }
        Ok(next)
    }

    fn push_msg(&mut self, topic: &str, msg: Obj) -> Result<()> {
        if !self.topics.contains_key(topic) {
            let q_path = std::path::PathBuf::from(&self.topics_path)
                .join(topic)
                .to_str()
                .unwrap()
                .to_string();
            let q = spoolq::Queue::<Obj>::new(&q_path)?;
            self.topics.insert(topic.to_string(), q);
        }
        let q = self.topics.get_mut(topic).unwrap();
        q.push(msg).map_err(error::Error::IO)
    }

    fn vars(&self) -> &HashMap<String, Value> {
        &self.checkpoint.vars
    }

    fn seq(&self) -> i32 {
        self.checkpoint.seq
    }
}

fn to_ioerror<E: std::error::Error>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.description())
}
