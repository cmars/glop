pub struct Match {
    conditions: Vec<Box<Condition>>,
    actions: Vec<Box<Action>>,
}

pub enum Condition {
    Cmp(Box<Expr>, CmpOpcode, Box<Expr>),
    IsSet(String),
    Message(String),
}

pub enum CmpOpcode {
    Equal,
    NotEqual,
}

pub enum Action {
    SetVar(String, String),
    UnsetVar(String),
    Acknowledge,
    Shell(String),
    Script(String),
}
