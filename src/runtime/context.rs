use std;
use std::collections::{HashMap, HashSet};
use std::process::Command;

use super::value::{Identifier, Message, Value};

pub struct Context {
    pub vars: HashMap<String, Value>,
    pub msgs: HashMap<String, Message>,
    pub popped_topics: HashSet<String>,
}

impl Context {
    pub fn new(vars: HashMap<String, Value>, msgs: HashMap<String, Message>) -> Context {
        Context {
            vars: vars,
            msgs: msgs,
            popped_topics: HashSet::new(),
        }
    }

    pub fn set_env(&self, cmd: &mut Command) {
        for (k, v) in Value::to_env(&self.vars) {
            cmd.env(k, v);
        }
        for (topic, msg) in &self.msgs {
            for (k, v) in Value::to_env(&msg.contents) {
                cmd.env(vec![topic.clone(), k.to_string()].join("__"), v);
            }
        }
        let path = std::env::var_os("PATH").unwrap_or(std::ffi::OsString::new());
        let exe_path = std::env::current_exe().unwrap();
        let exec_dir = std::path::Path::new(&exe_path).parent().unwrap();
        let mut new_path = std::env::split_paths(&path).collect::<Vec<_>>();
        new_path.insert(0, exec_dir.to_path_buf());
        cmd.env("PATH", std::env::join_paths(new_path).unwrap());
    }

    pub fn get_msg<'a>(&'a mut self, topic: &str, key: &Identifier) -> Option<&'a Value> {
        match self.msgs.get(topic) {
            Some(ref msg) => key.get(&msg.contents),
            None => None,
        }
    }

    pub fn get_var<'b>(&'b mut self, key: &Identifier) -> Option<&'b Value> {
        key.get(&mut self.vars)
    }

    pub fn set_var(&mut self, key: &Identifier, value: Value) {
        key.set(&mut self.vars, value)
    }

    pub fn unset_var(&mut self, key: &Identifier) {
        key.unset(&mut self.vars)
    }

    pub fn pop_msg(&mut self, topic: &str) {
        self.popped_topics.insert(topic.to_string());
    }
}
