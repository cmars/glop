extern crate fs2;
extern crate futures;
extern crate futures_cpupool;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::io::{Read, Write};
use std::os::unix::fs::DirBuilderExt;
use std::sync::{Arc, Mutex};

use self::fs2::FileExt;
use self::futures::{Future, Stream, Sink};
use self::futures::sync::mpsc;
use self::tokio_core::io::{Framed, Io};
use self::tokio_service::Service as TokioService;

use super::error::Error;
use super::cleanup;
use super::grammar;
use super::runtime;
use super::runtime::Storage;
use super::value::Obj;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Request {
    Add { source: String, name: String },
    Remove { name: String },
    List,
    SendTo(Envelope), /* RecvFrom { handle: String },
                       * Introduce { names: Vec<String> }, */
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Response {
    Add,
    Remove,
    List { names: Vec<String> },
    SendTo, /* RecvFrom { topic: String, contents: Obj },
             * Introduce, */
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

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Envelope {
    // src: String,
    pub dst: String,
    pub topic: String,
    pub contents: Obj,
}

pub struct Agent<S: Storage> {
    matches: Vec<runtime::Match>,
    st: runtime::State<S>,
    receiver: mpsc::Receiver<Envelope>,
    match_index: usize,
}

impl<S: Storage> Agent<S> {
    pub fn new_from_file(path: &str,
                         st: runtime::State<S>,
                         receiver: mpsc::Receiver<Envelope>)
                         -> Result<Agent<S>, Error> {
        let glop_contents = read_file(path)?;
        let glop = grammar::glop(&glop_contents).map_err(Error::Parse)?;
        let mut st = st;
        if st.storage().seq() == 0 {
            st.mut_storage().push_msg("init", Obj::new())?;
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
            Err(_) => {
                match self.st.rollback(txn) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(futures::Async::Ready(Some(())))
    }
}

impl<S: Storage> futures::stream::Stream for Agent<S> {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Option<Self::Item>, Self::Error> {
        // TODO: poll mpsc channel (receiver end) for state changes & apply?
        match self.receiver.poll() {
            Ok(futures::Async::Ready(Some(env))) => {
                self.st.mut_storage().push_msg(&env.topic, env.contents)?;
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

#[derive(Clone)]
pub struct Service {
    senders: Arc<Mutex<HashMap<String, mpsc::Sender<Envelope>>>>,
    handle: tokio_core::reactor::Handle,
    pool: futures_cpupool::CpuPool,
    path: String,
}

impl Service {
    pub fn new(path: &str, h: &tokio_core::reactor::Handle) -> Service {
        Service {
            senders: Arc::new(Mutex::new(HashMap::new())),
            handle: h.clone(),
            pool: futures_cpupool::CpuPool::new_num_cpus(),
            path: path.to_string(),
        }
    }

    fn do_call(&self, req: Request) -> Result<Response, Error> {
        let mut senders = self.senders.lock().unwrap();
        debug!("request {:?}", &req);
        let res = match req {
            Request::Add { source: ref add_source, name: ref add_name } => {
                let (sender, receiver) = mpsc::channel(10);
                if senders.contains_key(add_name) {
                    return Ok(Response::Error(format!("agent {} already added", add_name)));
                }
                senders.insert(add_name.to_string(), sender);
                let agent_path = std::path::PathBuf::from(&self.path)
                    .join(add_name)
                    .to_str()
                    .unwrap()
                    .to_string();
                let storage = runtime::DurableStorage::new(&agent_path)?;
                let agent =
                    Agent::new_from_file(add_source, runtime::State::new(storage), receiver)?;
                self.handle.spawn(self.pool
                    .spawn(agent.for_each(|_| Ok(()))
                        .or_else(|e| {
                            error!("{}", e);
                            Err(e)
                        })
                        .then(|_| Ok(()))));
                Response::Add
            }
            Request::Remove { ref name } => {
                senders.remove(name);
                Response::Remove
            }
            Request::List => Response::List { names: senders.keys().cloned().collect() },
            Request::SendTo(env) => {
                let sender = match senders.get(&env.dst) {
                    Some(s) => s.clone(),
                    None => return Ok(Response::Error(format!("agent {} not found", &env.dst))),
                };
                self.handle.spawn(sender.send(env).then(|_| Ok(())));
                Response::SendTo
            }
            // RecvFrom { handle: String },
            // Introduce { names: Vec<String> },
        };
        debug!("response {:?}", res);
        Ok(res)
    }
}

impl TokioService for Service {
    type Request = Request;
    type Response = Response;

    type Error = std::io::Error;

    type Future = futures::BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match self.do_call(req) {
            Ok(res) => futures::future::ok(res).boxed(),
            Err(err) => {
                futures::future::err(std::io::Error::new(std::io::ErrorKind::Other,
                                                         err.description()))
                    .boxed()
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

pub fn run_server() -> Result<(), std::io::Error> {
    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();
    let addr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio_core::net::TcpListener::bind(&addr, &handle)?;
    let listen_addr = &listener.local_addr()?;
    info!("server listening on {}", &listen_addr);
    let _lock = agent_lock(listen_addr)?;
    let connections = listener.incoming();
    let glop_dir = glop_dir()?;
    debug!("glop_dir={}", glop_dir);
    let service = Service::new(&glop_dir, &handle);
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
    core.run(server)
}

pub fn read_agent_addr() -> std::io::Result<String> {
    let lock_path = agent_lock_path()?;
    let mut agent_file = std::fs::File::open(lock_path)?;
    let mut result = String::new();
    agent_file.read_to_string(&mut result)?;
    Ok(result)
}

fn agent_lock(addr: &std::net::SocketAddr) -> std::io::Result<cleanup::Cleanup> {
    let lock_path = agent_lock_path()?;
    let mut f = std::fs::OpenOptions::new().read(true).write(true).create(true).open(&lock_path)?;
    f.try_lock_exclusive()?;
    write!(f, "{}", addr)?;
    f.flush()?;
    Ok(cleanup::Cleanup::Lock(f, lock_path))
}

fn glop_dir() -> std::io::Result<String> {
    let mut path_buf = match std::env::home_dir() {
        Some(home) => home,
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "cannot determine home directory"))
        }
    };
    path_buf.push(".glop");
    let path = path_buf.to_str().unwrap().to_string();
    std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&path)?;
    Ok(path)
}

fn agent_lock_path() -> std::io::Result<String> {
    let mut path_buf = match std::env::home_dir() {
        Some(home) => home,
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "cannot determine home directory"))
        }
    };
    path_buf.push(".glop.agent");
    Ok(path_buf.to_str().unwrap().to_string())
}
