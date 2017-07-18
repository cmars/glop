use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum State {
    SingularState {
        name: String,
        actions: Vec<Action>,
        fault: Vec<Action>,
    },
    NestedState {
        name: String,
        states: HashMap<String, State>,
        fault: Vec<Action>,
    },
    SplitState {
        name: String,
        splits: Vec<String>,
        fault: Vec<Action>,
    },
}

impl State {
    pub fn set_fault(&mut self, mut fault: Vec<Action>) {
        match self {
            &mut State::SingularState { name: _, actions: _, fault: ref mut fault_ } => {
                fault_.append(&mut fault);
            }
            &mut State::NestedState { name: _, states: _, fault: ref mut fault_ } => {
                fault_.append(&mut fault);
            }
            &mut State::SplitState { name: _, splits: _, fault: ref mut fault_ } => {
                fault_.append(&mut fault);
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            &State::SingularState { name: ref name_, actions: _, fault: _ } => name_,
            &State::NestedState { name: ref name_, states: _, fault: _ } => name_,
            &State::SplitState { name: ref name_, splits: _, fault: _ } => name_,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Goto(Identifier),
    Assert(Expression),
    Log(Expression),
    Conditional {
        conditions: Vec<When>,
        otherwise: Vec<Action>,
    },
    Shutdown,
    Spawn(Expression),
    Await(Vec<EventHandler>),
    Send {
        dst: Expression,
        role: Option<Expression>,
        topic: Expression,
        contents: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct When {
    pub expr: Expression,
    pub actions: Vec<Action>,
}

#[derive(Debug, PartialEq)]
pub enum EventHandler {
    Message {
        identifier: Option<Identifier>,
        topic: Expression,
        role: Option<Expression>,
        actions: Vec<Action>,
    },
    Elapsed {
        duration: Duration,
        actions: Vec<Action>,
    },
}

impl EventHandler {
    pub fn set_actions(&mut self, mut actions: Vec<Action>) {
        match self {
            &mut EventHandler::Message { actions: ref mut actions_,
                                         identifier: _,
                                         topic: _,
                                         role: _ } => {
                actions_.append(&mut actions);
            }
            &mut EventHandler::Elapsed { actions: ref mut actions_, duration: _ } => {
                actions_.append(&mut actions);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(i32),
    String(String),
    Bool(bool),
    Identifier(Identifier),
    UnOp(UnOp, Box<Expression>),
    BinOp(Box<Expression>, BinOp, Box<Expression>),
    Nil,
}

pub type Identifier = String;

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Equal,
    NotEqual,
    Gt,
    GtEqual,
    Lt,
    LtEqual,
}
