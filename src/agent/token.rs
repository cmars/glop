extern crate sodiumoxide;

use std;

use self::sodiumoxide::crypto::secretbox;

use super::*;

pub const TOKEN_NAME_LEN: usize = 32;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum Token {
    Admin { name: String, key: secretbox::Key },
    Peer {
        addr: std::net::SocketAddr,
        key: secretbox::Key,
    },
}

impl Token {
    pub fn id(&self) -> String {
        match self {
            &Token::Admin { ref name, key: _ } => name.to_string(),
            &Token::Peer { ref addr, key: _ } => format!("{}", addr),
        }
    }

    pub fn key(&self) -> secretbox::Key {
        match self {
            &Token::Admin { ref key, name: _ } => key.clone(),
            &Token::Peer { ref key, addr: _ } => key.clone(),
        }
    }
}

pub trait TokenStorage {
    fn add_token(&mut self, token: Token) -> Result<(), Error>;
    fn remove_token(&mut self, id: &str) -> Result<(), Error>;
    fn token(&self, id: &str) -> Result<Option<Token>, Error>;
}
