extern crate clap;

use std;
use std::env;
use std::fmt;
use std::io;
use std::net;
use std::string;

use super::grammar;

#[derive(Debug)]
pub enum Error {
    AddrParse(net::AddrParseError),
    CLI(clap::Error),
    Env(env::VarError),
    IO(io::Error),
    Parse(grammar::ParseError),
    StringConversion(string::FromUtf8Error),
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::CLI(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::AddrParse(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::Env(err)
    }
}

impl From<grammar::ParseError> for Error {
    fn from(err: grammar::ParseError) -> Error {
        Error::Parse(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringConversion(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AddrParse(ref err) => err.fmt(f),
            Error::CLI(ref err) => err.fmt(f),
            Error::Env(ref err) => err.fmt(f),
            Error::IO(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
            Error::StringConversion(ref err) => err.fmt(f),
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
        }
    }
}
