extern crate serde_json;
extern crate tokio_core;
extern crate tokio_proto;

use std;
use std::error::Error as StdError;

use self::tokio_core::io::{Codec, EasyBuf, Framed, Io};

pub struct ClientCodec;
pub struct ServiceCodec;

#[derive(Serialize, Deserialize)]
pub enum Request {
    GetVar { key: String },
    SetVar { key: String, value: String },
    GetMsg { topic: String, key: String },
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    GetVar { key: String, value: String },
    SetVar { key: String, value: String },
    GetMsg {
        topic: String,
        key: String,
        value: String,
    },
}

impl Codec for ServiceCodec {
    type In = Request;
    type Out = Response;

    fn decode(&mut self, buf: &mut EasyBuf) -> std::io::Result<Option<Self::In>> {
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

impl Codec for ClientCodec {
    type Out = Request;
    type In = Response;

    fn decode(&mut self, buf: &mut EasyBuf) -> std::io::Result<Option<Self::In>> {
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
