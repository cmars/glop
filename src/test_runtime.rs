#![cfg(unix)]
#![cfg(test)]

extern crate futures;
extern crate libc;
extern crate tokio_core;
extern crate tokio_signal;

use std::sync::mpsc::channel;
use std::sync::{Once, ONCE_INIT, Mutex, MutexGuard};
use std::thread;

use self::tokio_core::reactor::{Core, Timeout};
use self::tokio_signal::unix::Signal;

use super::ast;
use super::grammar;
use super::runtime;
use super::runtime::Stateful;
use super::value::{Identifier, Value};

const SIMPLE_INIT: &'static str = r#"match (message init) { acknowledge init; }"#;
const TWO_MSGS: &'static str =
    r#"match (message foo, message bar) { acknowledge foo; acknowledge bar; }"#;
const SIMPLE_EQUAL: &'static str = r#"match (foo == bar) { unset foo; }"#;
const SIMPLE_NOT_EQUAL: &'static str = r#"match (foo != bar) { set foo bar; }"#;
const SIMPLE_IS_SET: &'static str = r#"match (is_set foo) { unset foo; }"#;
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
fn unmatched_init_empty_state() {
    let m_ast = parse_one_match(SIMPLE_INIT);
    let mut st = runtime::State::new();
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => (),
    }
}

#[test]
fn matched_init_message() {
    let m_ast = parse_one_match(SIMPLE_INIT);
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
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn matched_only_init_message() {
    let m_ast = parse_one_match(SIMPLE_INIT);
    let mut st = runtime::State::new();
    st.push_msg("init", runtime::Msg::new());
    st.push_msg("blah",
                [("foo".to_string(), Value::Str("bar".to_string()))]
                    .iter()
                    .cloned()
                    .collect());
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
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn matched_two_messages() {
    let m_ast = parse_one_match(TWO_MSGS);
    let mut st = runtime::State::new();
    st.push_msg("foo", runtime::Msg::new());
    st.push_msg("bar", runtime::Msg::new());
    st.push_msg("foo", runtime::Msg::new());
    st.push_msg("bar", runtime::Msg::new());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    for i in 0..2 {
        match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, i);
                assert!(txn.with_context(|ref mut ctx| {
                        assert!(ctx.msgs.contains_key("foo"));
                        assert!(ctx.msgs.contains_key("bar"));
                        assert_eq!(ctx.msgs.len(), 2);
                        Ok(())
                    })
                    .is_ok());
                assert!(txn.apply(&m_exc).is_ok());
            }
            None => panic!("expected match"),
        }
    }
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn match_equal() {
    let m_ast = parse_one_match(SIMPLE_EQUAL);
    {
        let mut st = runtime::State::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("bar"));
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, 0);
                assert!(txn.apply(&m_exc).is_ok());
            }
            None => panic!("expected match"),
        }
        // foo is now unset
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
    {
        let mut st = runtime::State::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("blah"));
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn match_not_equal() {
    let m_ast = parse_one_match(SIMPLE_NOT_EQUAL);
    {
        let mut st = runtime::State::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("blah"));
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, 0);
            }
            None => panic!("expected match"),
        }
    }
    {
        let mut st = runtime::State::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("bar"));
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn simple_commit_progression() {
    let m_exc_ne = runtime::Match::new_from_ast(&parse_one_match(SIMPLE_NOT_EQUAL));
    let m_exc_eq = runtime::Match::new_from_ast(&parse_one_match(SIMPLE_EQUAL));
    let mut st = runtime::State::new();
    st.set_var(&Identifier::from_str("foo"), Value::from_str("blah"));
    // foo starts out != bar so we expect a match and apply
    match st.eval(&m_exc_ne) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.apply(&m_exc_ne).is_ok());
        }
        None => panic!("expected match"),
    }
    // above match sets foo == bar so m_exc_ne no longer matches
    match st.eval(&m_exc_ne) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
    // now let's match on foo == bar, should match committed state now
    match st.eval(&m_exc_eq) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 1);
            assert!(txn.apply(&m_exc_eq).is_ok());
        }
        None => panic!("expected match"),
    }
}

#[test]
fn match_is_set() {
    let m_ast = parse_one_match(SIMPLE_IS_SET);
    {
        let mut st = runtime::State::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("bar"));
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, 0);
                assert!(txn.apply(&m_exc).is_ok());
            }
            None => panic!("expected match"),
        }
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
    {
        let mut st = runtime::State::new();
        st.set_var(&Identifier::from_str("bar"), Value::from_str("foo"));
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
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
