use super::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Request {
    Add { source: String, name: String },
    Remove { name: String },
    List,
    SendTo(Message),
    Introduce(Vec<AgentRole>),
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
    SendTo { src: String, dst: String },
    Introduce(Vec<Response>),
    Error(String),
}
