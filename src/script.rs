extern crate tokio_core;
extern crate tokio_proto;

use std::io;
use std::io::Write;
use std::str;

use self::tokio_core::io::{Codec, EasyBuf, Framed, Io};
use self::tokio_proto::pipeline::ServerProto;

pub struct ServiceCodec;

pub enum ScriptRequest {
    Get(String),
    Set(String, String),
}

pub enum ScriptResponse {
    Get(String, String),
    Set(String, String),
}

impl Codec for ServiceCodec {
    type In = ScriptRequest;
    type Out = ScriptResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            let fields = str::from_utf8(&line.as_ref()).unwrap().split(" ").collect::<Vec<&str>>();
            if fields.len() < 1 {
                return Err(io::Error::new(io::ErrorKind::Other, "missing command"));
            }

            return match fields[0] {
                "get" => {
                    if fields.len() < 2 {
                        Err(io::Error::new(io::ErrorKind::Other, "get: missing key"))
                    } else {
                        Ok(Some(ScriptRequest::Get(fields[1].to_string())))
                    }
                }
                "set" => {
                    if fields.len() < 3 {
                        Err(io::Error::new(io::ErrorKind::Other, "set: missing key and/or value"))
                    } else {
                        Ok(Some(ScriptRequest::Set(fields[1].to_string(),
                                                   fields[2..].join(" ").to_string())))
                    }
                }
                _ => {
                    Err(io::Error::new(io::ErrorKind::Other,
                                       format!("unknown command: {}", fields[0])))
                }
            };
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        match msg {
            ScriptResponse::Get(ref k, ref v) => {
                // TODO: use json or something
                buf.write_fmt(format_args!("get {} -> {}", k, v))?;
            }
            ScriptResponse::Set(ref k, ref v) => {
                buf.write_fmt(format_args!("set {} {}", k, v))?;
            }
        }
        buf.push(b'\n');
        Ok(())
    }
}

pub struct ServiceProto;

impl<T: Io + 'static> ServerProto<T> for ServiceProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = ScriptRequest;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = ScriptResponse;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, ServiceCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ServiceCodec))
    }
}
