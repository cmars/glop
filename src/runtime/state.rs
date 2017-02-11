use std::collections::{HashMap, HashSet};

use super::*;
use self::context::Context;
use self::transaction::Transaction;
use self::value::{Identifier, Obj, Value};

pub trait State {
    fn push_msg(&mut self, topic: &str, msg: Obj);
    fn next_messages(&self, topics: &HashSet<String>) -> HashMap<String, Obj>;
    fn eval(&mut self, m: &Match) -> Option<Transaction>;
    fn commit(&mut self, actions: &Vec<Action>) -> Result<i32>;
    fn apply_action(&mut self, action: &Action) -> Result<()>;
    fn get_var<'b>(&'b mut self, key: &Identifier) -> Option<&'b Value>;
    fn set_var(&mut self, key: &Identifier, value: Value);
    fn unset_var(&mut self, key: &Identifier);
    fn ack_msg(&mut self, topic: &str) -> Result<()>;
}

pub struct MemState {
    seq: i32,
    vars: HashMap<String, Value>,
    inbox: HashMap<String, Vec<Obj>>,
}

impl MemState {
    pub fn new() -> MemState {
        MemState {
            seq: 0,
            vars: HashMap::new(),
            inbox: HashMap::new(),
        }
    }
}

impl State for MemState {
    fn push_msg(&mut self, topic: &str, msg: Obj) {
        match self.inbox.get_mut(topic) {
            Some(v) => {
                v.push(msg);
                return;
            }
            _ => {}
        }
        self.inbox.insert(topic.to_string(), vec![msg]);
    }

    fn next_messages(&self, topics: &HashSet<String>) -> HashMap<String, Obj> {
        let mut next: HashMap<String, Obj> = HashMap::new();
        for (k, v) in &self.inbox {
            if !topics.contains(k) {
                continue;
            }
            match v.last() {
                Some(msg) => {
                    next.insert(k.to_string(), msg.clone());
                }
                None => (),
            }
        }
        next
    }

    fn eval(&mut self, m: &Match) -> Option<Transaction> {
        let ctx = Context {
            vars: self.vars.clone(),
            msgs: self.next_messages(&m.msg_topics),
        };
        let txn = Transaction::new(self.seq, ctx);
        let is_match = m.conditions
            .iter()
            .fold(true, |acc, c| acc && txn.eval(c));
        if !is_match {
            return None;
        }
        Some(txn)
    }

    fn commit(&mut self, actions: &Vec<Action>) -> Result<i32> {
        for action in actions {
            self.apply_action(action)?;
        }
        let result = self.seq;
        self.seq += 1;
        Ok(result)
    }

    fn apply_action(&mut self, action: &Action) -> Result<()> {
        match action {
            &Action::SetVar(ref k, ref v) => {
                self.set_var(k, Value::Str(v.to_string()));
                Ok(())
            }
            &Action::UnsetVar(ref k) => {
                self.unset_var(k);
                Ok(())
            }
            &Action::Acknowledge(ref k) => self.ack_msg(k),
            _ => Err(Error::UnsupportedAction),
        }
    }

    fn get_var<'b>(&'b mut self, key: &Identifier) -> Option<&'b Value> {
        key.get(&mut self.vars)
    }

    fn set_var(&mut self, key: &Identifier, value: Value) {
        key.set(&mut self.vars, value)
    }

    fn unset_var(&mut self, key: &Identifier) {
        key.unset(&mut self.vars)
    }

    fn ack_msg(&mut self, topic: &str) -> Result<()> {
        match self.inbox.get_mut(topic) {
            Some(v) => {
                v.pop();
                Ok(())
            }
            None => Err(Error::Acknowledge(topic.to_string())),
        }
    }
}
