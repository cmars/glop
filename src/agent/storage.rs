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
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

use super::*;
use self::token::{Token, TokenStorage};

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
pub struct MemStorage {
    agents: HashMap<String, ast::Glop>,
    tokens: HashMap<String, Token>,
}

impl MemStorage {
    #[allow(dead_code)]
    pub fn new() -> MemStorage {
        MemStorage {
            agents: HashMap::new(),
            tokens: HashMap::new(),
        }
    }
}

impl AgentStorage for MemStorage {
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

impl TokenStorage for MemStorage {
    fn add_token(&mut self, token: Token) -> Result<(), Error> {
        self.tokens.insert(token.id(), token);
        Ok(())
    }

    fn remove_token(&mut self, id: &str) -> Result<(), Error> {
        self.tokens.remove(id);
        Ok(())
    }

    fn token(&self, id: &str) -> Result<Option<Token>, Error> {
        Ok(match self.tokens.get(id) {
            Some(token) => Some(token.clone()),
            None => None,
        })
    }
}

#[derive(Clone)]
pub struct DurableStorage {
    path: String,
    agents_path: String,
    tokens_path: String,
}

impl DurableStorage {
    pub fn new(path: &str) -> Result<DurableStorage, Error> {
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
            tokens_path: std::path::PathBuf::from(path)
                .join("tokens.json")
                .to_str()
                .unwrap()
                .to_string(),
        })
    }

    fn load_agents(&self) -> Result<HashMap<String, ast::Glop>, Error> {
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

    fn save_agents(&self, agents: HashMap<String, ast::Glop>) -> Result<(), Error> {
        let mut agents_file = std::fs::OpenOptions::new().write(true)
            .mode(0o600)
            .create(true)
            .truncate(true)
            .open(&self.agents_path)?;
        serde_json::to_writer(&mut agents_file, &agents).map_err(to_ioerror)
            .map_err(Error::IO)?;
        Ok(())
    }

    fn load_tokens(&self) -> Result<HashMap<String, Token>, Error> {
        if !std::path::PathBuf::from(&self.tokens_path).exists() {
            return Ok(HashMap::new());
        }
        let tokens_file = std::fs::OpenOptions::new().read(true)
            .open(&self.tokens_path)?;
        let tokens: HashMap<String, Token> =
            serde_json::from_reader(tokens_file).map_err(to_ioerror)
                .map_err(Error::IO)?;
        Ok(tokens)
    }

    fn save_tokens(&self, tokens: HashMap<String, Token>) -> Result<(), Error> {
        let mut tokens_file = std::fs::OpenOptions::new().write(true)
            .mode(0o600)
            .create(true)
            .truncate(true)
            .open(&self.tokens_path)?;
        serde_json::to_writer(&mut tokens_file, &tokens).map_err(to_ioerror)
            .map_err(Error::IO)?;
        Ok(())
    }
}

impl AgentStorage for DurableStorage {
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

impl TokenStorage for DurableStorage {
    fn add_token(&mut self, token: Token) -> Result<(), Error> {
        let mut tokens = self.load_tokens()?;
        tokens.insert(token.id(), token);
        self.save_tokens(tokens)?;
        Ok(())
    }

    fn remove_token(&mut self, id: &str) -> Result<(), Error> {
        let mut tokens = self.load_tokens()?;
        tokens.remove(id);
        self.save_tokens(tokens)?;
        Ok(())
    }

    fn token(&self, id: &str) -> Result<Option<Token>, Error> {
        let tokens = self.load_tokens()?;
        Ok(match tokens.get(id) {
            Some(token) => Some(token.clone()),
            None => None,
        })
    }
}
