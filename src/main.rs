extern crate clap;
extern crate futures;
extern crate tokio_core;

use std::fs::File;
use std::io::{Read, Write};
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
use glop::agent;
use glop::signal_fix;
use glop::error::Error;

type AppResult<T> = Result<T, Error>;

fn main() {
    let _lock = signal_fix::lock();
    let app = App::new("glop")
        .version("0")
        .author("Casey Marshall")
        .about("Glue Language for OPerations")
        .subcommand(SubCommand::with_name("agent").about("run the agent server"))
        .subcommand(SubCommand::with_name("run")
            .about("run the interpreter")
            .arg(Arg::with_name("GLOPFILE").index(1).multiple(true).required(true)))
        .subcommand(SubCommand::with_name("getvar")
            .about("get variable in context")
            .arg(Arg::with_name("KEY").index(1).required(true)))
        .subcommand(SubCommand::with_name("setvar")
            .about("set variable in context")
            .arg(Arg::with_name("KEY").index(1).required(true))
            .arg(Arg::with_name("VALUE").index(2).required(true)))
        .subcommand(SubCommand::with_name("getmsg")
            .about("get message in context")
            .arg(Arg::with_name("TOPIC").index(1).required(true))
            .arg(Arg::with_name("KEY").index(2).required(true)))
        .subcommand(SubCommand::with_name("send")
            .about("send a message")
            .arg(Arg::with_name("recipient").long("recipient").short("r").default_value("self"))
            .arg(Arg::with_name("topic").long("topic").short("t").required(true))
            .arg(Arg::with_name("CONTENTS").index(1).multiple(true).required(false)));
    let app_m = app.get_matches();
    let result = match app_m.subcommand_name() {
        Some("agent") => cmd_agent(app_m.subcommand_matches("agent").unwrap()),
        Some("run") => cmd_run(app_m.subcommand_matches("run").unwrap()),
        Some("getvar") => cmd_getvar(app_m.subcommand_matches("getvar").unwrap()),
        Some("setvar") => cmd_setvar(app_m.subcommand_matches("setvar").unwrap()),
        Some("getmsg") => cmd_getmsg(app_m.subcommand_matches("getmsg").unwrap()),
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

fn cmd_agent<'a>(_app_m: &ArgMatches<'a>) -> AppResult<()> {
    agent::run_server().map_err(Error::IO)?;
    Ok(())
}

fn cmd_run<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
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
                let result = match st.eval(&m_exc) {
                    Some(ref mut ctx) => Some(ctx.apply(&m_exc).unwrap()),
                    None => None,
                };
                match result {
                    Some(actions) => tx_result.send(st.commit(&actions)).unwrap(),
                    None => {}
                }
                thread::sleep(time::Duration::from_millis(200));
            }
        }
    });
    loop {
        match rx_result.recv().unwrap() {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
                exit(1);
            }
        }
    }
}

fn cmd_getvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let addr_str = std::env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let cmd = format!("getvar {}", app_m.value_of("KEY").unwrap());
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
                    return Err(Error::IO(std::io::Error::new(std::io::ErrorKind::Other,
                                                             "getvar: malformed response")));
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

fn cmd_setvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let addr_str = std::env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let cmd = format!("setvar {} {}",
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

fn cmd_getmsg<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let addr_str = std::env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let cmd = format!("getmsg {} {}",
                      app_m.value_of("TOPIC").unwrap(),
                      app_m.value_of("KEY").unwrap());
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
                    return Err(Error::IO(std::io::Error::new(std::io::ErrorKind::Other,
                                                             "getmsg: malformed response")));
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
