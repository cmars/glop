use std::collections::HashSet;

use super::*;
use value::{Value, Identifier};

#[derive(Clone, Debug)]
pub struct Match {
    pub conditions: Vec<Condition>,
    pub msg_topics: HashSet<String>,
    pub actions: Vec<Action>,
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

#[derive(Clone, Debug)]
pub enum Condition {
    Cmp(Identifier, CmpOpcode, String),
    IsSet(Identifier),
    IsUnset(Identifier),
    Message(String),
}

impl Condition {
    fn new(c_ast: &ast::Condition) -> Condition {
        match c_ast {
            &ast::Condition::Cmp(ref l, ref op, ref r) => {
                Condition::Cmp(Identifier::from_ast(l), CmpOpcode::new(op), r.to_string())
            }
            &ast::Condition::IsSet(ref k) => Condition::IsSet(Identifier::from_ast(k)),
            &ast::Condition::IsUnset(ref k) => Condition::IsUnset(Identifier::from_ast(k)),
            &ast::Condition::Message(ref k) => Condition::Message(k.to_string()),
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
    Script(String),
}

impl Action {
    fn new(a_ast: &ast::Action) -> Action {
        match a_ast {
            &ast::Action::SetVar(ref k, ref v) => {
                Action::SetVar(Identifier::from_ast(k), v.to_string())
            }
            &ast::Action::UnsetVar(ref k) => Action::UnsetVar(Identifier::from_ast(k)),
            &ast::Action::Script(ref contents) => Action::Script(contents.to_string()),
        }
    }
}
