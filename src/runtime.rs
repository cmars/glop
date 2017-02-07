extern crate futures;
extern crate textnonce;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_process;
extern crate tokio_service;

use std;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::process::Command;
use std::sync::{Arc, Mutex};

use self::futures::{future, Future, BoxFuture, Sink, Stream};
use self::tokio_core::io::Io;
use self::tokio_process::CommandExt;
use self::tokio_service::Service;

use super::ast;
use super::error;
use super::cleanup;
use super::script;
use super::value::{Value, Identifier, Obj};

pub type Msg = Obj;

pub struct Match {
    conditions: Vec<Condition>,
    msg_topics: HashSet<String>,
    actions: Vec<Action>,
}

impl Match {
    fn new() -> Match {
        Match {
            conditions: vec![],
            msg_topics: HashSet::new(),
            actions: vec![],
        }
    }

    pub fn new_from_ast(m_ast: &ast::Match) -> Match {
        let mut m_exc = Match::new();
        for c_ast in &m_ast.conditions {
            m_exc.conditions.push(Condition::new(c_ast));
            match c_ast {
                &ast::Condition::Message(ref topic) => {
                    m_exc.msg_topics.insert(topic.to_string());
                }
                _ => (),
            }
        }
        for a_ast in &m_ast.actions {
            m_exc.actions.push(Action::new(a_ast));
        }
        m_exc
    }
}

type RuntimeResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Base(error::Error),
    Exec(i32, String),
    Acknowledge(String),
    UnsupportedAction,
}

impl From<error::Error> for Error {
    fn from(err: error::Error) -> Error {
        Error::Base(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Base(ref err) => write!(f, "{}", err),
            Error::Exec(code, ref stderr) => write!(f, "script exit code {}: {}", code, stderr),
            Error::Acknowledge(ref topic) => write!(f, "invalid acknowledge: {}", topic),
            Error::UnsupportedAction => write!(f, "unsupported action"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Base(ref err) => err.description(),
            Error::Exec(_, ref stderr) => stderr,
            Error::Acknowledge(ref topic) => topic,
            Error::UnsupportedAction => "unsupported action",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Base(ref err) => Some(err),
            _ => None,
        }
    }
}

pub struct Context {
    pub vars: HashMap<String, Value>,
    pub msgs: HashMap<String, Msg>,
}

impl Context {
    fn set_env(&self, cmd: &mut Command) {
        for (k, v) in Value::to_env(&self.vars) {
            cmd.env(k, v);
        }
        for (topic, msg) in &self.msgs {
            for (k, v) in Value::to_env(&msg) {
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

    fn get_msg<'a>(&'a mut self, topic: &str, key: &Identifier) -> Option<&'a Value> {
        match self.msgs.get(topic) {
            Some(ref obj) => key.get(obj),
            None => None,
        }
    }

    fn get_var<'b>(&'b mut self, key: &Identifier) -> Option<&'b Value> {
        key.get(&mut self.vars)
    }

    fn set_var(&mut self, key: &Identifier, value: Value) {
        key.set(&mut self.vars, value)
    }

    fn unset_var(&mut self, key: &Identifier) {
        key.unset(&mut self.vars)
    }

    fn ack_msg(&mut self, topic: &str) -> RuntimeResult<()> {
        match self.msgs.remove(topic) {
            Some(_) => Ok(()),
            None => Err(Error::Acknowledge(topic.to_string())),
        }
    }
}

pub struct Transaction {
    pub seq: i32,
    pub ctx: Arc<Mutex<Context>>,
    pub applied: Vec<Action>,
}

impl Transaction {
    fn new(seq: i32, ctx: Context) -> Transaction {
        Transaction {
            seq: seq,
            ctx: Arc::new(Mutex::new(ctx)),
            applied: vec![],
        }
    }

    pub fn apply(&mut self, m: &Match) -> RuntimeResult<Vec<Action>> {
        for action in &m.actions {
            self.apply_action(&action)?;
        }
        Ok(self.applied.clone())
    }

    fn eval(&self, cond: &Condition) -> bool {
        let ctx = self.ctx.lock().unwrap();
        match cond {
            &Condition::Cmp(ref l, ref op, ref r) => {
                match l.get(&ctx.vars) {
                    Some(v) => op.eval(v, r),
                    None => false,
                }
            }
            &Condition::IsSet(ref k) => k.is_set(&ctx.vars),
            &Condition::Message(ref k) => ctx.msgs.contains_key(k),
        }
    }

    pub fn with_context<F>(&mut self, f: F) -> RuntimeResult<()>
        where F: Fn(&mut Context) -> RuntimeResult<()>
    {
        let mut ctx = self.ctx.lock().unwrap();
        f(&mut ctx)
    }

    fn apply_action(&mut self, action: &Action) -> RuntimeResult<()> {
        let mut actions = match action {
            &Action::SetVar(ref k, ref v) => {
                let mut ctx = self.ctx.lock().unwrap();
                ctx.set_var(k, Value::Str(v.to_string()));
                vec![action.clone()]
            }
            &Action::UnsetVar(ref k) => {
                let mut ctx = self.ctx.lock().unwrap();
                ctx.unset_var(k);
                vec![action.clone()]
            }
            &Action::Acknowledge(ref k) => {
                let mut ctx = self.ctx.lock().unwrap();
                ctx.ack_msg(k)?;
                vec![action.clone()]
            }
            &Action::Script(ref contents) => self.exec_script(contents)?,
        };
        self.applied.append(&mut actions);
        Ok(())
    }

    fn exec_script(&mut self, contents: &str) -> RuntimeResult<Vec<Action>> {
        let mut script_path_buf = std::env::temp_dir();
        let script_path_base = textnonce::TextNonce::sized_urlsafe(32).unwrap().into_string();
        script_path_buf.push(&script_path_base);
        let script_path = script_path_buf.to_str().unwrap();
        let cleanup = cleanup::Cleanup::File(script_path.to_string());
        {
            let mut script_file = OpenOptions::new().write(true)
                .mode(0o700)
                .create_new(true)
                .open(script_path)
                .map_err(error::Error::IO)
                .map_err(Error::Base)?;
            script_file.write_all(contents.as_bytes())
                .map_err(error::Error::IO)
                .map_err(Error::Base)?;
        }

        let actions = run_script(self.ctx.clone(), script_path)?;
        drop(cleanup);
        Ok(actions)
    }
}

pub struct ScriptService {
    ctx: Arc<Mutex<Context>>,
    actions: Arc<Mutex<Vec<Action>>>,
}

impl ScriptService {
    fn new(ctx: Arc<Mutex<Context>>, actions: Arc<Mutex<Vec<Action>>>) -> ScriptService {
        ScriptService {
            ctx: ctx,
            actions: actions,
        }
    }
}

impl Service for ScriptService {
    // These types must match the corresponding protocol types:
    type Request = script::Request;
    type Response = script::Response;

    // For non-streaming protocols, service errors are always io::Error
    type Error = std::io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let mut ctx = self.ctx.lock().unwrap();
        let res = match req {
            script::Request::GetVar { ref key } => {
                match ctx.get_var(&Identifier::from_str(key)) {
                    Some(ref value) => {
                        script::Response::GetVar {
                            key: key.to_string(),
                            value: value.to_string(),
                        }
                    }
                    None => {
                        script::Response::GetVar {
                            key: key.to_string(),
                            value: "".to_string(),
                        }
                    }
                }
            }
            script::Request::SetVar { ref key, ref value } => {
                let id = Identifier::from_str(key);
                ctx.set_var(&id, Value::from_str(value));
                drop(ctx);
                let mut actions = self.actions.lock().unwrap();
                actions.push(Action::SetVar(id, value.to_string()));
                drop(actions);
                script::Response::SetVar {
                    key: key.to_string(),
                    value: value.to_string(),
                }
            }
            script::Request::GetMsg { ref topic, ref key } => {
                match ctx.get_msg(topic, &Identifier::from_str(key)) {
                    Some(ref value) => {
                        script::Response::GetMsg {
                            topic: topic.to_string(),
                            key: key.to_string(),
                            value: value.to_string(),
                        }
                    }
                    None => {
                        script::Response::GetMsg {
                            topic: topic.to_string(),
                            key: key.to_string(),
                            value: "".to_string(),
                        }
                    }
                }
            }
        };
        future::ok(res).boxed()
    }
}

fn run_script(ctx: Arc<Mutex<Context>>, script_path: &str) -> Result<Vec<Action>, Error> {
    let mut core = tokio_core::reactor::Core::new().map_err(error::Error::IO)
        .map_err(Error::Base)?;
    let handle = core.handle();

    let addr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio_core::net::TcpListener::bind(&addr, &handle).map_err(error::Error::IO)
        .map_err(Error::Base)?;
    let listen_addr = &listener.local_addr()
        .map_err(error::Error::IO)
        .map_err(Error::Base)?;
    let connections = listener.incoming();
    let mut cmd = &mut Command::new(script_path);
    {
        let ctx = ctx.lock().unwrap();
        ctx.set_env(cmd);
    }
    let actions = Arc::new(Mutex::new(vec![]));
    let server_actions = actions.clone();
    let child = cmd.env("ADDR", format!("{}", listen_addr))
        .output_async(&handle)
        .then(|result| {
            match result {
                Ok(output) => {
                    if output.status.success() {
                        print!("stdout: {}", String::from_utf8(output.stdout).unwrap());
                        Ok(())
                    } else {
                        let code = match output.status.code() {
                            Some(value) => value,
                            None => 0,
                        };
                        let stderr = String::from_utf8(output.stderr).unwrap();
                        print!("stderr: {}", stderr);
                        Err(Error::Exec(code, stderr))
                    }
                }
                Err(e) => Err(Error::Base(error::Error::IO(e))),
            }
        });
    let server = connections.for_each(move |(socket, _peer_addr)| {
            let (wr, rd) = socket.framed(script::ServiceCodec).split();
            let service = ScriptService::new(ctx.clone(), server_actions.clone());
            let responses = rd.and_then(move |req| service.call(req));
            let responder = wr.send_all(responses).then(|_| Ok(()));
            handle.spawn(responder);
            Ok(())
        })
        .map_err(error::Error::IO)
        .map_err(Error::Base);
    let comb = server.select(child);
    match core.run(comb) {
        Err((e, _)) => {
            return Err(e);
        }
        Ok(_) => Ok(actions.lock().unwrap().clone()),
    }
}

pub enum Condition {
    Cmp(Identifier, CmpOpcode, String),
    IsSet(Identifier),
    Message(String),
}

impl Condition {
    fn new(c_ast: &ast::Condition) -> Condition {
        match c_ast {
            &ast::Condition::Cmp(ref l, ref op, ref r) => {
                Condition::Cmp(Identifier::from_ast(l), CmpOpcode::new(op), r.to_string())
            }
            &ast::Condition::IsSet(ref k) => Condition::IsSet(Identifier::from_ast(k)),
            &ast::Condition::Message(ref k) => Condition::Message(k.to_string()),
        }
    }
}

pub enum CmpOpcode {
    Equal,
    NotEqual,
}

impl CmpOpcode {
    fn new(c_ast: &ast::CmpOpcode) -> CmpOpcode {
        match c_ast {
            &ast::CmpOpcode::Equal => CmpOpcode::Equal,
            &ast::CmpOpcode::NotEqual => CmpOpcode::NotEqual,
        }
    }

    fn eval(&self, l: &Value, r: &str) -> bool {
        let v = match l {
            &Value::Int(i) => i.to_string(),
            &Value::Str(ref s) => s.to_string(),
            _ => {
                return false;
            } 
        };
        match self {
            &CmpOpcode::Equal => v == r,
            &CmpOpcode::NotEqual => v != r,
        }
    }
}

pub enum Action {
    SetVar(Identifier, String),
    UnsetVar(Identifier),
    Acknowledge(String),
    Script(String),
}

impl Action {
    fn new(a_ast: &ast::Action) -> Action {
        match a_ast {
            &ast::Action::SetVar(ref k, ref v) => {
                Action::SetVar(Identifier::from_ast(k), v.to_string())
            }
            &ast::Action::UnsetVar(ref k) => Action::UnsetVar(Identifier::from_ast(k)),
            &ast::Action::Acknowledge(ref topic) => Action::Acknowledge(topic.to_string()),
            &ast::Action::Script(ref contents) => Action::Script(contents.to_string()),
            _ => panic!(format!("action {} not implemented yet", a_ast)),
        }
    }
}

impl Clone for Action {
    fn clone(&self) -> Action {
        match self {
            &Action::SetVar(ref k, ref v) => Action::SetVar(k.clone(), v.clone()),
            &Action::UnsetVar(ref k) => Action::UnsetVar(k.clone()),
            &Action::Acknowledge(ref v) => Action::Acknowledge(v.clone()),
            &Action::Script(ref v) => Action::Script(v.clone()),
        }
    }
}

pub trait State {
    fn push_msg(&mut self, topic: &str, msg: Msg);
    fn next_messages(&self, topics: &HashSet<String>) -> HashMap<String, Msg>;
    fn eval(&mut self, m: &Match) -> Option<Transaction>;
    fn commit(&mut self, actions: &Vec<Action>) -> RuntimeResult<i32>;
    fn apply_action(&mut self, action: &Action) -> RuntimeResult<()>;
    fn get_var<'b>(&'b mut self, key: &Identifier) -> Option<&'b Value>;
    fn set_var(&mut self, key: &Identifier, value: Value);
    fn unset_var(&mut self, key: &Identifier);
    fn ack_msg(&mut self, topic: &str) -> RuntimeResult<()>;
}

pub struct MemState {
    seq: i32,
    vars: HashMap<String, Value>,
    inbox: HashMap<String, Vec<Msg>>,
}

impl MemState {
    pub fn new() -> MemState {
        MemState {
            seq: 0,
            vars: HashMap::new(),
            inbox: HashMap::new(),
        }
    }
}

impl State for MemState {
    fn push_msg(&mut self, topic: &str, msg: Msg) {
        match self.inbox.get_mut(topic) {
            Some(v) => {
                v.push(msg);
                return;
            }
            _ => {}
        }
        self.inbox.insert(topic.to_string(), vec![msg]);
    }

    fn next_messages(&self, topics: &HashSet<String>) -> HashMap<String, Msg> {
        let mut next: HashMap<String, Msg> = HashMap::new();
        for (k, v) in &self.inbox {
            if !topics.contains(k) {
                continue;
            }
            match v.last() {
                Some(msg) => {
                    next.insert(k.to_string(), msg.clone());
                }
                None => (),
            }
        }
        next
    }

    fn eval(&mut self, m: &Match) -> Option<Transaction> {
        let ctx = Context {
            vars: self.vars.clone(),
            msgs: self.next_messages(&m.msg_topics),
        };
        let txn = Transaction::new(self.seq, ctx);
        let is_match = m.conditions
            .iter()
            .fold(true, |acc, c| acc && txn.eval(c));
        if !is_match {
            return None;
        }
        Some(txn)
    }

    fn commit(&mut self, actions: &Vec<Action>) -> RuntimeResult<i32> {
        for action in actions {
            self.apply_action(action)?;
        }
        let result = self.seq;
        self.seq += 1;
        Ok(result)
    }

    fn apply_action(&mut self, action: &Action) -> RuntimeResult<()> {
        match action {
            &Action::SetVar(ref k, ref v) => {
                self.set_var(k, Value::Str(v.to_string()));
                Ok(())
            }
            &Action::UnsetVar(ref k) => {
                self.unset_var(k);
                Ok(())
            }
            &Action::Acknowledge(ref k) => self.ack_msg(k),
            _ => Err(Error::UnsupportedAction),
        }
    }

    fn get_var<'b>(&'b mut self, key: &Identifier) -> Option<&'b Value> {
        key.get(&mut self.vars)
    }

    fn set_var(&mut self, key: &Identifier, value: Value) {
        key.set(&mut self.vars, value)
    }

    fn unset_var(&mut self, key: &Identifier) {
        key.unset(&mut self.vars)
    }

    fn ack_msg(&mut self, topic: &str) -> RuntimeResult<()> {
        match self.inbox.get_mut(topic) {
            Some(v) => {
                v.pop();
                Ok(())
            }
            None => Err(Error::Acknowledge(topic.to_string())),
        }
    }
}
