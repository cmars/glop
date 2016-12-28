pub struct Match {
    pub conditions: Vec<Box<Condition>>,
    pub actions: Vec<Box<Action>>,
}

pub enum Condition {
    Cmp(String, CmpOpcode, String),
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
