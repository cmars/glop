extern crate base64;
extern crate bytes;
extern crate futures;
extern crate sodiumoxide;
extern crate serde_json;
extern crate textnonce;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

use self::futures::future::Future;
use self::sodiumoxide::crypto::secretbox;
use self::tokio_core::reactor::Core;
use self::tokio_proto::TcpClient;
use self::tokio_service::Service;

use super::*;
use self::api::{Request, Response};
use self::token::{DurableTokenStorage, Token, TokenStorage};

pub struct ClientCodec;

impl tokio_io::codec::Decoder for ClientCodec {
    type Item = Response;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> std::io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and
            // return it in a Frame.
            let maybe_req: Result<Self::Item, serde_json::error::Error> =
                serde_json::from_slice(&line[..]);
            match maybe_req {
                Ok(req) => {
                    debug!("client decode: {:?}", req);
                    buf.take();
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
}

impl tokio_io::codec::Encoder for ClientCodec {
    type Item = Request;
    type Error = std::io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut bytes::BytesMut) -> std::io::Result<()> {
        match serde_json::to_vec(&msg) {
            Ok(json) => {
                debug!("client encode: {:?}", msg);
                buf.extend(&json[..]);
                Ok(())
            }
            Err(e) => {
                error!("client encode failed: {}", e);
                Err(std::io::Error::new(std::io::ErrorKind::Other, e.description()))
            }
        }?;
        buf.extend(&b"\n"[..]);
        Ok(())
    }
}

pub struct SecureClientCodec {
    id: String,
    codec: crypto::SecretBoxCodec<ClientCodec>,
}

impl SecureClientCodec {
    pub fn new(token: Token) -> SecureClientCodec {
        SecureClientCodec {
            id: token.id(),
            codec: crypto::SecretBoxCodec::new(ClientCodec, token.key()),
        }
    }
}

impl tokio_io::codec::Decoder for SecureClientCodec {
    type Item = <ClientCodec as tokio_io::codec::Decoder>::Item;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> std::io::Result<Option<Self::Item>> {
        self.codec.decode(buf)
    }
}

impl tokio_io::codec::Encoder for SecureClientCodec {
    type Item = <ClientCodec as tokio_io::codec::Encoder>::Item;
    type Error = std::io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut bytes::BytesMut) -> std::io::Result<()> {
        // Write prelude identifying the secret key.
        buf.extend(self.id.as_bytes());
        buf.extend(&b"\0"[..]);
        // Encrypt the message with the associated secret key.
        self.codec.encode(msg, buf)
    }
}

pub struct ClientProto {
    token: Token,
}

impl ClientProto {
    pub fn new(token: Token) -> ClientProto {
        ClientProto { token: token }
    }
}

impl<T: tokio_io::AsyncRead + tokio_io::AsyncWrite + 'static> tokio_proto::pipeline::ClientProto<T> for ClientProto {
    type Request = Request;
    type Response = Response;
    type Transport = tokio_io::codec::Framed<T, SecureClientCodec>;
    type BindTransport = Result<Self::Transport, std::io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(SecureClientCodec::new(self.token.clone())))
    }
}

pub struct Client {
    remotes: RemotesStorage,
    tokens: DurableTokenStorage,
}

impl Client {
    pub fn new(path: &str) -> Result<Client, Error> {
        std::fs::DirBuilder::new().recursive(true)
            .mode(0o700)
            .create(path)
            .map_err(Error::IO)?;
        Ok(Client {
            remotes: RemotesStorage::new(std::path::PathBuf::from(path)
                .join("remotes.json")
                .to_str()
                .unwrap()),
            tokens: DurableTokenStorage::new(std::path::PathBuf::from(path)
                .join("tokens.json")
                .to_str()
                .unwrap()),
        })
    }

    pub fn call(&self, name: &str, req: Request) -> Result<Response, Error> {
        let mut core = Core::new()?;
        let handle = core.handle();

        match self.remote(name) {
            Ok(Some((remote, token))) => {
                trace!("calling remote {} addr {} token id {}",
                       name,
                       remote.addr,
                       remote.token_id);
                let builder = TcpClient::new(ClientProto::new(token));
                let resp =
                    core.run(builder.connect(&remote.addr, &handle).and_then(|svc| svc.call(req)))
                        .map_err(Error::IO)?;
                Ok(resp)
            }
            Ok(None) => {
                Err(Error::InvalidArgument(format!("remote agent service {} not found", name)))
            }
            Err(e) => Err(e),
        }
    }

    pub fn add_remote_str(&mut self,
                          name: &str,
                          addr_str: &str,
                          id: &str,
                          encoded_key: &str)
                          -> Result<(Remote, Token), Error> {
        let addr = addr_str.parse().map_err(Error::AddrParse)?;
        let key_bytes = base64::decode(encoded_key).map_err(to_ioerror)?;
        let key = match secretbox::Key::from_slice(&key_bytes) {
            Some(k) => k,
            None => return Err(Error::InvalidArgument("invalid token length".to_string())),
        };
        self.add_remote(name, addr, id, key)
    }

    pub fn add_remote(&mut self,
                      name: &str,
                      addr: std::net::SocketAddr,
                      id: &str,
                      key: secretbox::Key)
                      -> Result<(Remote, Token), Error> {
        let token = Token::Admin {
            id: id.to_string(),
            key: key,
        };
        let remote = Remote {
            name: name.to_string(),
            token_id: id.to_string(),
            addr: addr,
        };
        self.tokens.add_token(token.clone())?;
        self.remotes.add_remote(remote.clone())?;
        Ok((remote, token))
    }

    pub fn remove_remote(&mut self, name: &str) -> Result<(), Error> {
        match self.remotes.remote(name) {
            Ok(Some(ref remote)) => {
                self.tokens.remove_token(&remote.token_id)?;
                self.remotes.remove_remote(name)?;
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn remotes(&self) -> Result<Vec<Remote>, Error> {
        let remotes = self.remotes.load_remotes()?;
        Ok(remotes.values().cloned().collect())
    }

    fn remote(&self, name: &str) -> Result<Option<(Remote, Token)>, Error> {
        match self.remotes.remote(name) {
            Ok(Some(ref remote)) => {
                match self.tokens.token(&remote.token_id) {
                    Ok(Some(ref token)) => Ok(Some((remote.clone(), token.clone()))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Remote {
    pub name: String,
    pub token_id: String,
    pub addr: std::net::SocketAddr,
}

pub struct RemotesStorage {
    path: String,
}

impl RemotesStorage {
    pub fn new(path: &str) -> RemotesStorage {
        RemotesStorage { path: path.to_string() }
    }

    fn load_remotes(&self) -> Result<HashMap<String, Remote>, Error> {
        if !std::path::PathBuf::from(&self.path).exists() {
            return Ok(HashMap::new());
        }
        let remotes_file = std::fs::OpenOptions::new().read(true)
            .open(&self.path)?;
        let remotes: HashMap<String, Remote> =
            serde_json::from_reader(remotes_file).map_err(to_ioerror)
                .map_err(Error::IO)?;
        Ok(remotes)
    }

    fn save_remotes(&self, remotes: HashMap<String, Remote>) -> Result<(), Error> {
        let mut remotes_file = std::fs::OpenOptions::new().write(true)
            .mode(0o600)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        serde_json::to_writer(&mut remotes_file, &remotes).map_err(to_ioerror)
            .map_err(Error::IO)?;
        Ok(())
    }

    fn add_remote(&mut self, remote: Remote) -> Result<(), Error> {
        let mut remotes = self.load_remotes()?;
        remotes.insert(remote.name.to_string(), remote);
        self.save_remotes(remotes)?;
        Ok(())
    }

    fn remove_remote(&mut self, name: &str) -> Result<(), Error> {
        let mut remotes = self.load_remotes()?;
        remotes.remove(name);
        self.save_remotes(remotes)?;
        Ok(())
    }

    fn remote(&self, name: &str) -> Result<Option<Remote>, Error> {
        let remotes = self.load_remotes()?;
        Ok(match remotes.get(name) {
            Some(remote) => Some(remote.clone()),
            None => None,
        })
    }
}
