use std::collections::{HashMap, HashSet};

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
    fn mut_vars(&mut self) -> &mut HashMap<String, Value>;
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

    fn mut_vars(&mut self) -> &mut HashMap<String, Value> {
        &mut self.vars
    }
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
        debug!(target: "State.eval", "{:?}", &m);
        let (seq, vars) = self.storage.load()?;
        let msgs = self.storage.next_messages(&m.msg_topics)?;
        let ctx = Context {
            vars: vars,
            msgs: msgs,
        };
        let txn = Transaction::new(m, seq, ctx);
        if txn.eval() {
            debug!(target: "State.eval", "MATCHED");
            Ok(Some(txn))
        } else {
            debug!(target:"State.eval", "MISSED");
            Ok(None)
        }
    }

    pub fn commit(&mut self, txn: Transaction) -> Result<i32> {
        debug!(target: "State.commit", "BEGIN transaction seq={}", txn.seq);
        let mut txn = txn;
        let mut vars = self.storage.vars().clone();
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
                _ => return Err(Error::UnsupportedAction),
            };
        }
        self.storage.save(txn.seq, vars)?;
        debug!(target: "State.commit", "OK transaction seq={}", txn.seq);
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
