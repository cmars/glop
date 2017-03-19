extern crate base64;
extern crate futures;
extern crate sodiumoxide;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_process;
extern crate tokio_service;

use std;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};
use std::process::Command;

use self::futures::{future, Future, BoxFuture, Sink, Stream};
use self::sodiumoxide::crypto::secretbox;
use self::tokio_core::io::{Codec, EasyBuf, Framed, Io};
use self::tokio_process::CommandExt;
use self::tokio_service::Service;

use super::*;
use self::context::Context;
use self::value::{Identifier, Obj, Value};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Request {
    GetVar { key: String },
    SetVar { key: String, value: String },
    UnsetVar { key: String },
    GetMsg { topic: String, key: String },
    SendMsg {
        dst: String,
        topic: String,
        contents: Obj,
    },
    ReplyMsg {
        src_topic: String,
        topic: String,
        contents: Obj,
    },
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Response {
    GetVar { key: String, value: String },
    SetVar { key: String, value: String },
    UnsetVar { key: String },
    GetMsg {
        topic: String,
        key: String,
        value: String,
    },
    SendMsg { dst: String, topic: String },
    Error(String),
}

pub struct ServiceCodec;

impl Codec for ServiceCodec {
    type In = Request;
    type Out = Response;

    fn decode(&mut self, buf: &mut EasyBuf) -> std::io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and
            // return it in a Frame.
            let maybe_req: std::result::Result<Self::In, serde_json::error::Error> =
                serde_json::from_slice(line.as_slice());
            match maybe_req {
                Ok(req) => {
                    debug!("service decode {:?}", req);
                    Ok(Some(req))
                }
                Err(e) => {
                    error!("decode failed: {}", e);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
                }
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        match serde_json::to_writer(buf, &msg) {
            Ok(_) => {
                debug!("service encode {:?}", msg);
                Ok(())
            }
            Err(e) => {
                error!("service encode failed: {}", e);
                Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
            }
        }?;
        buf.push(b'\n');
        Ok(())
    }
}

pub struct ClientCodec;

impl Codec for ClientCodec {
    type Out = Request;
    type In = Response;

    fn decode(&mut self, buf: &mut EasyBuf) -> std::io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and
            // return it in a Frame.
            let maybe_req: std::result::Result<Self::In, serde_json::error::Error> =
                serde_json::from_slice(line.as_slice());
            match maybe_req {
                Ok(req) => {
                    debug!("client decode {:?}", req);
                    Ok(Some(req))
                }
                Err(e) => {
                    error!("client decode failed: {}", e);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
                }
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        match serde_json::to_writer(buf, &msg) {
            Ok(_) => {
                debug!("client encode {:?}", msg);
                Ok(())
            }
            Err(e) => {
                error!("client encode failed: {}", e);
                Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
            }
        }?;
        buf.push(b'\n');
        Ok(())
    }
}

pub struct ClientProto {
    key: secretbox::Key,
}

impl ClientProto {
    pub fn new_from_env() -> Result<ClientProto> {
        let key_str = std::env::var("GLOP_SCRIPT_KEY").map_err(Error::Env)?;
        let key_bytes =
            base64::decode(&key_str).map_err(|e| Error::InvalidArgument(format!("{}", e)))?;
        let key = match secretbox::Key::from_slice(&key_bytes) {
            Some(k) => k,
            None => return Err(Error::InvalidArgument("GLOP_SCRIPT_KEY".to_string())),
        };
        Ok(ClientProto { key: key })
    }
}

impl<T: Io + 'static> tokio_proto::pipeline::ClientProto<T> for ClientProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, crypto::SecretBoxCodec<ClientCodec>>;
    type BindTransport = std::io::Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(crypto::SecretBoxCodec::new(ClientCodec, self.key.clone())))
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
    type Request = Request;
    type Response = Response;

    // For non-streaming protocols, service errors are always io::Error
    type Error = std::io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let mut ctx = self.ctx.lock().unwrap();
        let res = match req {
            Request::GetVar { ref key } => {
                match ctx.get_var(&Identifier::from_str(key)) {
                    Some(ref value) => {
                        Response::GetVar {
                            key: key.to_string(),
                            value: value.to_string(),
                        }
                    }
                    None => {
                        Response::GetVar {
                            key: key.to_string(),
                            value: "".to_string(),
                        }
                    }
                }
            }
            Request::SetVar { ref key, ref value } => {
                let id = Identifier::from_str(key);
                ctx.set_var(&id, Value::from_str(value));
                drop(ctx);
                let mut actions = self.actions.lock().unwrap();
                actions.push(Action::SetVar(id, value.to_string()));
                drop(actions);
                Response::SetVar {
                    key: key.to_string(),
                    value: value.to_string(),
                }
            }
            Request::UnsetVar { ref key } => {
                let id = Identifier::from_str(key);
                ctx.unset_var(&id);
                drop(ctx);
                let mut actions = self.actions.lock().unwrap();
                actions.push(Action::UnsetVar(id));
                drop(actions);
                Response::UnsetVar { key: key.to_string() }
            }
            Request::GetMsg { ref topic, ref key } => {
                match ctx.get_msg(topic, &Identifier::from_str(key)) {
                    Some(ref value) => {
                        Response::GetMsg {
                            topic: topic.to_string(),
                            key: key.to_string(),
                            value: value.to_string(),
                        }
                    }
                    None => {
                        Response::GetMsg {
                            topic: topic.to_string(),
                            key: key.to_string(),
                            value: "".to_string(),
                        }
                    }
                }
            }
            Request::SendMsg { ref dst, ref topic, ref contents } => {
                drop(ctx);
                let mut actions = self.actions.lock().unwrap();
                actions.push(Action::SendMsg {
                    dst: dst.to_string(),
                    topic: topic.to_string(),
                    contents: contents.clone(),
                });
                drop(actions);
                Response::SendMsg {
                    dst: dst.to_string(),
                    topic: topic.to_string(),
                }
            }
            Request::ReplyMsg { ref src_topic, ref topic, ref contents } => {
                if let Some(ref src_msg) = ctx.msgs.get(src_topic) {
                    let mut actions = self.actions.lock().unwrap();
                    actions.push(Action::SendMsg {
                        dst: src_msg.src.to_string(),
                        topic: topic.to_string(),
                        contents: contents.clone(),
                    });
                    drop(actions);
                    Response::SendMsg {
                        dst: src_msg.src.to_string(),
                        topic: topic.to_string(),
                    }
                } else {
                    Response::Error(format!("topic {} not found", topic))
                }
            }
        };
        future::ok(res).boxed()
    }
}

pub fn run_script(ctx: Arc<Mutex<Context>>, script_path: &str) -> Result<Vec<Action>> {
    let mut core = tokio_core::reactor::Core::new().map_err(error::Error::IO)?;
    let handle = core.handle();

    let addr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio_core::net::TcpListener::bind(&addr, &handle).map_err(error::Error::IO)?;
    let listen_addr = &listener.local_addr()
        .map_err(error::Error::IO)?;
    let connections = listener.incoming();
    let mut cmd = &mut Command::new(script_path);
    let src = {
        let ctx = ctx.lock().unwrap();
        ctx.set_env(cmd);
        ctx.src.to_string()
    };
    let key = secretbox::gen_key();
    let actions = Arc::new(Mutex::new(vec![]));
    let server_actions = actions.clone();
    let child = cmd.env("GLOP_SCRIPT_ADDR", format!("{}", listen_addr))
        .env("GLOP_SCRIPT_KEY", base64::encode(&key.0))
        .output_async(&handle)
        .then(|result| {
            match result {
                Ok(output) => {
                    let mut stdout = String::from_utf8(output.stdout).unwrap();
                    stdout.pop();
                    if !stdout.is_empty() {
                        info!("{}: stdout: {}", src, stdout);
                    }
                    let mut stderr = String::from_utf8(output.stderr).unwrap();
                    stderr.pop();
                    if !stderr.is_empty() {
                        info!("{}: stderr: {}", src, stderr);
                    }
                    if output.status.success() {
                        Ok(())
                    } else {
                        let code = match output.status.code() {
                            Some(value) => value,
                            None => 0,
                        };
                        Err(Error::Exec(code, stderr))
                    }
                }
                Err(e) => Err(Error::IO(e)),
            }
        });
    let server = connections.for_each(move |(socket, _peer_addr)| {
            let (wr, rd) = socket.framed(crypto::SecretBoxCodec::new(ServiceCodec, key.clone()))
                .split();
            let service = ScriptService::new(ctx.clone(), server_actions.clone());
            let responses = rd.and_then(move |req| service.call(req));
            let responder = wr.send_all(responses).then(|_| Ok(()));
            handle.spawn(responder);
            Ok(())
        })
        .map_err(Error::IO);
    let comb = server.select(child);
    match core.run(comb) {
        Err((e, _)) => {
            return Err(e);
        }
        Ok(_) => Ok(actions.lock().unwrap().clone()),
    }
}
