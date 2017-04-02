extern crate sodiumoxide;
extern crate serde_json;

use std;
use std::collections::HashMap;
use std::os::unix::fs::OpenOptionsExt;

use self::sodiumoxide::crypto::secretbox;

use super::*;

pub const TOKEN_NAME_LEN: usize = 32;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum Token {
    Admin { id: String, key: secretbox::Key },
    Peer {
        addr: std::net::SocketAddr,
        key: secretbox::Key,
    },
}

impl Token {
    pub fn id(&self) -> String {
        match self {
            &Token::Admin { ref id, key: _ } => id.to_string(),
            &Token::Peer { ref addr, key: _ } => format!("{}", addr),
        }
    }

    pub fn key(&self) -> secretbox::Key {
        match self {
            &Token::Admin { ref key, id: _ } => key.clone(),
            &Token::Peer { ref key, addr: _ } => key.clone(),
        }
    }
}

pub trait TokenStorage {
    fn add_token(&mut self, token: Token) -> Result<(), Error>;
    fn remove_token(&mut self, id: &str) -> Result<(), Error>;
    fn token(&self, id: &str) -> Result<Option<Token>, Error>;
}

#[derive(Clone)]
pub struct MemTokenStorage {
    tokens: HashMap<String, Token>,
}

impl MemTokenStorage {
    #[allow(dead_code)]
    pub fn new() -> MemTokenStorage {
        MemTokenStorage { tokens: HashMap::new() }
    }
}

impl TokenStorage for MemTokenStorage {
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
pub struct DurableTokenStorage {
    path: String,
}

impl DurableTokenStorage {
    pub fn new(path: &str) -> DurableTokenStorage {
        DurableTokenStorage { path: path.to_string() }
    }

    fn load_tokens(&self) -> Result<HashMap<String, Token>, Error> {
        if !std::path::PathBuf::from(&self.path).exists() {
            return Ok(HashMap::new());
        }
        let tokens_file = std::fs::OpenOptions::new().read(true)
            .open(&self.path)?;
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
            .open(&self.path)?;
        serde_json::to_writer(&mut tokens_file, &tokens).map_err(to_ioerror)
            .map_err(Error::IO)?;
        Ok(())
    }
}

impl TokenStorage for DurableTokenStorage {
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
