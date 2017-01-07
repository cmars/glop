use std::collections::{HashMap, HashSet};
use std::env;
use std::error;
use std::fmt;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::process::Command;

extern crate textnonce;

use super::ast;
use super::cleanup::Cleanup;
use super::value::{Value, Identifier, Obj};

pub type Msg = Obj;

pub struct Match {
    conditions: Vec<Condition>,
    msg_topics: HashSet<String>,
    actions: Vec<Action>,
}

impl Match {
    fn new() -> Match {
        Match {
            conditions: vec![],
            msg_topics: HashSet::new(),
            actions: vec![],
        }
    }

    pub fn new_from_ast(m_ast: &ast::Match) -> Match {
        let mut m_exc = Match::new();
        for c_ast in &m_ast.conditions {
            m_exc.conditions.push(Condition::new(c_ast));
            match c_ast {
                &ast::Condition::Message(ref topic) => {
                    m_exc.msg_topics.insert(topic.to_string());
                }
                _ => (),
            }
        }
        for a_ast in &m_ast.actions {
            m_exc.actions.push(Action::new(a_ast));
        }
        m_exc
    }
}

type StatefulResult = Result<(), StatefulError>;

#[derive(Debug)]
pub enum StatefulError {
    ScriptFile(io::Error),
    ScriptExec(i32, String),
    Acknowledge(String),
}

impl From<io::Error> for StatefulError {
    fn from(err: io::Error) -> StatefulError {
        StatefulError::ScriptFile(err)
    }
}

impl fmt::Display for StatefulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatefulError::ScriptFile(ref err) => write!(f, "Script file error: {}", err),
            StatefulError::ScriptExec(code, ref stderr) => {
                write!(f, "Script exit code {}: {}", code, stderr)
            }
            StatefulError::Acknowledge(ref topic) => write!(f, "Invalid acknowledge: {}", topic),
        }
    }
}

impl error::Error for StatefulError {
    fn description(&self) -> &str {
        match *self {
            StatefulError::ScriptFile(ref err) => err.description(),
            StatefulError::ScriptExec(_, ref stderr) => stderr,
            StatefulError::Acknowledge(ref topic) => topic,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            StatefulError::ScriptFile(ref err) => Some(err),
            _ => None,
        }
    }
}

pub trait Stateful {
    fn set_var(&mut self, key: &Identifier, value: Value);

    fn unset_var(&mut self, key: &Identifier);

    fn ack_msg(&mut self, topic: &str) -> StatefulResult;
}

pub struct Context<'a, 'b> {
    pub seq: i32,
    pub vars: HashMap<String, Value>,
    pub msgs: HashMap<String, Msg>,
    pub applied: Vec<&'a Action>,
    pub st: &'b mut State,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn apply(&mut self, m: &'a Match) -> StatefulResult {
        for action in &m.actions {
            try!(self.apply_action(&action));
        }
        self.commit()
    }

    fn commit(&mut self) -> StatefulResult {
        for action in &self.applied {
            try!(self.st.apply_action(action));
        }
        self.st.next_seq();
        Ok(())
    }

    fn eval(&self, cond: &Condition) -> bool {
        match cond {
            &Condition::Cmp(ref l, ref op, ref r) => {
                match l.get(&self.vars) {
                    Some(v) => op.eval(v, r),
                    None => false,
                }
            }
            &Condition::IsSet(ref k) => k.is_set(&self.vars),
            &Condition::Message(ref k) => self.msgs.contains_key(k),
        }
    }

    fn apply_action(&mut self, action: &'a Action) -> StatefulResult {
        let result = match action {
            &Action::SetVar(ref k, ref v) => {
                self.set_var(k, Value::Str(v.to_string()));
                Ok(())
            }
            &Action::UnsetVar(ref k) => {
                self.unset_var(k);
                Ok(())
            }
            &Action::Acknowledge(ref k) => self.ack_msg(k),
            &Action::Script(ref contents) => self.exec_script(contents),
            // TODO: exec
        };
        match result {
            Ok(_) => self.applied.push(action),
            _ => {}
        }
        result
    }

    fn exec_script(&mut self, contents: &str) -> StatefulResult {
        let mut script_path_buf = env::temp_dir();
        script_path_buf.push(textnonce::TextNonce::sized_urlsafe(32).unwrap().into_string());
        let script_path = script_path_buf.to_str().unwrap();
        let cleanup = Cleanup::File(script_path.to_string());
        {
            let mut script_file =
                OpenOptions::new().write(true).mode(0o700).create_new(true).open(script_path)?;
            script_file.write_all(contents.as_bytes()).map_err(StatefulError::ScriptFile)?;
        }
        let output = Command::new(script_path).output().map_err(StatefulError::ScriptFile)?;
        drop(cleanup);
        if output.status.success() {
            Ok(())
        } else {
            let code = match output.status.code() {
                Some(value) => value,
                None => 0,
            };
            let stderr = match String::from_utf8(output.stderr) {
                Ok(s) => s,
                Err(_) => "(stderr was invalid utf8)".to_string(),
            };
            Err(StatefulError::ScriptExec(code, stderr))
        }
    }
}

impl<'a, 'b> Stateful for Context<'a, 'b> {
    fn set_var(&mut self, key: &Identifier, value: Value) {
        key.set(&mut self.vars, value)
    }

    fn unset_var(&mut self, key: &Identifier) {
        key.unset(&mut self.vars)
    }

    fn ack_msg(&mut self, topic: &str) -> StatefulResult {
        match self.msgs.remove(topic) {
            Some(_) => Ok(()),
            None => Err(StatefulError::Acknowledge(topic.to_string())),
        }
    }
}

pub enum Condition {
    Cmp(Identifier, CmpOpcode, String),
    IsSet(Identifier),
    Message(String),
}

impl Condition {
    fn new(c_ast: &ast::Condition) -> Condition {
        match c_ast {
            &ast::Condition::Cmp(ref l, ref op, ref r) => {
                Condition::Cmp(Identifier::from_ast(l), CmpOpcode::new(op), r.to_string())
            }
            &ast::Condition::IsSet(ref k) => Condition::IsSet(Identifier::from_ast(k)),
            &ast::Condition::Message(ref k) => Condition::Message(k.to_string()),
        }
    }
}

pub enum CmpOpcode {
    Equal,
    NotEqual,
}

impl CmpOpcode {
    fn new(c_ast: &ast::CmpOpcode) -> CmpOpcode {
        match c_ast {
            &ast::CmpOpcode::Equal => CmpOpcode::Equal,
            &ast::CmpOpcode::NotEqual => CmpOpcode::NotEqual,
        }
    }

    fn eval(&self, l: &Value, r: &str) -> bool {
        let v = match l {
            &Value::Int(i) => i.to_string(),
            &Value::Str(ref s) => s.to_string(),
            _ => {
                return false;
            } 
        };
        match self {
            &CmpOpcode::Equal => v == r,
            &CmpOpcode::NotEqual => v != r,
        }
    }
}

pub enum Action {
    SetVar(Identifier, String),
    UnsetVar(Identifier),
    Acknowledge(String),
    Script(String),
}

impl Action {
    fn new(a_ast: &ast::Action) -> Action {
        match a_ast {
            &ast::Action::SetVar(ref k, ref v) => {
                Action::SetVar(Identifier::from_ast(k), v.to_string())
            }
            &ast::Action::UnsetVar(ref k) => Action::UnsetVar(Identifier::from_ast(k)),
            &ast::Action::Acknowledge(ref topic) => Action::Acknowledge(topic.to_string()),
            &ast::Action::Script(ref contents) => Action::Script(contents.to_string()),
            _ => panic!(format!("action {} not implemented yet", a_ast)),
        }
    }
}

pub struct State {
    seq: i32,
    vars: HashMap<String, Value>,
    pending_msgs: HashMap<String, Vec<Msg>>,
}

impl State {
    pub fn new() -> State {
        State {
            seq: 0,
            vars: HashMap::new(),
            pending_msgs: HashMap::new(),
        }
    }

    pub fn push_msg(&mut self, topic: &str, msg: Msg) {
        match self.pending_msgs.get_mut(topic) {
            Some(v) => {
                v.push(msg);
                return;
            }
            _ => {}
        }
        self.pending_msgs.insert(topic.to_string(), vec![msg]);
    }

    fn next_seq(&mut self) -> i32 {
        self.seq += 1;
        self.seq
    }

    fn next_messages(&self, topics: &HashSet<String>) -> HashMap<String, Msg> {
        let mut next: HashMap<String, Msg> = HashMap::new();
        for (k, v) in &self.pending_msgs {
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

    pub fn eval(&mut self, m: &Match) -> Option<Context> {
        let ctx = Context {
            seq: self.seq,
            vars: self.vars.clone(),
            msgs: self.next_messages(&m.msg_topics),
            applied: vec![],
            st: self,
        };
        let is_match = m.conditions
            .iter()
            .fold(true, |acc, c| acc && ctx.eval(c));
        if !is_match {
            return None;
        }
        Some(ctx)
    }

    fn apply_action(&mut self, action: &Action) -> StatefulResult {
        let result = match action {
            &Action::SetVar(ref k, ref v) => {
                self.set_var(k, Value::Str(v.to_string()));
                Ok(())
            }
            &Action::UnsetVar(ref k) => {
                self.unset_var(k);
                Ok(())
            }
            &Action::Acknowledge(ref k) => self.ack_msg(k),
            &Action::Script(_) => Ok(()),
            // TODO: exec
        };
        result
    }
}

impl Stateful for State {
    fn set_var(&mut self, key: &Identifier, value: Value) {
        key.set(&mut self.vars, value)
    }

    fn unset_var(&mut self, key: &Identifier) {
        key.unset(&mut self.vars)
    }

    fn ack_msg(&mut self, topic: &str) -> StatefulResult {
        match self.pending_msgs.get_mut(topic) {
            Some(v) => {
                v.pop();
                Ok(())
            }
            None => Err(StatefulError::Acknowledge(topic.to_string())),
        }
    }
}
