extern crate futures;
extern crate futures_cpupool;
extern crate itertools;
extern crate serde_json;
extern crate sodiumoxide;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::io::Read;
use std::sync::{Arc, Mutex};

use self::futures::{Future, Stream, Sink};
use self::futures::sync::mpsc;
use self::itertools::Itertools;
use self::tokio_core::io::{Codec, Io};
use self::tokio_service::Service as TokioService;

use super::*;
use self::api::{Request, Response};
use self::token::{TOKEN_NAME_LEN, TokenStorage};
use self::storage::{AgentStorage, DurableStorage};

pub struct ServiceCodec;

impl tokio_core::io::Codec for ServiceCodec {
    type In = Request;
    type Out = Response;

    fn decode(&mut self, buf: &mut tokio_core::io::EasyBuf) -> std::io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Deserialize JSON and return it in a Frame.
            let maybe_req: Result<Self::In, serde_json::error::Error> =
                serde_json::from_slice(line.as_slice());
            match maybe_req {
                Ok(req) => {
                    trace!("service decode {:?}", req);
                    Ok(Some(req))
                }
                Err(e) => {
                    error!("service decode failed: {}", e);
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
                trace!("service encode {:?}", msg);
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

fn read_file(path: &str) -> Result<String, Error> {
    let mut f = std::fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s).map_err(Error::IO)?;
    Ok(s)
}

type AgentOutboxMap = HashMap<String, mpsc::Sender<Message>>;

struct ServiceState<S: AgentStorage + Send + 'static> {
    storage: S,
    outboxes: AgentOutboxMap,
}

impl<S: AgentStorage + Send> ServiceState<S> {
    pub fn new(storage: S) -> ServiceState<S> {
        ServiceState {
            storage: storage,
            outboxes: AgentOutboxMap::new(),
        }
    }

    fn has_agent(&self, name: &str) -> bool {
        self.outboxes.contains_key(name)
    }

    fn remove(&mut self, name: &str) -> Result<(), Error> {
        self.storage.remove_agent(name)?;
        self.outboxes.remove(name);
        Ok(())
    }

    fn add_all_agents(&mut self, svc: &Service<S>) -> Result<(), Error> {
        let agents = self.storage.agents()?;
        for (name, glop) in agents {
            svc.spawn_agent(&name, &glop, self)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Service<S: AgentStorage + Send + 'static> {
    state: Arc<Mutex<ServiceState<S>>>,
    handle: tokio_core::reactor::Handle,
    pool: futures_cpupool::CpuPool,
}

impl<S: AgentStorage + Send> Service<S> {
    pub fn new(storage: S, h: &tokio_core::reactor::Handle) -> Result<Service<S>, Error> {
        let svc = Service {
            state: Arc::new(Mutex::new(ServiceState::new(storage))),
            handle: h.clone(),
            pool: futures_cpupool::CpuPool::new_num_cpus(),
        };
        {
            let mut state = &mut svc.state.lock().unwrap();
            state.add_all_agents(&svc)?;
        }
        Ok(svc)
    }

    fn add_agent_source(&self,
                        name: &str,
                        source: &str,
                        state: &mut ServiceState<S>)
                        -> Result<(), Error> {
        let glop_contents = read_file(source)?;
        let glop = grammar::glop(&glop_contents).map_err(Error::Parse)?;
        self.add_agent(name, glop, state)
    }

    fn add_agent(&self,
                 name: &str,
                 glop: ast::Glop,
                 state: &mut ServiceState<S>)
                 -> Result<(), Error> {
        self.spawn_agent(name, &glop, state)?;
        state.storage.add_agent(name.to_string(), glop)
    }

    fn spawn_agent(&self,
                   name: &str,
                   glop: &ast::Glop,
                   state: &mut ServiceState<S>)
                   -> Result<(), Error> {
        let runtime_st = state.storage
            .new_state(name,
                       Box::new(SenderOutbox {
                           src: name.to_string(),
                           remote: self.handle.remote().clone(),
                           state: self.state.clone(),
                       }) as Box<runtime::Outbox + Send>)?;
        let (sender, receiver) = mpsc::channel(10);
        let agent = Agent::new(glop, runtime_st, receiver)?;
        state.outboxes.insert(name.to_string(), sender);
        self.handle.spawn(self.pool
            .spawn(agent.for_each(|_| Ok(()))
                .or_else(|e| {
                    error!("{}", e);
                    Err(e)
                })
                .then(|_| Ok(()))));
        Ok(())
    }

    fn do_call(&self, req: Request) -> Result<Response, Error> {
        let mut state = self.state.lock().unwrap();
        debug!("request {:?}", &req);
        let res = match req {
            Request::Add { source: ref add_source, name: ref add_name } => {
                if state.has_agent(add_name) {
                    return Ok(Response::Error(format!("agent {} already added", add_name)));
                }
                self.add_agent_source(add_name, add_source, &mut state)?;
                Response::Add
            }
            Request::Remove { ref name } => {
                state.remove(name)?;
                Response::Remove
            }
            Request::List => Response::List { names: state.outboxes.keys().cloned().collect() },
            Request::SendTo(msg) => self.send_to(&state, msg),
            Request::Introduce(agent_roles) => {
                let mut result = vec![];
                for ref p in agent_roles.iter().combinations(2) {
                    result.push(self.send_to(&state,
                                             Message {
                                                 src: p[0].name.to_string(),
                                                 src_role: Some(p[0].role.to_string()),
                                                 dst: p[1].name.to_string(),
                                                 topic: "intro".to_string(),
                                                 contents: Obj::new(),
                                             }));
                    result.push(self.send_to(&state,
                                             Message {
                                                 src: p[1].name.to_string(),
                                                 src_role: Some(p[1].role.to_string()),
                                                 dst: p[0].name.to_string(),
                                                 topic: "intro".to_string(),
                                                 contents: Obj::new(),
                                             }));
                }
                Response::Introduce(result)
            }
        };
        debug!("response {:?}", res);
        Ok(res)
    }

    fn send_to(&self, state: &ServiceState<S>, msg: Message) -> Response {
        if let Some(sender) = state.outboxes.get(&msg.dst) {
            let resp = Response::SendTo {
                src: msg.src.to_string(),
                dst: msg.dst.to_string(),
            };
            let sender = sender.clone();
            self.handle.spawn(sender.send(msg).then(|_| Ok(())));
            resp
        } else {
            Response::Error(format!("agent {} not found", &msg.dst))
        }
    }
}

pub struct SecureServiceCodec {
    tokens: Box<TokenStorage + 'static>,
    codec: Option<crypto::SecretBoxCodec<ServiceCodec>>,
}

impl SecureServiceCodec {
    #[allow(dead_code)]
    pub fn new(tokens: Box<TokenStorage + 'static>) -> SecureServiceCodec {
        SecureServiceCodec {
            tokens: tokens,
            codec: None,
        }
    }

    fn decode_prelude(&mut self,
                      buf: &mut tokio_core::io::EasyBuf)
                      -> std::io::Result<Option<<ServiceCodec as tokio_core::io::Codec>::In>> {
        if let Some(i) = buf.as_slice().iter().take(TOKEN_NAME_LEN).position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            let id = std::str::from_utf8(line.as_slice()).map_err(to_ioerror)?;
            match self.tokens.token(id) {
                Ok(Some(ref token)) => {
                    let mut codec = crypto::SecretBoxCodec::new(ServiceCodec, token.key().clone());
                    let result = codec.decode(buf)?;
                    self.codec = Some(codec);
                    Ok(result)
                }
                Ok(None) => {
                    Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            format!("missing or invalid token name: {}", id)))
                }
                Err(e) => Err(to_ioerror(e)),
            }
        } else if buf.len() >= TOKEN_NAME_LEN {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "missing or invalid prelude"))
        } else {
            Ok(None)
        }
    }
}

impl tokio_core::io::Codec for SecureServiceCodec {
    type In = <ServiceCodec as tokio_core::io::Codec>::In;
    type Out = <ServiceCodec as tokio_core::io::Codec>::Out;

    fn decode(&mut self, buf: &mut tokio_core::io::EasyBuf) -> std::io::Result<Option<Self::In>> {
        match self.codec {
            Some(ref mut codec) => codec.decode(buf),
            None => self.decode_prelude(buf),
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        match self.codec {
            None => {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "missing or invalid prelude"))
            }
            Some(ref mut codec) => codec.encode(msg, buf),
        }
    }
}

#[derive(Clone)]
struct SenderOutbox<S: AgentStorage + Send + 'static> {
    src: String,
    remote: tokio_core::reactor::Remote,
    state: Arc<Mutex<ServiceState<S>>>,
}

impl<S: AgentStorage + Send> runtime::Outbox for SenderOutbox<S> {
    fn send_msg(&self, msg: Message) -> Result<(), Error> {
        let state = self.state.lock().unwrap();
        let sender = match state.outboxes.get(&msg.dst) {
            Some(s) => s.clone(),
            None => return Err(Error::InvalidArgument(msg.dst.to_string())),
        };
        self.remote.spawn(|_| sender.send(msg).then(|_| Ok(())));
        Ok(())
    }
}

impl<S: AgentStorage + Send> TokioService for Service<S> {
    type Request = Request;
    type Response = Response;

    type Error = std::io::Error;

    type Future = futures::BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match self.do_call(req) {
            Ok(res) => futures::future::ok(res).boxed(),
            Err(e) => {
                futures::future::ok(Response::Error(format!("agent service error: {}", e))).boxed()
            }
        }
    }
}

pub struct Server {
    addr: std::net::SocketAddr,
    path: String,
}

impl Server {
    pub fn new(addr: std::net::SocketAddr, path: &str) -> Server {
        Server {
            addr: addr,
            path: path.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        let mut core = tokio_core::reactor::Core::new().map_err(Error::IO)?;
        let handle = core.handle();
        let listener = tokio_core::net::TcpListener::bind(&self.addr, &handle).map_err(Error::IO)?;
        info!("server listening on {}", self.addr);
        let connections = listener.incoming();
        let svc_storage = DurableStorage::new(&self.path)?;
        let service = Service::new(svc_storage, &handle)?;
        let server = connections.for_each(move |(socket, _peer_addr)| {
            let (wr, rd) = socket.framed(ServiceCodec).split();
            let service = service.clone();
            let responses = rd.and_then(move |req| service.call(req));
            let responder = wr.send_all(responses)
                .or_else(|e| {
                    error!("{}", e);
                    Err(e)
                })
                .then(|_| Ok(()));
            handle.spawn(responder);
            Ok(())
        });
        core.run(server).map_err(Error::IO)
    }
}
