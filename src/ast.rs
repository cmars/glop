extern crate serde;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Glop {
    #[serde(default)]
    pub summary: String,
    pub roles: HashMap<String, Role>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Role {
    #[serde(default)]
    pub summary: String,
    pub reactions: HashMap<String, Reaction>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Reaction {
    pub events: Vec<Event>,
    pub script: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    #[serde(rename = "message")]
    Message { topic: String, from: Option<String> },
    #[serde(rename = "elapsed")]
    Elapsed(String),
}
