use std::collections::{HashMap, HashSet};

use super::ast;

pub type Msg = HashMap<String, String>;

pub struct Match {
    conditions: Vec<Condition>,
    msg_topics: HashSet<String>, // actions Vec<Action>,
}

impl Match {
    fn new() -> Match {
        Match {
            conditions: vec![],
            msg_topics: HashSet::new(),
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
        // for a_ast in m_ast.actions {
        // m_exc.actions.push(Action::new(a_ast));
        // TODO: plumbing for exec & script actions
        //
        m_exc
    }

    pub fn eval(&self, st: &mut State) -> Option<Context> {
        let ctx = Context {
            seq: -1,
            vars: st.vars.clone(),
            msgs: st.next_messages(&self.msg_topics),
        };
        let is_match = self.conditions
            .iter()
            .fold(true, |acc, c| acc && ctx.eval(c));
        if !is_match {
            return None;
        }
        let mut ctx = ctx;
        ctx.seq = st.next_seq();
        Some(ctx)
    }
}

pub struct Context {
    pub seq: i32,
    pub vars: HashMap<String, String>,
    pub msgs: HashMap<String, Msg>,
}

impl Context {
    fn eval(&self, cond: &Condition) -> bool {
        match cond {
            &Condition::Cmp(ref l, ref op, ref r) => {
                match self.vars.get(l) {
                    Some(v) => op.eval(v, r),
                    None => false,
                }
            }
            &Condition::IsSet(ref k) => self.vars.contains_key(k),
            &Condition::Message(ref k) => self.msgs.contains_key(k),
        }
    }
}

pub enum Condition {
    Cmp(String, CmpOpcode, String),
    IsSet(String),
    Message(String),
}

impl Condition {
    fn new(c_ast: &ast::Condition) -> Condition {
        match c_ast {
            &ast::Condition::Cmp(ref l, ref op, ref r) => {
                Condition::Cmp(l.to_string(), CmpOpcode::new(op), r.to_string())
            }
            &ast::Condition::IsSet(ref k) => Condition::IsSet(k.to_string()),
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

    fn eval(&self, l: &str, r: &str) -> bool {
        match self {
            &CmpOpcode::Equal => l == r,
            &CmpOpcode::NotEqual => l != r,
        }
    }
}

// enum Action {
// SetVar(String, String),
// UnsetVar(String),
// Acknowledge(String),
// Exec(String),
//

// type History = Vec<Context>;

pub struct State {
    seq: i32,
    vars: HashMap<String, String>,
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

    pub fn set_var(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
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

    // fn apply(&mut self, action Action) {
    // match op {
    // SetVar(k, v) => &self.vars.insert(k, v),
    // UnsetVar(k) => &self.vars.remove(k),
    // Acknowledge(k) => match self.pending_msgs.get_mut(k) {
    // Some(msgs) => msgs.pop(),
    // None => (),
    // }
    // }
    // }
    //
}

// struct Executive {
// state State;
// history History;
// step Step;
// }
//
// impl Executive {
// fn begin_step(&mut self, match_stanza ast::Match) -> &Step {
// let vars = &self.state.vars.clone();
// let msgs = &self.state.next_messages();
// &self.step = Step{
// seq: &self.state.next_seq(),
//
// match_stanza: match_stanza,
//
// vars: &self.vars.clone(),
// msgs: &self.state.next_messages(&match_stanza),
// ops: &self.ops(&match_stanza),
// };
// &self.step
// }
//
// fn end_step(&mut self, &step Step) {
// for op in &step.ops.drain(..) {
// &step.state.apply(op);
// }
// &self.history.push(&self.step);
// }
// }
//
