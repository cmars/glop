use super::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Request {
    Add { contents: String, name: String },
    Remove { name: String },
    List,
    SendTo(Message),
    Introduce(Vec<AgentRole>),
}

#[derive(Debug)]
pub struct Authenticated<T> {
    pub auth_id: String,
    pub item: T,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentRole {
    pub name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Response {
    Add,
    Remove,
    List { names: Vec<String> },
    SendTo {
        id: String,
        src: String,
        dst: String,
    },
    Introduce(Vec<Response>),
    Error(String),
}
