use std::collections::HashSet;

use super::*;
use value::{Identifier, Obj, Value};

#[derive(Clone, Debug)]
pub struct Match {
    pub conditions: Vec<Condition>,
    pub msg_topics: HashSet<String>,
    pub actions: Vec<Action>,
    pub acting_roles: HashSet<String>,
}

impl Match {
    fn new() -> Match {
        Match {
            conditions: vec![],
            msg_topics: HashSet::new(),
            actions: vec![],
            acting_roles: HashSet::new(),
        }
    }

    pub fn new_from_ast(m_ast: &ast::Match) -> Match {
        let mut m_exc = Match::new();
        m_exc.conditions = m_ast.conditions
            .iter()
            .map(|c_ast| {
                if let &ast::Condition::Message { ref topic, peer_role: _, acting_role: _ } =
                    c_ast {
                    m_exc.msg_topics.insert(topic.to_string());
                }
                Condition::new(c_ast)
            })
            .collect();
        m_exc.actions = m_ast.actions.iter().map(|a_ast| Action::new(a_ast)).collect();
        m_exc.acting_roles = m_ast.acting_roles.clone();
        m_exc
    }
}

#[derive(Clone, Debug)]
pub enum Condition {
    Cmp(Identifier, CmpOpcode, String),
    IsSet(Identifier),
    IsUnset(Identifier),
    Message {
        topic: String,
        peer_role: Option<String>,
        acting_role: Option<String>,
    },
}

impl Condition {
    fn new(c_ast: &ast::Condition) -> Condition {
        match c_ast {
            &ast::Condition::Cmp(ref l, ref op, ref r) => {
                Condition::Cmp(Identifier::from_ast(l), CmpOpcode::new(op), r.to_string())
            }
            &ast::Condition::IsSet(ref k) => Condition::IsSet(Identifier::from_ast(k)),
            &ast::Condition::IsUnset(ref k) => Condition::IsUnset(Identifier::from_ast(k)),
            &ast::Condition::Message { ref topic, ref peer_role, ref acting_role } => {
                Condition::Message {
                    topic: topic.to_string(),
                    peer_role: peer_role.clone(),
                    acting_role: acting_role.clone(),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
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

    pub fn eval(&self, l: &Value, r: &str) -> bool {
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

#[derive(Clone, Debug)]
pub enum Action {
    SetVar(Identifier, String),
    UnsetVar(Identifier),
    PopMsg(String),
    Script(String),
    SendMsg {
        dst: String,
        topic: String,
        contents: Obj,
    },
}

impl Action {
    fn new(a_ast: &ast::Action) -> Action {
        match a_ast {
            &ast::Action::SetVar(ref k, ref v) => {
                Action::SetVar(Identifier::from_ast(k), v.to_string())
            }
            &ast::Action::UnsetVar(ref k) => Action::UnsetVar(Identifier::from_ast(k)),
            &ast::Action::PopMsg(ref topic) => Action::PopMsg(topic.to_string()),
            &ast::Action::Script(ref contents) => Action::Script(contents.to_string()),
        }
    }
}
