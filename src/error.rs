extern crate clap;

use std;

use super::grammar;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AddrParse(std::net::AddrParseError),
    CLI(clap::Error),
    Env(std::env::VarError),
    IO(std::io::Error),
    Parse(grammar::ParseError),
    StringConversion(std::string::FromUtf8Error),
    InvalidArgument(String),
    BadResponse,
    Exec(i32, String),
    Acknowledge(String),
    UnsupportedAction,
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::CLI(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Error {
        Error::AddrParse(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Error {
        Error::Env(err)
    }
}

impl From<grammar::ParseError> for Error {
    fn from(err: grammar::ParseError) -> Error {
        Error::Parse(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::StringConversion(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::AddrParse(ref err) => err.fmt(f),
            Error::CLI(ref err) => err.fmt(f),
            Error::Env(ref err) => err.fmt(f),
            Error::IO(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
            Error::StringConversion(ref err) => err.fmt(f),
            Error::InvalidArgument(ref msg) => write!(f, "invalid argument: {}", msg),
            Error::BadResponse => write!(f, "bad response"),
            Error::Exec(code, ref stderr) => write!(f, "script exit code {}: {}", code, stderr),
            Error::Acknowledge(ref topic) => write!(f, "invalid acknowledge: {}", topic),
            Error::UnsupportedAction => write!(f, "unsupported action"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AddrParse(ref err) => err.description(),
            Error::CLI(ref err) => err.description(),
            Error::Env(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Parse(ref err) => err.description(),
            Error::StringConversion(ref err) => err.description(),
            Error::InvalidArgument(ref msg) => msg,
            Error::BadResponse => "bad response",
            Error::Exec(_, ref stderr) => stderr,
            Error::Acknowledge(ref topic) => topic,
            Error::UnsupportedAction => "unsupported action",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::AddrParse(ref err) => Some(err),
            Error::CLI(ref err) => Some(err),
            Error::Env(ref err) => Some(err),
            Error::IO(ref err) => Some(err),
            Error::Parse(ref err) => Some(err),
            Error::StringConversion(ref err) => Some(err),
            _ => None,
        }
    }
}
