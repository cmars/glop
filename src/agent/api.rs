use super::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Request {
    Add { contents: String, name: String },
    Remove { name: String },
    List,
    SendTo(Message),
    Introduce(Vec<AgentRole>),
    FetchReply { in_reply_to: String },
    FetchMsgs,
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
        src_agent: String,
        dst_agent: String,
    },
    Introduce(Vec<Response>),
    FetchReply(Option<Message>),
    FetchMsgs(Vec<Message>),
    Error(String),
}
