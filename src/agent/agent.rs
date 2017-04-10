extern crate futures;
extern crate serde_json;

use std;
use std::collections::HashMap;
use std::os::unix::fs::OpenOptionsExt;

use self::futures::sync::mpsc;

use super::*;

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
            st.mut_storage().push_msg(Message::new("init", Obj::new()))?;
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

pub trait AgentStorage {
    type RuntimeStorage: runtime::Storage + Send;

    fn new_state(&self,
                 name: &str,
                 outbox: Box<runtime::Outbox + Send + 'static>)
                 -> Result<runtime::State<Self::RuntimeStorage>, Error>;
    fn add_agent(&mut self, name: String, glop: ast::Glop) -> Result<(), Error>;
    fn remove_agent(&mut self, name: &str) -> Result<(), Error>;
    fn agents(&self) -> Result<HashMap<String, ast::Glop>, Error>;
}

#[derive(Clone)]
pub struct MemAgentStorage {
    agents: HashMap<String, ast::Glop>,
}

impl MemAgentStorage {
    #[allow(dead_code)]
    pub fn new() -> MemAgentStorage {
        MemAgentStorage { agents: HashMap::new() }
    }
}

impl AgentStorage for MemAgentStorage {
    type RuntimeStorage = runtime::MemStorage;

    fn new_state(&self,
                 name: &str,
                 outbox: Box<runtime::Outbox + Send + 'static>)
                 -> Result<runtime::State<Self::RuntimeStorage>, Error> {
        Ok(runtime::State::new_outbox(name, Self::RuntimeStorage::new(), outbox))
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
pub struct DurableAgentStorage {
    path: String,
    agents_json_path: String,
}

impl DurableAgentStorage {
    pub fn new(path: &str) -> DurableAgentStorage {
        DurableAgentStorage {
            path: path.to_string(),
            agents_json_path: std::path::PathBuf::from(path)
                .join("agents.json")
                .to_str()
                .unwrap()
                .to_string(),
        }
    }

    fn load_agents(&self) -> Result<HashMap<String, ast::Glop>, Error> {
        if !std::path::PathBuf::from(&self.agents_json_path).exists() {
            return Ok(HashMap::new());
        }
        let agents_file = std::fs::OpenOptions::new().read(true)
            .open(&self.agents_json_path)?;
        let agents: HashMap<String, ast::Glop> =
            serde_json::from_reader(agents_file).map_err(to_ioerror)
                .map_err(Error::IO)?;
        Ok(agents)
    }

    fn save_agents(&self, agents: HashMap<String, ast::Glop>) -> Result<(), Error> {
        let mut agents_file = std::fs::OpenOptions::new().write(true)
            .mode(0o600)
            .create(true)
            .truncate(true)
            .open(&self.agents_json_path)?;
        serde_json::to_writer(&mut agents_file, &agents).map_err(to_ioerror)
            .map_err(Error::IO)?;
        Ok(())
    }
}

impl AgentStorage for DurableAgentStorage {
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
        let mut agents = self.load_agents()?;
        agents.insert(name, glop);
        self.save_agents(agents)?;
        Ok(())
    }

    fn remove_agent(&mut self, name: &str) -> Result<(), Error> {
        let mut agents = self.load_agents()?;
        agents.remove(name);
        self.save_agents(agents)?;
        Ok(())
    }

    fn agents(&self) -> Result<HashMap<String, ast::Glop>, Error> {
        let agents = self.load_agents()?;
        Ok(agents)
    }
}
