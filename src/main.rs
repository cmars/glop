extern crate clap;
extern crate futures;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{thread, time};

use clap::{Arg, ArgMatches, App, SubCommand};
use futures::future::Future;
use tokio_core::reactor::Core;
use tokio_proto::TcpClient;
use tokio_service::Service;

extern crate glop;
use glop::grammar;
use glop::runtime;
use glop::agent;
use glop::signal_fix;
use glop::script;
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
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
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
    loop {
        for m_exc in &m_excs {
            let result = match st.eval(&m_exc) {
                Some(ref mut ctx) => {
                    match ctx.apply(&m_exc) {
                        Ok(result) => Some(result),
                        Err(e) => {
                            println!("{}", e);
                            None
                        }
                    }
                }
                None => None,
            };
            match result {
                Some(actions) => {
                    match st.commit(&actions) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                }
                None => {}
            }
            thread::sleep(time::Duration::from_millis(200));
        }
    }
}

fn cmd_getvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let core = tokio_core::reactor::Core::new().map_err(Error::IO)?;
    let handle = core.handle();
    let addr_str = std::env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = script::Request::GetVar { key: app_m.value_of("KEY").unwrap().to_string() };
    let builder = TcpClient::new(script::ClientProto);
    let resp = builder.connect(&addr, &handle)
        .and_then(|svc| svc.call(req))
        .wait()
        .map_err(Error::IO)?;
    match resp {
        script::Response::GetVar { key: _, ref value } => {
            println!("{}", value);
            Ok(())
        }
        _ => Err(Error::BadResponse),
    }
}

fn cmd_setvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = script::Request::SetVar {
        key: app_m.value_of("KEY").unwrap().to_string(),
        value: app_m.value_of("VALUE").unwrap().to_string(),
    };
    let builder = TcpClient::new(script::ClientProto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        script::Response::SetVar { key: _, value: _ } => Ok(()),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_getmsg<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = script::Request::GetMsg {
        topic: app_m.value_of("TOPIC").unwrap().to_string(),
        key: app_m.value_of("KEY").unwrap().to_string(),
    };
    let builder = TcpClient::new(script::ClientProto);
    let resp =
        builder.connect(&addr, &handle).and_then(|svc| svc.call(req)).wait().map_err(Error::IO)?;
    match resp {
        script::Response::GetMsg { topic: _, key: _, ref value } => {
            println!("{}", value);
            Ok(())
        }
        _ => Err(Error::BadResponse),
    }
}
