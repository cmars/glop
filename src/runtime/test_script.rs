#![cfg(test)]

use super::*;
use super::super::grammar;
use super::super::signal_fix;
use self::value::{Message, Obj, Value};

const SIMPLE_SCRIPT_OK: &'static str = r###"
when (message init) {
    script #!/bin/bash
set -ex
[ -n "$GLOP_WORKSPACE" ]
[ "$GLOP_WORKSPACE" = "$(pwd)" ]
echo "hello world"
!#
}
"###;
const SIMPLE_SCRIPT_ERR: &'static str = r###"
when (message init) {
    script #!/bin/bash
>&2 echo "crash and burn"
exit 1
!#
}
"###;
const ENV_CHECK_SCRIPT: &'static str = r###"
when (message test) {
    var set foo bar;
    script #!/bin/bash
env
[ "${test__content}" == "hello world" ]
!#
}
"###;
const HELLO_SCRIPT_SERVER: &'static str = r###"
when (message init) {
    var set foo bar;
    script #!/bin/bash
set -e
[ -n "$GLOP_SCRIPT_ADDR" ]
[ -n "$GLOP_SCRIPT_KEY" ]

FOO=$(cargo run var get foo)
[ "${FOO}" = "bar" ]
cargo run var set foo hello-${FOO}
!#
}
"###;
const SCRIPT_SERVER_ACCESS_MSG: &'static str = r###"
when (message init) {
    script #!/bin/bash
set -e
[ -n "$GLOP_SCRIPT_ADDR" ]
[ -n "$GLOP_SCRIPT_KEY" ]

# glop msg get init foo
FOO=$(cargo run msg get init foo)
[ "${FOO}" = "bar" ]

# glop var set all good
cargo run var set all good
!#
}
"###;

fn test_msg(topic: &str, contents: Obj) -> Message {
    Message::new(topic, contents)
        .src("")
        .src_role(None)
        .dst("test")
}

fn parse_one_match(s: &str) -> ast::Match {
    let mut g = grammar::glop(s).unwrap();
    assert_eq!(g.matches.len(), 1);
    g.matches.pop().unwrap()
}

#[test]
fn simple_script() {
    let _lock = signal_fix::lock();

    let m_ast = parse_one_match(SIMPLE_SCRIPT_OK);
    let mut st = State::new("test", MemStorage::new());
    st.mut_storage().push_msg(test_msg("init", Obj::new())).unwrap();
    let m_exc = Match::new_from_ast(&m_ast);
    let mut txn = match st.eval(m_exc.clone()).unwrap() {
        Some(mut txn) => {
            assert_eq!(txn.seq, 0);
            txn.with_context(|ref mut ctx| {
                assert!(ctx.msgs.contains_key("init"));
                assert_eq!(ctx.msgs.len(), 1);
            });
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
}

#[test]
fn simple_script_err() {
    let _lock = signal_fix::lock();

    let m_ast = parse_one_match(SIMPLE_SCRIPT_ERR);
    let mut st = State::new("test", MemStorage::new());
    st.mut_storage().push_msg(test_msg("init", Obj::new())).unwrap();
    let m_exc = Match::new_from_ast(&m_ast);
    let mut txn = match st.eval(m_exc.clone()).unwrap() {
        Some(mut txn) => {
            assert_eq!(txn.seq, 0);
            txn.with_context(|ref mut ctx| {
                assert!(ctx.msgs.contains_key("init"));
                assert_eq!(ctx.msgs.len(), 1);
            });
            txn
        }
        None => panic!("expected match"),
    };
    match st.commit(&mut txn) {
        Ok(_) => panic!("expected script to error"),
        Err(e) => {
            match e {
                Error::Exec(rc, ref stderr) => {
                    assert_eq!(rc, 1);
                    assert_eq!(stderr, "crash and burn");
                }
                _ => {
                    panic!("unexpected error: {}", e);
                }
            }
        }
    }
}

#[test]
fn env_check_script_ok() {
    let _lock = signal_fix::lock();

    let m_ast = parse_one_match(ENV_CHECK_SCRIPT);
    let mut st = State::new("test", MemStorage::new());
    st.mut_storage()
        .push_msg(test_msg("test",
                           [("content".to_string(), Value::from_str("hello world"))]
                               .iter()
                               .cloned()
                               .collect()))
        .unwrap();
    let m_exc = Match::new_from_ast(&m_ast);
    let mut txn = match st.eval(m_exc.clone()).unwrap() {
        Some(mut txn) => {
            assert_eq!(txn.seq, 0);
            txn.with_context(|ref mut ctx| {
                assert!(ctx.msgs.contains_key("test"));
                assert_eq!(ctx.msgs.len(), 1);
            });
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
}

#[test]
fn hello_script_server() {
    let _lock = signal_fix::lock();

    let m_ast = parse_one_match(HELLO_SCRIPT_SERVER);
    let mut st = State::new("test", MemStorage::new());
    st.mut_storage().push_msg(test_msg("init", Obj::new())).unwrap();
    let m_exc = Match::new_from_ast(&m_ast);
    let mut txn = match st.eval(m_exc.clone()).unwrap() {
        Some(mut txn) => {
            assert_eq!(txn.seq, 0);
            txn.with_context(|ctx| {
                assert!(ctx.msgs.contains_key("init"));
                assert_eq!(ctx.msgs.len(), 1);
            });
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
    assert_eq!(st.storage().vars().get("foo"),
               Some(&Value::from_str("hello-bar")));
}

#[test]
fn script_server_access_msg() {
    let _lock = signal_fix::lock();

    let m_ast = parse_one_match(SCRIPT_SERVER_ACCESS_MSG);
    let mut st = State::new("test", MemStorage::new());
    st.mut_storage()
        .push_msg(test_msg("init",
                           [("foo".to_string(), Value::Str("bar".to_string()))]
                               .iter()
                               .cloned()
                               .collect()))
        .unwrap();
    let m_exc = Match::new_from_ast(&m_ast);
    let mut txn = match st.eval(m_exc.clone()).unwrap() {
        Some(txn) => txn,
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
    assert_eq!(st.storage().vars().get("all"),
               Some(&Value::from_str("good")));
}
