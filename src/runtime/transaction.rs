extern crate textnonce;

use std;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};

use super::*;
use self::context::Context;
use self::value::Value;

pub struct Transaction {
    pub m: Match,
    pub seq: i32,
    pub ctx: Arc<Mutex<Context>>,
    pub applied: Vec<Action>,
    matched_topics: HashSet<String>,
}

impl Transaction {
    pub fn new(m: Match, seq: i32, ctx: Context) -> Transaction {
        Transaction {
            m: m,
            seq: seq,
            ctx: Arc::new(Mutex::new(ctx)),
            applied: vec![],
            matched_topics: HashSet::new(),
        }
    }

    pub fn apply(&mut self) -> Result<Vec<Action>> {
        self.matched_topics = self.m.topics();
        let mut actions = self.m.actions.clone();
        let mut applied: Vec<Action> = vec![];
        loop {
            if actions.is_empty() {
                return Ok(applied);
            }
            let action = actions.remove(0);
            let mut resulting_actions = match action {
                Action::SetVar(ref k, ref v) => {
                    let mut ctx = self.ctx.lock().unwrap();
                    ctx.set_var(k, Value::Str(v.to_string()));
                    vec![action.clone()]
                }
                Action::UnsetVar(ref k) => {
                    let mut ctx = self.ctx.lock().unwrap();
                    ctx.unset_var(k);
                    vec![action.clone()]
                }
                Action::Script(ref contents) => self.exec_script(contents)?,
                Action::SendMsg { dst_remote: _,
                                  dst_agent: _,
                                  topic: _,
                                  in_reply_to: _,
                                  contents: _ } => vec![action],
                Action::ReplyTo { ref topic, ref contents } => {
                    let ctx = self.ctx.lock().unwrap();
                    match ctx.msgs.get(topic) {
                        Some(ref subject) => {
                            vec![Action::SendMsg {
                                     dst_agent: subject.src_agent.to_string(),
                                     dst_remote: subject.src_remote.clone(),
                                     topic: topic.to_string(),
                                     in_reply_to: subject.in_reply_to.clone(),
                                     contents: contents.clone(),
                                 }]
                        }
                        None => return Err(Error::UndeliverableMessage(topic.to_string())),
                    }
                }
                Action::Match(ref m) => {
                    if self.eval_match(m) {
                        self.matched_topics.extend(m.topics());
                        actions.append(&mut m.actions.clone())
                    }
                    continue;
                }
            };
            applied.append(&mut resulting_actions);
        }
    }

    pub fn matched_topics(&self) -> HashSet<String> {
        self.matched_topics.clone()
    }

    pub fn eval(&self) -> bool {
        self.eval_match(&self.m)
    }

    fn eval_match(&self, m: &Match) -> bool {
        m.conditions
            .iter()
            .fold(true, |acc, c| acc && self.eval_condition(c))
    }

    fn eval_condition(&self, cond: &Condition) -> bool {
        let ctx = self.ctx.lock().unwrap();
        match cond {
            &Condition::Cmp(ref l, ref op, ref r) => {
                match l.get(&ctx.vars) {
                    Some(v) => op.eval(v, r),
                    None => false,
                }
            }
            &Condition::IsSet(ref k) => k.is_set(&ctx.vars),
            &Condition::IsUnset(ref k) => !k.is_set(&ctx.vars),
            &Condition::Message { ref topic, ref src_role, acting_role: _ } => {
                if let Some(msg) = ctx.msgs.get(topic) {
                    src_role.eq(&msg.src_role)
                } else {
                    false
                }
            }
        }
    }

    pub fn with_context<F, T>(&mut self, f: F) -> T
        where F: Fn(&mut Context) -> T
    {
        let mut ctx = self.ctx.lock().unwrap();
        f(&mut ctx)
    }

    fn exec_script(&mut self, contents: &str) -> Result<Vec<Action>> {
        let mut script_path_buf = std::env::temp_dir();
        let script_path_base = textnonce::TextNonce::sized_urlsafe(32)
            .unwrap()
            .into_string();
        script_path_buf.push(&script_path_base);
        let script_path = script_path_buf.to_str().unwrap();
        let cleanup = cleanup::Cleanup::File(script_path.to_string());
        {
            let mut script_file = OpenOptions::new().write(true)
                .mode(0o700)
                .create_new(true)
                .open(script_path)
                .map_err(error::Error::IO)?;
            script_file.write_all(contents.as_bytes())
                .map_err(error::Error::IO)?;
        }

        let actions = script::run_script(self.ctx.clone(), script_path)?;
        drop(cleanup);
        Ok(actions)
    }
}
