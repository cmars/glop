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
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};

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
pub enum Request {
    Add { source: String, name: String },
    Remove { name: String },
    List,
    SendTo(Envelope), /* RecvFrom { handle: String },
                       * Introduce { names: Vec<String> }, */
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Add,
    Remove,
    List { names: Vec<String> },
    SendTo, /* RecvFrom { topic: String, contents: Obj },
             * Introduce, */
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
                Ok(req) => Ok(Some(req)),
                Err(e) => {
                    println!("decode failed: {}", e);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
                }
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        match serde_json::to_writer(buf, &msg) {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e.description())),
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
                Ok(req) => Ok(Some(req)),
                Err(e) => {
                    println!("decode failed: {}", e);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
                }
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        match serde_json::to_writer(buf, &msg) {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e.description())),
        }?;
        buf.push(b'\n');
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
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
        st.mut_storage().push_msg("init", Obj::new())?;
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
        let txn = match self.st.eval(m.clone()) {
            Ok(Some(txn)) => txn,
            Ok(None) => return Ok(futures::Async::NotReady),
            Err(e) => return Err(e),
        };
        // TODO: intelligent selection of next match?
        self.match_index = self.match_index + 1;
        // TODO: graceful agent termination (nothing left to do)?
        match self.st.commit(txn) {
            Ok(_) => Ok(futures::Async::Ready(Some(()))),
            Err(e) => Err(e),
        }
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
}

impl Service {
    pub fn new(h: &tokio_core::reactor::Handle) -> Service {
        Service {
            senders: Arc::new(Mutex::new(HashMap::new())),
            handle: h.clone(),
            pool: futures_cpupool::CpuPool::new_num_cpus(),
        }
    }

    fn do_call(&self, req: Request) -> Result<Response, Error> {
        let mut senders = self.senders.lock().unwrap();
        let res = match req {
            Request::Add { source: ref add_source, name: ref add_name } => {
                let (sender, receiver) = mpsc::channel(10);
                senders.insert(add_name.clone(), sender);
                let agent = Agent::new_from_file(add_source,
                                                 runtime::State::new(runtime::MemStorage::new()),
                                                 receiver)?;
                self.handle.spawn(self.pool
                    .spawn(agent.for_each(|_| Ok(()))
                        .or_else(|e| {
                            println!("{}", e);
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
                    None => return Ok(Response::SendTo), // TODO: handle unmatched dst
                };
                self.handle.spawn(sender.send(env).then(|_| Ok(())));
                Response::SendTo
            }
            // RecvFrom { handle: String },
            // Introduce { names: Vec<String> },
        };
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
    let _agent_file_cleanup = write_agent_addr(listen_addr)?;
    let connections = listener.incoming();
    let service = Service::new(&handle);
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (wr, rd) = socket.framed(ServiceCodec).split();
        let service = service.clone();
        let responses = rd.and_then(move |req| service.call(req));
        let responder = wr.send_all(responses)
            .or_else(|e| {
                println!("{}", e);
                Err(e)
            })
            .then(|_| Ok(()));
        handle.spawn(responder);
        Ok(())
    });
    core.run(server)
}

fn write_agent_addr(addr: &std::net::SocketAddr) -> Result<cleanup::Cleanup, std::io::Error> {
    let mut agent_path_buf = match std::env::home_dir() {
        Some(home) => home,
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "cannot determine home directory"))
        }
    };
    agent_path_buf.push(".glop.agent");
    let agent_path = agent_path_buf.to_str().unwrap();
    let cleanup = cleanup::Cleanup::File(agent_path.to_string());
    {
        let mut agent_file = std::fs::OpenOptions::new().write(true)
            .mode(0o600)
            .create_new(true)
            .open(agent_path)?;
        write!(agent_file, "{}", addr)?;
    }
    Ok(cleanup)
}

pub fn read_agent_addr() -> std::io::Result<String> {
    let mut agent_path_buf = match std::env::home_dir() {
        Some(home) => home,
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                           "cannot determine home directory"))
        }
    };
    agent_path_buf.push(".glop.agent");
    let agent_path = agent_path_buf.to_str().unwrap();
    let mut agent_file = std::fs::File::open(agent_path)?;
    let mut result = String::new();
    agent_file.read_to_string(&mut result)?;
    Ok(result)
}
