extern crate tokio_core;
extern crate tokio_proto;

use std::io;
use std::io::Write;
use std::str;

use self::tokio_core::io::{Codec, EasyBuf};

pub struct ServiceCodec;

pub enum Request {
    GetVar(String),
    SetVar(String, String),
    GetMsg(String, String),
}

pub enum Response {
    GetVar(String, String),
    SetVar(String, String),
    GetMsg(String, String, String),
}

impl Codec for ServiceCodec {
    type In = Request;
    type Out = Response;

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
                "getvar" => {
                    if fields.len() < 2 {
                        Err(io::Error::new(io::ErrorKind::Other, "getvar: missing key"))
                    } else {
                        Ok(Some(Request::GetVar(fields[1].to_string())))
                    }
                }
                "setvar" => {
                    if fields.len() < 3 {
                        Err(io::Error::new(io::ErrorKind::Other,
                                           "setvar: missing key and/or value"))
                    } else {
                        Ok(Some(Request::SetVar(fields[1].to_string(),
                                                fields[2..].join(" ").to_string())))
                    }
                }
                "getmsg" => {
                    if fields.len() < 3 {
                        Err(io::Error::new(io::ErrorKind::Other,
                                           "getmsg: missing topic and/or key"))
                    } else {
                        Ok(Some(Request::GetMsg(fields[1].to_string(), fields[2].to_string())))
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
            Response::GetVar(ref k, ref v) => {
                // TODO: use json or something
                buf.write_fmt(format_args!("getvar {} -> {}", k, v))?;
            }
            Response::SetVar(ref k, ref v) => {
                buf.write_fmt(format_args!("setvar {} {}", k, v))?;
            }
            Response::GetMsg(ref topic, ref k, ref v) => {
                buf.write_fmt(format_args!("getmsg {} {} -> {}", topic, k, v))?;
            }
        }
        buf.push(b'\n');
        Ok(())
    }
}
