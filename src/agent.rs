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
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::sync::{Arc, Mutex};

use self::futures::{Future, Stream, Sink};
use self::futures::sync::mpsc;
use self::itertools::Itertools;
use self::sodiumoxide::crypto::secretbox;
use self::tokio_core::io::{Framed, Io};
use self::tokio_service::Service as TokioService;

use super::ast;
use super::error::{Error, to_ioerror};
use super::grammar;
use super::runtime;
use super::value::{Message, Obj};

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

pub struct ClientCodec;
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

            // Turn this data into a UTF string and
            // return it in a Frame.
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

impl tokio_core::io::Codec for ClientCodec {
    type Out = Request;
    type In = Response;

    fn decode(&mut self, buf: &mut tokio_core::io::EasyBuf) -> std::io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and
            // return it in a Frame.
            let maybe_req: Result<Self::In, serde_json::error::Error> =
                serde_json::from_slice(line.as_slice());
            match maybe_req {
                Ok(req) => {
                    debug!("client decode: {:?}", req);
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
                debug!("client encode: {:?}", msg);
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

pub struct Agent<S: runtime::Storage> {
    matches: Vec<runtime::Match>,
    st: runtime::State<S>,
    receiver: mpsc::Receiver<Message>,
    match_index: usize,
}

impl<S: runtime::Storage> Agent<S> {
    pub fn new(glop: &ast::Glop,
               st: runtime::State<S>,
               receiver: mpsc::Receiver<Message>)
               -> Result<Agent<S>, Error> {
        let name = st.name().to_string();
        let mut st = st;
        let (seq, _) = st.mut_storage().load()?;
        if seq == 0 {
            st.mut_storage()
                .push_msg(Message {
                    src: "".to_string(),
                    src_role: None,
                    topic: "init".to_string(),
                    dst: name,
                    contents: Obj::new(),
                })?;
        }
        let m_excs = glop.matches
            .iter()
            .map(|m_ast| runtime::Match::new_from_ast(&m_ast))
            .collect::<Vec<_>>();
        Ok(Agent {
            matches: m_excs,
            st: st,
            receiver: receiver,
            match_index: 0,
        })
    }

    fn poll_matches(&mut self) -> futures::Poll<Option<()>, Error> {
        let i = self.match_index % self.matches.len();
        let m = &self.matches[i];
        // TODO: intelligent selection of next match?
        self.match_index = self.match_index + 1;
        let mut txn = match self.st.eval(m.clone()) {
            Ok(Some(txn)) => txn,
            Ok(None) => return Ok(futures::Async::Ready(Some(()))),
            Err(e) => return Err(e),
        };
        // TODO: graceful agent termination (nothing left to do)?
        let result = self.st.commit(&mut txn);
        match result {
            Ok(_) => {}
            Err(e) => {
                error!("transaction seq={} failed: {}", txn.seq, e);
                match self.st.rollback(txn) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(futures::Async::Ready(Some(())))
    }
}

impl<S: runtime::Storage> futures::stream::Stream for Agent<S> {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Option<Self::Item>, Self::Error> {
        // TODO: poll mpsc channel (receiver end) for state changes & apply?
        match self.receiver.poll() {
            Ok(futures::Async::Ready(Some(msg))) => {
                self.st.mut_storage().push_msg(msg)?;
            }
            Ok(futures::Async::Ready(None)) => return Ok(futures::Async::Ready(None)),
            Ok(futures::Async::NotReady) => {}
            Err(_) => return Ok(futures::Async::Ready(None)),
        }
        let result = self.poll_matches();
        std::thread::sleep(std::time::Duration::from_millis(100));
        result
    }
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut f = std::fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s).map_err(Error::IO)?;
    Ok(s)
}

type AgentOutboxMap = HashMap<String, mpsc::Sender<Message>>;

struct ServiceState<S: Storage + Send + 'static> {
    storage: S,
    outboxes: AgentOutboxMap,
}

impl<S: Storage + Send> ServiceState<S> {
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
pub struct Service<S: Storage + Send + 'static> {
    state: Arc<Mutex<ServiceState<S>>>,
    handle: tokio_core::reactor::Handle,
    pool: futures_cpupool::CpuPool,
}

impl<S: Storage + Send> Service<S> {
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

pub enum Token {
    Admin { name: String, key: secretbox::Key },
    Peer {
        addr: std::net::SocketAddr,
        key: secretbox::Key,
    },
}

impl Token {
    pub fn to_str(&self) -> String {
        match self {
            &Token::Admin { ref name, key: _ } => name.to_string(),
            &Token::Peer { ref addr, key: _ } => format!("{}", addr),
        }
    }

    pub fn key(&self) -> &secretbox::Key {
        match self {
            &Token::Admin { ref key, name: _ } => key,
            &Token::Peer { ref key, addr: _ } => key,
        }
    }
}

pub trait Storage {
    type RuntimeStorage: runtime::Storage + Send;

    fn new_state(&self,
                 name: &str,
                 outbox: Box<runtime::Outbox + Send + 'static>)
                 -> Result<runtime::State<Self::RuntimeStorage>, Error>;
    fn add_agent(&mut self, name: String, glop: ast::Glop) -> Result<(), Error>;
    fn remove_agent(&mut self, name: &str) -> Result<(), Error>;
    fn agents(&self) -> Result<HashMap<String, ast::Glop>, Error>;

    // fn add_token(&mut self, token: Token) -> Result<String, Error>
    // fn remove_token(&mut self, id: String) -> Result<String, Error>
    // fn tokens(&self) -> Result<Vec<Token>>,
    //
}

#[derive(Clone)]
struct MemStorage {
    agents: HashMap<String, ast::Glop>,
}

impl MemStorage {
    #[allow(dead_code)]
    fn new() -> MemStorage {
        MemStorage { agents: HashMap::new() }
    }
}

impl Storage for MemStorage {
    type RuntimeStorage = runtime::MemStorage;

    fn new_state(&self,
                 name: &str,
                 outbox: Box<runtime::Outbox + Send + 'static>)
                 -> Result<runtime::State<Self::RuntimeStorage>, Error> {
        Ok(runtime::State::new_outbox(name, runtime::MemStorage::new(), outbox))
    }

    fn add_agent(&mut self, name: String, glop: ast::Glop) -> Result<(), Error> {
        if self.agents.contains_key(&name) {
            return Err(Error::AgentExists(name));
        }
        self.agents.insert(name, glop);
        Ok(())
    }

    fn remove_agent(&mut self, name: &str) -> Result<(), Error> {
        self.agents.remove(name);
        Ok(())
    }

    fn agents(&self) -> Result<HashMap<String, ast::Glop>, Error> {
        Ok(self.agents.clone())
    }
}

#[derive(Clone)]
struct DurableStorage {
    path: String,
    agents_path: String,
}

impl DurableStorage {
    fn new(path: &str) -> Result<DurableStorage, Error> {
        std::fs::DirBuilder::new().recursive(true)
            .mode(0o700)
            .create(path)
            .map_err(Error::IO)?;
        Ok(DurableStorage {
            path: path.to_string(),
            agents_path: std::path::PathBuf::from(path)
                .join("agents.json")
                .to_str()
                .unwrap()
                .to_string(),
        })
    }

    fn load(&self) -> Result<HashMap<String, ast::Glop>, Error> {
        if !std::path::PathBuf::from(&self.agents_path).exists() {
            return Ok(HashMap::new());
        }
        let agents_file = std::fs::OpenOptions::new().read(true)
            .open(&self.agents_path)?;
        let agents: HashMap<String, ast::Glop> =
            serde_json::from_reader(agents_file).map_err(to_ioerror)
                .map_err(Error::IO)?;
        Ok(agents)
    }

    fn save(&self, agents: HashMap<String, ast::Glop>) -> Result<(), Error> {
        let mut agents_file = std::fs::OpenOptions::new().write(true)
            .mode(0o600)
            .create(true)
            .truncate(true)
            .open(&self.agents_path)?;
        serde_json::to_writer(&mut agents_file, &agents).map_err(to_ioerror)
            .map_err(Error::IO)?;
        Ok(())
    }
}

impl Storage for DurableStorage {
    type RuntimeStorage = runtime::DurableStorage;

    fn new_state(&self,
                 name: &str,
                 outbox: Box<runtime::Outbox + Send + 'static>)
                 -> Result<runtime::State<Self::RuntimeStorage>, Error> {
        let runtime_path = std::path::PathBuf::from(&self.path)
            .join(name)
            .to_str()
            .unwrap()
            .to_string();
        let runtime_storage = runtime::DurableStorage::new(&runtime_path)?;
        Ok(runtime::State::new_outbox(name, runtime_storage, outbox))
    }

    fn add_agent(&mut self, name: String, glop: ast::Glop) -> Result<(), Error> {
        let mut agents = self.load()?;
        agents.insert(name, glop);
        self.save(agents)?;
        Ok(())
    }

    fn remove_agent(&mut self, name: &str) -> Result<(), Error> {
        let mut agents = self.load()?;
        agents.remove(name);
        self.save(agents)?;
        Ok(())
    }

    fn agents(&self) -> Result<HashMap<String, ast::Glop>, Error> {
        let agents = self.load()?;
        Ok(agents)
    }
}

#[derive(Clone)]
struct SenderOutbox<S: Storage + Send + 'static> {
    src: String,
    remote: tokio_core::reactor::Remote,
    state: Arc<Mutex<ServiceState<S>>>,
}

impl<S: Storage + Send> runtime::Outbox for SenderOutbox<S> {
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

impl<S: Storage + Send> TokioService for Service<S> {
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

pub struct ClientProto;

impl<T: Io + 'static> tokio_proto::pipeline::ClientProto<T> for ClientProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, ClientCodec>;
    type BindTransport = Result<Self::Transport, std::io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ClientCodec))
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
