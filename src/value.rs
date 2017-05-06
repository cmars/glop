extern crate textnonce;

use std::collections::HashMap;

use super::ast;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Str(String),
    Object(Obj),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            &Value::Int(x) => {
                match other {
                    &Value::Int(y) => x == y,
                    _ => false,
                }
            }
            &Value::Str(ref x) => {
                match other {
                    &Value::Str(ref y) => x == y,
                    _ => false,
                }
            }
            &Value::Object(ref x) => {
                match other {
                    &Value::Object(ref y) => x.eq(y),
                    _ => false,
                }
            }
        }
    }
}

impl Value {
    pub fn from_int(i: i32) -> Value {
        Value::Int(i)
    }

    pub fn from_str(s: &str) -> Value {
        Value::Str(s.to_string())
    }

    pub fn from_obj(o: Obj) -> Value {
        Value::Object(o)
    }

    pub fn from_flat_map(m: HashMap<String, String>) -> Obj {
        let mut result = Obj::new();
        for (k, v) in m.iter() {
            let id = Identifier::from_str(&k);
            id.set(&mut result, Value::from_str(v));
        }
        result
    }

    pub fn to_string(&self) -> String {
        match self {
            &Value::Int(ref i) => i.to_string(),
            &Value::Str(ref s) => s.to_string(),
            &Value::Object(_) => "{...}".to_string(), // FIXME
        }
    }

    pub fn to_env(o: &Obj) -> Env {
        Value::to_env_prefix(o, "").into_iter().collect()
    }

    fn to_env_prefix(o: &Obj, prefix: &str) -> Vec<(String, String)> {
        o.iter()
            .map(|(k, v)| {
                let fqprefix = match prefix {
                    "" => k.to_string(),
                    _ => vec![prefix, k].join("__").to_string(),
                };
                match v {
                    &Value::Object(ref child) => Value::to_env_prefix(child, &fqprefix),
                    _ => vec![(fqprefix.clone(), v.to_string())],
                }
            })
            .flat_map(|v| v.into_iter())
            .collect()
    }
}

/// A free-form structured object.
pub type Obj = HashMap<String, Value>;

/// Messages are the basis of communication among agents.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Message {
    /// Unique ID assigned to each message.
    pub id: String,

    /// Source remote ID. None if local source.
    pub src_remote: Option<String>,

    /// Source agent name.
    pub src_agent: String,

    /// Source role name, if the message is addressed as coming from an agent acting in a role.
    pub src_role: Option<String>,

    /// Destination remote ID. None if local destination.
    pub dst_remote: Option<String>,

    /// Destination agent name.
    pub dst_agent: String,

    /// ID of message being replied to.
    pub in_reply_to: Option<String>,

    /// Topic of the message.
    pub topic: String,

    /// Contents of the message.
    pub contents: Obj,
}

impl Message {
    pub fn new(topic: &str, contents: Obj) -> Message {
        Message {
            id: textnonce::TextNonce::sized_urlsafe(32)
                .unwrap()
                .into_string(),
            src_remote: None,
            src_agent: "".to_string(),
            src_role: None,
            dst_remote: None,
            dst_agent: "".to_string(),
            topic: topic.to_string(),
            in_reply_to: None,
            contents: contents,
        }
    }

    pub fn new_id(mut self) -> Message {
        self.id = textnonce::TextNonce::sized_urlsafe(32)
            .unwrap()
            .into_string();
        self
    }

    pub fn src_remote(mut self, src_remote: &str) -> Message {
        self.src_remote = Some(src_remote.to_string());
        self
    }

    pub fn src_agent(mut self, src_agent: &str) -> Message {
        self.src_agent = src_agent.to_string();
        self
    }

    pub fn src_role(mut self, src_role: Option<String>) -> Message {
        self.src_role = src_role.clone();
        self
    }

    pub fn dst_remote(mut self, dst_remote: &str) -> Message {
        self.dst_remote = Some(dst_remote.to_string());
        self
    }

    pub fn dst_agent(mut self, dst_agent: &str) -> Message {
        self.dst_agent = dst_agent.to_string();
        self
    }

    pub fn in_reply_to(mut self, in_reply_to: Option<String>) -> Message {
        self.in_reply_to = in_reply_to.clone();
        self
    }
}

/// Environment variable settings.
pub type Env = HashMap<String, String>;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Identifier(Vec<String>);

impl Identifier {
    pub fn from_ast(i_ast: &ast::Identifier) -> Identifier {
        Identifier(i_ast.clone())
    }

    pub fn from_str(s: &str) -> Identifier {
        let v: Vec<String> = s.split(".").map(|x| x.to_string()).collect();
        Identifier(v)
    }

    pub fn get<'b>(&self, root: &'b Obj) -> Option<&'b Value> {
        if self.0.is_empty() {
            return None;
        }
        let mut cur = root;
        for i in 0..self.0.len() {
            match cur.get(&self.0[i]) {
                Some(v) => {
                    match v {
                        &Value::Object(ref o) => {
                            cur = o;
                        }
                        _ => {}
                    }
                    if i == self.0.len() - 1 {
                        return Some(v);
                    }
                }
                None => {
                    return None;
                }
            }
        }
        return None;
    }

    pub fn is_set(&self, root: &Obj) -> bool {
        match self.get(root) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn set(&self, root: &mut Obj, value: Value) {
        Identifier::set_slice(&self.0, root, value);
    }

    fn set_slice(path: &[String], o: &mut Obj, value: Value) {
        if path.is_empty() {
            return;
        }
        let (next, rest) = path.split_first().unwrap();
        if rest.is_empty() {
            o.insert(next.to_string(), value);
            return;
        }
        match o.get_mut(next) {
            Some(v) => {
                match v {
                    &mut Value::Object(ref mut child) => {
                        Identifier::set_slice(rest, child, value);
                        return;
                    }
                    _ => {}
                }
            }
            None => {}
        }
        o.insert(next.to_string(), Value::Object(Obj::new()));
        Identifier::set_slice(path, o, value);
    }

    pub fn unset(&self, root: &mut Obj) {
        Identifier::unset_slice(&self.0, root)
    }

    fn unset_slice(path: &[String], o: &mut Obj) {
        if path.is_empty() {
            return;
        }
        let (next, rest) = path.split_first().unwrap();
        if rest.is_empty() {
            o.remove(next);
            return;
        }
        match o.get_mut(next) {
            Some(v) => {
                match v {
                    &mut Value::Object(ref mut child) => {
                        Identifier::unset_slice(rest, child);
                        return;
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }
}
