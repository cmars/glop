#[macro_use]
extern crate log;

extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::{thread, time};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process::exit;

use clap::{Arg, ArgMatches, App, SubCommand};
use futures::future::Future;
use tokio_core::reactor::Core;
use tokio_proto::TcpClient;
use tokio_service::Service;

extern crate glop;
use glop::agent;
use glop::error::Error;
use glop::grammar;
use glop::runtime;
use glop::runtime::Storage;
use glop::signal_fix;
use glop::value;

type AppResult<T> = Result<T, Error>;

fn main() {
    let _ = env_logger::init();
    let _lock = signal_fix::lock();
    let app = App::new("glop")
        .version("0")
        .author("Casey Marshall")
        .about("Glue Language for OPerations")
        .subcommand(SubCommand::with_name("server").about("run the agent server"))
        .subcommand(SubCommand::with_name("run")
            .about("run the interpreter")
            .arg(Arg::with_name("GLOPFILE").index(1).multiple(true).required(true)))
        .subcommand(SubCommand::with_name("var")
            .about("access variables from glop runtime")
            .subcommand(SubCommand::with_name("get")
                .about("display value of variable")
                .arg(Arg::with_name("KEY").index(1).required(true)))
            .subcommand(SubCommand::with_name("set")
                .about("set value of variable")
                .arg(Arg::with_name("KEY").index(1).required(true))
                .arg(Arg::with_name("VALUE").index(2).required(true)))
            .subcommand(SubCommand::with_name("unset")
                .about("unset variable")
                .arg(Arg::with_name("KEY").index(1).required(true))))
        .subcommand(SubCommand::with_name("msg")
            .about("access messages from glop runtime")
            .subcommand(SubCommand::with_name("get")
                .about("get value of message")
                .arg(Arg::with_name("TOPIC").index(1).required(true))
                .arg(Arg::with_name("KEY").index(2).required(true)))
            .subcommand(SubCommand::with_name("send")
                .about("send a message to an agent")
                .arg(Arg::with_name("ROLE").short("r").long("role").takes_value(true))
                .arg(Arg::with_name("NAME").index(1).required(true))
                .arg(Arg::with_name("TOPIC").index(2).required(true))
                .arg(Arg::with_name("CONTENTS").index(3).multiple(true).required(false)))
            .subcommand(SubCommand::with_name("reply")
                .about("reply to the sender of a topic")
                .arg(Arg::with_name("SRC_TOPIC").index(1).required(true))
                .arg(Arg::with_name("TOPIC").index(2).required(true))
                .arg(Arg::with_name("CONTENTS").index(3).multiple(true).required(false))))
        .subcommand(SubCommand::with_name("agent")
            .about("manage the agent server")
            .subcommand(SubCommand::with_name("add")
                .about("add an agent")
                .arg(Arg::with_name("NAME").index(1).required(true))
                .arg(Arg::with_name("SOURCE").index(2).required(true)))
            .subcommand(SubCommand::with_name("remove")
                .about("remove an agent")
                .arg(Arg::with_name("NAME").index(1).required(true)))
            .subcommand(SubCommand::with_name("list").about("list agents"))
            .subcommand(SubCommand::with_name("introduce")
                .about("introduce agents")
                .arg(Arg::with_name("NAME:ROLE").index(1).multiple(true).required(true)))
            .subcommand(SubCommand::with_name("send")
                .about("send a message to an agent")
                .arg(Arg::with_name("SOURCE").short("s").long("src").takes_value(true))
                .arg(Arg::with_name("ROLE").short("r").long("role").takes_value(true))
                .arg(Arg::with_name("NAME").index(1).required(true))
                .arg(Arg::with_name("TOPIC").index(2).required(true))
                .arg(Arg::with_name("CONTENTS").index(3).multiple(true).required(false))));
    let app_m = app.get_matches();
    let result =
        match app_m.subcommand_name() {
            Some("server") => cmd_server(app_m.subcommand_matches("server").unwrap()),
            Some("run") => cmd_run(app_m.subcommand_matches("run").unwrap()),
            Some("var") => {
                let sub_m = app_m.subcommand_matches("var").unwrap();
                match sub_m.subcommand_name() {
                    Some("get") => cmd_getvar(sub_m.subcommand_matches("get").unwrap()),
                    Some("set") => cmd_setvar(sub_m.subcommand_matches("set").unwrap()),
                    Some("unset") => cmd_unsetvar(sub_m.subcommand_matches("unset").unwrap()),
                    Some(subcmd) => {
                        error!("unsupported command {}", subcmd);
                        Err(Error::CLI(clap::Error::with_description("unsupported command",
                                                             clap::ErrorKind::HelpDisplayed)))
                    }
                    None => Err(Error::CLI(clap::Error::with_description("missing subcommand",
                                                         clap::ErrorKind::HelpDisplayed))),
                }
            }
            Some("msg") => {
                let sub_m = app_m.subcommand_matches("msg").unwrap();
                match sub_m.subcommand_name() {
                    Some("get") => cmd_getmsg(sub_m.subcommand_matches("get").unwrap()),
                    Some("send") => cmd_send_script(sub_m.subcommand_matches("send").unwrap()),
                    Some("reply") => cmd_reply_script(sub_m.subcommand_matches("reply").unwrap()),
                    Some(subcmd) => {
                        error!("unsupported command {}", subcmd);
                        Err(Error::CLI(clap::Error::with_description("unsupported command",
                                                             clap::ErrorKind::HelpDisplayed)))
                    }
                    None => Err(Error::CLI(clap::Error::with_description("missing subcommand",
                                                         clap::ErrorKind::HelpDisplayed))),
                }
            }
            Some("agent") => {
                let sub_m = app_m.subcommand_matches("agent").unwrap();
                match sub_m.subcommand_name() {
                    Some("add") => cmd_add(sub_m.subcommand_matches("add").unwrap()),
                    Some("remove") => cmd_remove(sub_m.subcommand_matches("remove").unwrap()),
                    Some("list") => cmd_list(sub_m.subcommand_matches("list").unwrap()),
                    Some("send") => cmd_send_agent(sub_m.subcommand_matches("send").unwrap()),
                    Some("introduce") => {
                        cmd_introduce(sub_m.subcommand_matches("introduce").unwrap())
                    }
                    Some(subcmd) => {
                        error!("unsupported command {}", subcmd);
                        Err(Error::CLI(clap::Error::with_description("unsupported command",
                                                             clap::ErrorKind::HelpDisplayed)))
                    }
                    None => Err(Error::CLI(clap::Error::with_description("missing subcommand",
                                                         clap::ErrorKind::HelpDisplayed))),
                }
            }
            Some(subcmd) => {
                error!("unsupported command {}", subcmd);
                Err(Error::CLI(clap::Error::with_description("unsupported command",
                                                             clap::ErrorKind::HelpDisplayed)))
            }
            None => {
                Err(Error::CLI(clap::Error::with_description("missing subcommand",
                                                             clap::ErrorKind::HelpDisplayed)))
            }
        };
    match result {
        Ok(_) => exit(0),
        Err(Error::CLI(e)) => {
            error!("{}", e);
            error!("{}", app_m.usage());
            exit(64)  // EX_USAGE
        }
        Err(e) => {
            error!("{}", e);
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

fn cmd_server<'a>(_app_m: &ArgMatches<'a>) -> AppResult<()> {
    agent::run_server()?;
    Ok(())
}

fn cmd_run<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let glop_file = app_m.value_of("GLOPFILE").unwrap();
    let glop_contents = try!(read_file(glop_file));
    let glop = grammar::glop(&glop_contents).map_err(Error::Parse)?;
    let mut st = runtime::State::new("main", runtime::MemStorage::new());
    st.mut_storage()
        .push_msg(value::Message {
            src: "".to_string(),
            src_role: None,
            dst: "main".to_string(),
            topic: "init".to_string(),
            contents: value::Obj::new(),
        })?;
    let m_excs =
        glop.matches.iter().map(|m_ast| runtime::Match::new_from_ast(&m_ast)).collect::<Vec<_>>();
    loop {
        for m_exc in &m_excs {
            let mut txn = match st.eval(m_exc.clone()) {
                Ok(Some(txn)) => txn,
                Ok(None) => continue,
                Err(e) => {
                    error!("eval failed: {}", e);
                    continue;
                }
            };
            match st.commit(&mut txn) {
                Ok(_seq) => (),
                Err(e) => error!("commit failed: {}", e),
            };
            thread::sleep(time::Duration::from_millis(200));
        }
    }
}

fn cmd_getvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = tokio_core::reactor::Core::new().map_err(Error::IO)?;
    let handle = core.handle();
    let addr_str = std::env::var("GLOP_SCRIPT_ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = runtime::ScriptRequest::GetVar { key: app_m.value_of("KEY").unwrap().to_string() };
    let proto = runtime::ScriptClientProto::new_from_env()?;
    let builder = TcpClient::new(proto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        runtime::ScriptResponse::GetVar { key: _, ref value } => {
            println!("{}", value);
            Ok(())
        }
        _ => Err(Error::BadResponse),
    }
}

fn cmd_setvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("GLOP_SCRIPT_ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = runtime::ScriptRequest::SetVar {
        key: app_m.value_of("KEY").unwrap().to_string(),
        value: app_m.value_of("VALUE").unwrap().to_string(),
    };
    let proto = runtime::ScriptClientProto::new_from_env()?;
    let builder = TcpClient::new(proto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        runtime::ScriptResponse::SetVar { key: _, value: _ } => Ok(()),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_unsetvar<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("GLOP_SCRIPT_ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = runtime::ScriptRequest::UnsetVar { key: app_m.value_of("KEY").unwrap().to_string() };
    let proto = runtime::ScriptClientProto::new_from_env()?;
    let builder = TcpClient::new(proto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        runtime::ScriptResponse::UnsetVar { key: _ } => Ok(()),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_getmsg<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("GLOP_SCRIPT_ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = runtime::ScriptRequest::GetMsg {
        topic: app_m.value_of("TOPIC").unwrap().to_string(),
        key: app_m.value_of("KEY").unwrap().to_string(),
    };
    let proto = runtime::ScriptClientProto::new_from_env()?;
    let builder = TcpClient::new(proto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        runtime::ScriptResponse::GetMsg { topic: _, key: _, ref value } => {
            println!("{}", value);
            Ok(())
        }
        _ => Err(Error::BadResponse),
    }
}

fn cmd_add<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = agent::read_agent_addr().map_err(Error::IO)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = agent::Request::Add {
        source: app_m.value_of("SOURCE").unwrap().to_string(),
        name: app_m.value_of("NAME").unwrap().to_string(),
    };
    let builder = TcpClient::new(agent::ClientProto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        agent::Response::Add => Ok(()),
        agent::Response::Error(msg) => Err(Error::ErrorResponse(msg)),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_remove<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = agent::read_agent_addr().map_err(Error::IO)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = agent::Request::Remove { name: app_m.value_of("NAME").unwrap().to_string() };
    let builder = TcpClient::new(agent::ClientProto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        agent::Response::Remove => Ok(()),
        agent::Response::Error(msg) => Err(Error::ErrorResponse(msg)),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_list<'a>(_app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = agent::read_agent_addr().map_err(Error::IO)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let req = agent::Request::List;
    let builder = TcpClient::new(agent::ClientProto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        agent::Response::List { ref names } => {
            for name in names {
                println!("{}", name);
            }
            Ok(())
        }
        agent::Response::Error(msg) => Err(Error::ErrorResponse(msg)),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_send_agent<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = agent::read_agent_addr().map_err(Error::IO)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let contents = kv_map(app_m.values_of("CONTENTS"));
    let req = agent::Request::SendTo(value::Message {
        src: if let Some(ref src) = app_m.value_of("SOURCE") {
            src.to_string()
        } else {
            "user".to_string()
        },
        src_role: if let Some(ref role) = app_m.value_of("ROLE") {
            Some(role.to_string())
        } else {
            None
        },
        dst: app_m.value_of("NAME").unwrap().to_string(),
        topic: app_m.value_of("TOPIC").unwrap().to_string(),
        contents: value::Value::from_flat_map(contents),
    });
    let builder = TcpClient::new(agent::ClientProto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        agent::Response::SendTo { src: _, dst: _ } => Ok(()),
        agent::Response::Error(msg) => Err(Error::ErrorResponse(msg)),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_introduce<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = agent::read_agent_addr().map_err(Error::IO)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let name_roles = name_roles(app_m.values_of("NAME:ROLE"));
    let req = agent::Request::Introduce(name_roles);
    let builder = TcpClient::new(agent::ClientProto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        agent::Response::Introduce(ref results) => {
            for result in results {
                if let &agent::Response::Error(ref msg) = result {
                    return Err(Error::ErrorResponse(msg.to_string()));
                }
            }
            Ok(())
        }
        agent::Response::Error(msg) => Err(Error::ErrorResponse(msg)),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_send_script<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("GLOP_SCRIPT_ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let contents = kv_map(app_m.values_of("CONTENTS"));
    let req = runtime::ScriptRequest::SendMsg {
        dst: app_m.value_of("NAME").unwrap().to_string(),
        topic: app_m.value_of("TOPIC").unwrap().to_string(),
        contents: value::Value::from_flat_map(contents),
    };
    let proto = runtime::ScriptClientProto::new_from_env()?;
    let builder = TcpClient::new(proto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        runtime::ScriptResponse::SendMsg { dst: _, topic: _ } => Ok(()),
        _ => Err(Error::BadResponse),
    }
}

fn cmd_reply_script<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let addr_str = std::env::var("GLOP_SCRIPT_ADDR").map_err(Error::Env)?;
    let addr = addr_str.parse().map_err(Error::AddrParse)?;
    let contents = kv_map(app_m.values_of("CONTENTS"));
    let req = runtime::ScriptRequest::ReplyMsg {
        src_topic: app_m.value_of("SRC_TOPIC").unwrap().to_string(),
        topic: app_m.value_of("TOPIC").unwrap().to_string(),
        contents: value::Value::from_flat_map(contents),
    };
    let proto = runtime::ScriptClientProto::new_from_env()?;
    let builder = TcpClient::new(proto);
    let resp = core.run(builder.connect(&addr, &handle).and_then(|svc| svc.call(req)))
        .map_err(Error::IO)?;
    match resp {
        runtime::ScriptResponse::SendMsg { dst: _, topic: _ } => Ok(()),
        _ => Err(Error::BadResponse),
    }
}

fn kv_map<'a>(maybe_args: Option<clap::Values<'a>>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Some(values) = maybe_args {
        for kvpair in values {
            let kvs = kvpair.split("=").collect::<Vec<_>>();
            result.insert(kvs[0].to_string(), kvs[1].to_string());
        }
    }
    result
}

fn name_roles<'a>(maybe_args: Option<clap::Values<'a>>) -> Vec<agent::AgentRole> {
    let mut result = vec![];
    if let Some(values) = maybe_args {
        for nrpair in values {
            let nrs = nrpair.split(":").collect::<Vec<_>>();
            result.push(agent::AgentRole {
                name: nrs[0].to_string(),
                role: nrs[1].to_string(),
            });
        }
    }
    result
}
