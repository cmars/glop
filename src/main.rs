extern crate clap;
extern crate futures;
extern crate tokio_core;

use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::net;
use std::process::exit;
use std::sync::mpsc;
use std::{thread, time};

use clap::{Arg, ArgMatches, App, SubCommand};
use futures::future::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

extern crate glop;
use glop::grammar;
use glop::runtime;

type AppResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AddrParse(net::AddrParseError),
    CLI(clap::Error),
    IO(io::Error),
    Parse(glop::grammar::ParseError),
    Env(env::VarError),
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

impl From<glop::grammar::ParseError> for Error {
    fn from(err: glop::grammar::ParseError) -> Error {
        Error::Parse(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AddrParse(ref err) => write!(f, "{}", err),
            Error::CLI(ref err) => write!(f, "{}", err),
            Error::Env(ref err) => write!(f, "{}", err),
            Error::IO(ref err) => write!(f, "{}", err),
            Error::Parse(ref err) => write!(f, "{}", err),
        }
    }
}

fn main() {
    let app = App::new("glop")
        .version("0")
        .author("Casey Marshall")
        .about("Glue Language for OPerations")
        .subcommand(SubCommand::with_name("agent")
            .about("run the interpreter agent")
            .arg(Arg::with_name("GLOPFILE").index(1).multiple(true).required(true)))
        .subcommand(SubCommand::with_name("set")
            .about("set variable in context")
            .arg(Arg::with_name("KEY").index(1).required(true))
            .arg(Arg::with_name("VALUE").index(2).required(true)))
        .subcommand(SubCommand::with_name("get")
            .about("get value of variable or message content")
            .arg(Arg::with_name("KEY").index(1).required(true)))
        .subcommand(SubCommand::with_name("send")
            .about("send a message")
            .arg(Arg::with_name("recipient").long("recipient").short("r").default_value("self"))
            .arg(Arg::with_name("topic").long("topic").short("t").required(true))
            .arg(Arg::with_name("CONTENTS").index(1).multiple(true).required(false)));
    let app_m = app.get_matches();
    let result = match app_m.subcommand_name() {
        Some("agent") => cmd_agent(app_m.subcommand_matches("agent").unwrap()),
        Some("get") => cmd_get(app_m.subcommand_matches("get").unwrap()),
        Some("set") => cmd_set(app_m.subcommand_matches("set").unwrap()),
        Some(subcmd) => {
            println!("unsupported command {}", subcmd);
            println!("{}", app_m.usage());
            Err(Error::CLI(clap::Error::with_description("unsupported command",
                                                         clap::ErrorKind::HelpDisplayed)))
        }
        None => {
            println!("{}", app_m.usage());
            Err(Error::CLI(clap::Error::with_description("missing subcommand",
                                                         clap::ErrorKind::HelpDisplayed)))
        }
    };
    match result {
        Ok(_) => exit(0),
        Err(_) => exit(1),
    };
}

fn read_file(path: &str) -> AppResult<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    f.read_to_string(&mut s).map_err(Error::IO)?;
    Ok(s)
}

fn cmd_agent<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let glop_file = app_m.value_of("GLOPFILE").unwrap();
    let glop_contents = try!(read_file(glop_file));
    let glop = grammar::glop(&glop_contents).map_err(Error::Parse)?;
    let mut st = runtime::State::new();
    st.push_msg("init", runtime::Msg::new());
    let m_excs =
        glop.matches.iter().map(|m_ast| runtime::Match::new_from_ast(&m_ast)).collect::<Vec<_>>();
    let (tx_result, rx_result) = mpsc::channel();
    thread::spawn(move || {
        loop {
            for m_exc in &m_excs {
                match st.eval(&m_exc) {
                    Some(ref mut ctx) => {
                        let result = ctx.apply(&m_exc);
                        tx_result.send(result).unwrap();
                    }
                    None => {}
                }
                thread::sleep(time::Duration::from_millis(200));
            }
        }
    });
    loop {
        match rx_result.recv().unwrap() {
            Ok(_) => println!("Ok"),
            Err(e) => println!("Err {}", e),
        }
    }
}

fn cmd_get<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let addr_str = env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let cmd = format!("get {}", app_m.value_of("KEY").unwrap());
    let mut core = Core::new()?;
    let handle = core.handle();
    let socket = TcpStream::connect(&addr, &handle);
    let request = socket.and_then(|mut sock| {
        sock.write_all(cmd.as_bytes())?;
        Ok(sock)
    });
    let response = request.and_then(|mut sock| {
        let mut s = String::new();
        sock.read_to_string(&mut s)?;
        Ok(s)
    });
    let value = match core.run(response) {
        Ok(data) => {
            let mut values = data.split(" -> ");
            match values.nth(1) {
                Some(v) => v.to_string(),
                None => {
                    return Err(Error::IO(io::Error::new(io::ErrorKind::Other,
                                                        "get: malformed response")));
                }
            }
        }
        Err(e) => {
            return Err(Error::IO(e));
        }
    };
    println!("{}", value);
    Ok(())
}

fn cmd_set<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let addr_str = env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let cmd = format!("set {} {}",
                      app_m.value_of("KEY").unwrap(),
                      app_m.value_of("VALUE").unwrap());
    let mut core = Core::new()?;
    let handle = core.handle();
    let socket = TcpStream::connect(&addr, &handle);
    let request = socket.and_then(|mut sock| {
        sock.write_all(cmd.as_bytes())?;
        Ok(sock)
    });
    let response = request.and_then(|mut sock| {
        let mut s = String::new();
        sock.read_to_string(&mut s)?;
        Ok(s)
    });
    match core.run(response) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(Error::IO(e));
        }
    }
}
