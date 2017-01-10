use std::{thread, time};
use std::fs::File;
use std::process::exit;
use std::io;
use std::io::Read;

extern crate clap;
use clap::{Arg, ArgMatches, App, SubCommand};

extern crate glop;
use glop::grammar;
use glop::runtime;

type AppResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CLI(clap::Error),
    Parse(glop::grammar::ParseError),
    IO(io::Error),
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

impl From<glop::grammar::ParseError> for Error {
    fn from(err: glop::grammar::ParseError) -> Error {
        Error::Parse(err)
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
        Some("agent") => cmd_agent(&app_m),
        Some("get") => cmd_get(&app_m),
        Some("set") => cmd_set(&app_m),
        Some("send") => cmd_send(&app_m),
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
    let m_excs =
        glop.matches.iter().map(|m_ast| runtime::Match::new_from_ast(&m_ast)).collect::<Vec<_>>();
    loop {
        for m_exc in &m_excs {
            match st.eval(&m_exc) {
                Some(ref mut ctx) => {
                    match ctx.apply(&m_exc) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("error applying match: {}", e);
                        }
                    }
                }
                None => {}
            }
            thread::sleep(time::Duration::from_millis(200));
        }
    }
}

fn cmd_get<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    println!("TODO: get");
    Ok(())
}

fn cmd_set<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    println!("TODO: set");
    Ok(())
}

fn cmd_send<'a>(app_m: &ArgMatches<'a>) -> AppResult<()> {
    println!("TODO: send");
    Ok(())
}

// let g = grammar::glop(r#"
// match (message init) {
// set installed false;
// set initialized true;
// acknowledge init;
// }
//
// match (installed == false, initialized == true) {
// exec "install-things.bash";
// set installed true;
// }
// "#)
// .unwrap();
// println!("{}", g);
//
// let mut child = Command::new("cat")
// .stdin(Stdio::piped())
// .spawn()
// .expect("command failed");
// child.stdin.as_mut().unwrap().write_all("hello".as_bytes()).unwrap();
// let output = child.wait_with_output().unwrap();
// println!("{}", String::from_utf8(output.stdout).unwrap());
// }
//
