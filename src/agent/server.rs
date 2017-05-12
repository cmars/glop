extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate itertools;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::os::unix::fs::DirBuilderExt;
use std::sync::{Arc, Mutex};

use self::futures::{Future, Stream, Sink};
use self::futures::sync::mpsc;
use self::itertools::Itertools;
use self::tokio_io::AsyncRead;
use self::tokio_service::Service as TokioService;

use super::*;
use self::agent::{AgentStorage, DurableAgentStorage};
use self::api::{Authenticated, Request, Response};
use self::token::{DurableTokenStorage, TOKEN_NAME_LEN, TokenStorage};

pub struct ServiceCodec;

impl tokio_io::codec::Decoder for ServiceCodec {
    type Item = Request;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> std::io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Deserialize JSON and return it in a Frame.
            let maybe_req: Result<Self::Item, serde_json::error::Error> =
                serde_json::from_slice(&line[..]);
            match maybe_req {
                Ok(req) => {
                    trace!("service decode {:?}", req);
                    buf.take();
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
}

impl tokio_io::codec::Encoder for ServiceCodec {
    type Item = Response;
    type Error = std::io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut bytes::BytesMut) -> std::io::Result<()> {
        match serde_json::to_vec(&msg) {
            Ok(json) => {
                trace!("service encode {:?}", msg);
                buf.extend(&json[..]);
                Ok(())
            }
            Err(e) => {
                error!("service encode {:?} failed: {}", msg, e);
                Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
            }
        }?;
        buf.extend(&b"\n"[..]);
        Ok(())
    }
}

/// Associate local agent name with a sender that sends messages to that agent.
type AgentSenderMap = HashMap<String, mpsc::Sender<Message>>;

struct ServiceState<S: AgentStorage + Send + 'static> {
    storage: S,
    local_senders: AgentSenderMap,
}

impl<S: AgentStorage + Send> ServiceState<S> {
    pub fn new(storage: S) -> ServiceState<S> {
        ServiceState {
            storage: storage,
            local_senders: AgentSenderMap::new(),
        }
    }

    fn has_agent(&self, name: &str) -> bool {
        self.local_senders.contains_key(name)
    }

    fn remove(&mut self, name: &str) -> Result<(), Error> {
        self.storage.remove_agent(name)?;
        self.local_senders.remove(name);
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

    fn add_agent_contents(&self,
                          name: &str,
                          contents: &str,
                          state: &mut ServiceState<S>)
                          -> Result<(), Error> {
        let glop = grammar::glop(contents).map_err(Error::Parse)?;
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
                           src_agent: name.to_string(),
                           remote: self.handle.remote().clone(),
                           state: self.state.clone(),
                       }) as Box<runtime::Outbox + Send>)?;
        let (sender, receiver) = mpsc::channel(10);
        let agent = Agent::new(glop, runtime_st, receiver)?;
        state.local_senders.insert(name.to_string(), sender);
        self.handle
            .spawn(self.pool
                .spawn(agent.for_each(|_| Ok(()))
                    .or_else(|e| {
                        error!("{}", e);
                        Err(e)
                    })
                    .then(|_| Ok(()))));
        Ok(())
    }

    fn do_call(&self, req: Authenticated<Request>) -> Result<Response, Error> {
        debug!("request {:?}", &req);
        let res = match req.item {
            Request::Add { contents: ref add_contents, name: ref add_name } => {
                let mut state = self.state.lock().unwrap();
                if state.has_agent(add_name) {
                    return Ok(Response::Error(format!("agent {} already added", add_name)));
                }
                self.add_agent_contents(add_name, add_contents, &mut state)?;
                Response::Add
            }
            Request::Remove { ref name } => {
                let mut state = self.state.lock().unwrap();
                state.remove(name)?;
                Response::Remove
            }
            Request::List => {
                let state = self.state.lock().unwrap();
                Response::List { names: state.local_senders.keys().cloned().collect() }
            }
            Request::SendTo(msg) => self.send_to(msg.src_remote(&req.auth_id)),
            Request::Introduce(agent_roles) => {
                let mut result = vec![];
                for ref p in agent_roles.iter().combinations(2) {
                    result.push(self.send_to(Message::new("intro", Obj::new())
                        .src_agent(&p[0].name)
                        .src_role(Some(p[0].role.to_string()))
                        .dst_agent(&p[1].name)));
                    result.push(self.send_to(Message::new("intro", Obj::new())
                        .src_agent(&p[1].name)
                        .src_role(Some(p[1].role.to_string()))
                        .dst_agent(&p[0].name)));
                }
                Response::Introduce(result)
            }
            Request::FetchReply { ref in_reply_to } => {
                let mut state = self.state.lock().unwrap();
                let maybe_msg = state.storage
                    .fetch_remote_reply(&req.auth_id, in_reply_to)?;
                Response::FetchReply(maybe_msg)
            }
            Request::FetchMsgs => {
                let mut state = self.state.lock().unwrap();
                let msgs = state.storage.fetch_remote_msgs(&req.auth_id)?;
                Response::FetchMsgs(msgs)
            }
        };
        debug!("response {:?}", res);
        Ok(res)
    }

    fn send_to(&self, msg: Message) -> Response {
        if let Some(sender) = self.state
            .lock()
            .unwrap()
            .local_senders
            .get(&msg.dst_agent) {
            let resp = Response::SendTo {
                id: msg.id.to_string(),
                src_agent: msg.src_agent.to_string(),
                dst_agent: msg.dst_agent.to_string(),
            };
            let sender = sender.clone();
            self.handle.spawn(sender.send(msg).then(|_| Ok(())));
            resp
        } else {
            Response::Error(format!("agent {} not found", &msg.dst_agent))
        }
    }
}

#[derive(Clone)]
struct SenderOutbox<S: AgentStorage + Send + 'static> {
    src_agent: String,
    remote: tokio_core::reactor::Remote,
    state: Arc<Mutex<ServiceState<S>>>,
}

impl<S: AgentStorage + Send> runtime::Outbox for SenderOutbox<S> {
    fn send_msg(&self, msg: Message) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap();
        match msg.dst_remote.clone() {
            Some(_) => state.storage.push_remote_msg(msg),
            None => {
                let sender = match state.local_senders.get(&msg.dst_agent) {
                    Some(s) => s.clone(),
                    None => return Err(Error::InvalidArgument(msg.dst_agent.to_string())),
                };
                self.remote.spawn(|_| sender.send(msg).then(|_| Ok(())));
                Ok(())
            }
        }
    }
}

impl<S: AgentStorage + Send> TokioService for Service<S> {
    type Request = Authenticated<Request>;
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

pub struct SecureServiceCodec {
    tokens: Box<TokenStorage>,
    codec: Option<crypto::SecretBoxCodec<ServiceCodec>>,
    auth_id: Option<String>,
}

impl SecureServiceCodec {
    #[allow(dead_code)]
    pub fn new(tokens: Box<TokenStorage>) -> SecureServiceCodec {
        SecureServiceCodec {
            tokens: tokens,
            codec: None,
            auth_id: None,
        }
    }

    fn decode_prelude
        (&mut self,
         buf: &mut bytes::BytesMut)
         -> std::io::Result<Option<(String, crypto::SecretBoxCodec<ServiceCodec>)>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\0') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\0'
            buf.split_to(1);

            trace!("from {:?}", line);
            let id = std::str::from_utf8(&line[..]).map_err(to_ioerror)?;
            trace!("matched prelude, connection from {}", id);
            match self.tokens.token(id) {
                Ok(Some(ref token)) => {
                    Ok(Some((id.to_string(),
                             crypto::SecretBoxCodec::new(ServiceCodec, token.key().clone()))))
                }
                Ok(None) => {
                    Err(std::io::Error::new(std::io::ErrorKind::Other,
                                            format!("missing or invalid token name: {}", id)))
                }
                Err(e) => Err(to_ioerror(e)),
            }
        } else if buf.len() > TOKEN_NAME_LEN + 1 {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "missing or invalid prelude"))
        } else {
            Ok(None)
        }
    }
}

impl tokio_io::codec::Decoder for SecureServiceCodec {
    type Item = Authenticated<<ServiceCodec as tokio_io::codec::Decoder>::Item>;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> std::io::Result<Option<Self::Item>> {
        match self.decode_prelude(buf) {
            Ok(Some((id, codec))) => {
                self.auth_id = Some(id);
                self.codec = Some(codec);
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }
        if let (&Some(ref id), &mut Some(ref mut codec)) = (&self.auth_id, &mut self.codec) {
            trace!("decoding encrypted contents");
            match codec.decode(buf) {
                Ok(Some(req)) => {
                    buf.take();
                    Ok(Some(Authenticated {
                        auth_id: id.to_string(),
                        item: req,
                    }))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }
}

impl tokio_io::codec::Encoder for SecureServiceCodec {
    type Item = <ServiceCodec as tokio_io::codec::Encoder>::Item;
    type Error = std::io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut bytes::BytesMut) -> std::io::Result<()> {
        match self.codec {
            None => {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "missing or invalid prelude"))
            }
            Some(ref mut codec) => codec.encode(msg, buf),
        }
    }
}

pub struct Server {
    pub addr: std::net::SocketAddr,
    pub tokens_path: String,
    pub agents_path: String,
}

impl Server {
    pub fn new(addr_str: &str, path: &str) -> Result<Server, Error> {
        let addr = addr_str.parse()?;
        Server::new_addr(addr, path)
    }

    pub fn new_addr(addr: std::net::SocketAddr, path: &str) -> Result<Server, Error> {
        std::fs::DirBuilder::new().recursive(true)
            .mode(0o700)
            .create(path)
            .map_err(Error::IO)?;
        Ok(Server {
            addr: addr,
            agents_path: std::path::PathBuf::from(path)
                .join("agents")
                .to_str()
                .unwrap()
                .to_string(),
            tokens_path: std::path::PathBuf::from(path)
                .join("tokens.json")
                .to_str()
                .unwrap()
                .to_string(),
        })
    }

    pub fn run(&self) -> Result<(), Error> {
        let mut core = tokio_core::reactor::Core::new().map_err(Error::IO)?;
        let handle = core.handle();
        let listener = tokio_core::net::TcpListener::bind(&self.addr, &handle).map_err(Error::IO)?;
        let local_addr = listener.local_addr().map_err(Error::IO)?;
        info!("server listening on {}", local_addr);
        let connections = listener.incoming();
        let agent_storage = DurableAgentStorage::new(&self.agents_path);
        let service = Service::new(agent_storage, &handle)?;
        let server = connections.for_each(move |(socket, _peer_addr)| {
            let token_storage = DurableTokenStorage::new(&self.tokens_path);
            let (wr, rd) = socket.framed(SecureServiceCodec::new(Box::new(token_storage)))
                .split();
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
