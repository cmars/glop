#![cfg(unix)]
#![cfg(test)]

extern crate futures;
extern crate libc;
extern crate tokio_core;
extern crate tokio_signal;

use std::sync::mpsc::channel;
use std::sync::{Once, ONCE_INIT, Mutex, MutexGuard};
use std::thread;

use self::tokio_core::reactor::Core;
use self::tokio_signal::unix::Signal;

use super::ast;
use super::grammar;
use super::runtime;
use super::runtime::Stateful;
use super::value::{Identifier, Value};

const SIMPLE_SCRIPT_OK: &'static str = r###"
match (message init) {
    script #!/bin/bash
set -ex
echo "hello world"
!#
}
"###;
const SIMPLE_SCRIPT_ERR: &'static str = r###"
match (message init) {
    script #!/bin/bash
>&2 echo "crash and burn"
exit 1
!#
}
"###;
const ENV_CHECK_SCRIPT: &'static str = r###"
match (message test) {
    set foo bar;
    script #!/bin/bash
env
[ "${test__content}" == "hello world" ]
!#
    acknowledge test;
}
"###;
const HELLO_SCRIPT_SERVER: &'static str = r###"
match (message init) {
    set foo bar;
    script #!/bin/bash
set -e
[ -n "$ADDR" ]
PORT=$(echo ${ADDR} | sed 's/.*://')

# glop get foo
FOO=$(nc 127.0.0.1 ${PORT} <<EOF
get foo
EOF)
[ "$FOO" = "get foo -> bar" ]
FOO=$(echo $FOO | awk '{print $4}')

# glop set foo hello
nc 127.0.0.1 ${PORT} <<EOF
set foo hello-${FOO}
EOF

!#
}
"###;

static INIT: Once = ONCE_INIT;
static mut LOCK: *mut Mutex<()> = 0 as *mut _;

fn lock() -> MutexGuard<'static, ()> {
    unsafe {
        INIT.call_once(|| {
            LOCK = Box::into_raw(Box::new(Mutex::new(())));
            let (tx, rx) = channel();
            thread::spawn(move || {
                let mut lp = Core::new().unwrap();
                let handle = lp.handle();
                let _signal = lp.run(Signal::new(libc::SIGALRM, &handle)).unwrap();
                tx.send(()).unwrap();
                drop(lp.run(futures::empty::<(), ()>()));
            });
            rx.recv().unwrap();
        });
        (*LOCK).lock().unwrap()
    }
}

fn parse_one_match(s: &str) -> ast::Match {
    let mut g = grammar::glop(s).unwrap();
    assert_eq!(g.matches.len(), 1);
    g.matches.pop().unwrap()
}

#[test]
fn simple_script() {
    let _lock = lock();

    let m_ast = parse_one_match(SIMPLE_SCRIPT_OK);
    let mut st = runtime::State::new();
    st.push_msg("init", runtime::Msg::new());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match st.eval(&m_exc) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("init"));
                    assert_eq!(ctx.msgs.len(), 1);
                    Ok(())
                })
                .is_ok());
            assert!(txn.apply(&m_exc).is_ok());
        }
        None => panic!("expected match"),
    }
}

#[test]
fn simple_script_err() {
    let _lock = lock();

    let m_ast = parse_one_match(SIMPLE_SCRIPT_ERR);
    let mut st = runtime::State::new();
    st.push_msg("init", runtime::Msg::new());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match st.eval(&m_exc) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("init"));
                    assert_eq!(ctx.msgs.len(), 1);
                    Ok(())
                })
                .is_ok());
            match txn.apply(&m_exc) {
                Ok(()) => panic!("expected script to error"),
                Err(e) => {
                    match e {
                        runtime::StatefulError::Exec(rc, ref stderr) => {
                            assert_eq!(rc, 1);
                            assert_eq!(stderr, "crash and burn\n");
                        }
                        _ => {
                            panic!("unexpected error: {}", e);
                        }
                    }
                }
            }
        }
        None => panic!("expected match"),
    }
}

#[test]
fn env_check_script_ok() {
    let _lock = lock();

    let m_ast = parse_one_match(ENV_CHECK_SCRIPT);
    let mut st = runtime::State::new();
    st.push_msg("test",
                [("content".to_string(), Value::from_str("hello world"))]
                    .iter()
                    .cloned()
                    .collect());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match st.eval(&m_exc) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("test"));
                    assert_eq!(ctx.msgs.len(), 1);
                    Ok(())
                })
                .is_ok());
            assert!(txn.apply(&m_exc).is_ok());
        }
        None => panic!("expected match"),
    }
}

#[test]
fn hello_script_server() {
    let _lock = lock();

    let m_ast = parse_one_match(HELLO_SCRIPT_SERVER);
    let mut st = runtime::State::new();
    st.push_msg("init", runtime::Msg::new());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match st.eval(&m_exc) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("init"));
                    assert_eq!(ctx.msgs.len(), 1);
                    Ok(())
                })
                .is_ok());
            assert!(txn.apply(&m_exc).is_ok());
        }
        None => panic!("expected match"),
    }
    assert_eq!(st.get(&Identifier::from_str("foo")),
               Some(&Value::from_str("hello-bar")));
}
