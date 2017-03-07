#![cfg(test)]

extern crate env_logger;
extern crate textnonce;

use std;

use super::*;
use super::super::grammar;
use self::value::{Message, Obj, Value};

const SIMPLE_INIT: &'static str = r#"when (message init) { }"#;
const TWO_MSGS: &'static str = r#"when (message foo, message bar) { }"#;
const OTHER_MSG: &'static str = r#"when (message other) { var set other matched; }"#;
const TWO_MSGS_IS_SET: &'static str = r#"when (message foo, message bar, is_set baz) { }"#;
const SIMPLE_EQUAL: &'static str = r#"when (foo == bar) { var unset foo; }"#;
const SIMPLE_NOT_EQUAL: &'static str = r#"when (foo != bar) { var set foo bar; }"#;
const SIMPLE_IS_SET: &'static str = r#"when (is_set foo) { var unset foo; }"#;

fn test_msg(topic: &str, contents: Obj) -> Message {
    Message {
        src: "".to_string(),
        src_role: None,
        dst: "test".to_string(),
        topic: topic.to_string(),
        contents: contents,
    }
}

fn setup() {
    let _ = env_logger::init();
}

fn parse_one_match(s: &str) -> ast::Match {
    let mut g = grammar::glop(s).unwrap();
    assert_eq!(g.matches.len(), 1);
    g.matches.pop().unwrap()
}

fn mem_state() -> (State<MemStorage>, cleanup::Cleanup) {
    (State::new("test", MemStorage::new()), cleanup::Cleanup::Empty)
}

fn durable_state() -> (State<DurableStorage>, cleanup::Cleanup) {
    let mut storage_path_buf = std::env::temp_dir();
    storage_path_buf.push(rand_string());
    let storage_path = storage_path_buf.to_str().unwrap();
    let cl = cleanup::Cleanup::Dir(storage_path.to_string());
    let storage = DurableStorage::new(storage_path).unwrap();
    (State::new("test", storage), cl)
}

fn rand_string() -> String {
    textnonce::TextNonce::sized_urlsafe(32).unwrap().into_string()
}

type StateFactory<T> = fn() -> (State<T>, cleanup::Cleanup);

#[test]
fn mem_unmatched_init_empty_state() {
    unmatched_init_empty_state(mem_state)
}

#[test]
fn durable_unmatched_init_empty_state() {
    unmatched_init_empty_state(durable_state)
}

fn unmatched_init_empty_state<T: Storage>(f: StateFactory<T>) {
    setup();
    let (st, _cleanup) = f();
    let mut st = st;
    let m_ast = parse_one_match(SIMPLE_INIT);
    let m_exc = Match::new_from_ast(&m_ast);
    match st.eval(m_exc).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => (),
    }
}

#[test]
fn mem_matched_init_message() {
    matched_init_message(mem_state)
}

#[test]
fn durable_matched_init_message() {
    matched_init_message(durable_state)
}

fn matched_init_message<T: Storage>(f: StateFactory<T>) {
    setup();
    let (st, _cleanup) = f();
    let mut st = st;
    let m_ast = parse_one_match(SIMPLE_INIT);
    st.mut_storage()
        .push_msg(test_msg("init", Obj::new()))
        .unwrap();
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

    match st.eval(m_exc.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn mem_rollback_msg() {
    rollback_msg(mem_state)
}

#[test]
fn durable_rollback_msg() {
    rollback_msg(durable_state)
}

fn rollback_msg<T: Storage>(f: StateFactory<T>) {
    setup();
    let (st, _cleanup) = f();
    let mut st = st;
    let m_ast = parse_one_match(SIMPLE_INIT);
    st.mut_storage()
        .push_msg(test_msg("init", Obj::new()))
        .unwrap();
    let m_exc = Match::new_from_ast(&m_ast);
    let txn = match st.eval(m_exc.clone()).unwrap() {
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
    assert!(st.rollback(txn).is_ok());

    // init message should have been nak'ed on rollback
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

    // Now init message has been consumed.
    match st.eval(m_exc.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn mem_matched_only_init_message() {
    matched_only_init_message(mem_state)
}

#[test]
fn durable_matched_only_init_message() {
    matched_only_init_message(durable_state)
}

fn matched_only_init_message<T: Storage>(f: StateFactory<T>) {
    setup();
    let (st, _cleanup) = f();
    let mut st = st;
    let m_ast = parse_one_match(SIMPLE_INIT);
    st.mut_storage()
        .push_msg(test_msg("init", Obj::new()))
        .unwrap();
    st.mut_storage()
        .push_msg(test_msg("blah",
                           [("foo".to_string(), Value::Str("bar".to_string()))]
                               .iter()
                               .cloned()
                               .collect()))
        .unwrap();
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

    match st.eval(m_exc.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn mem_matched_two_messages() {
    matched_two_messages(mem_state)
}

#[test]
fn durable_matched_two_messages() {
    matched_two_messages(durable_state)
}

fn matched_two_messages<T: Storage>(f: StateFactory<T>) {
    setup();
    let (st, _cleanup) = f();
    let mut st = st;
    let m_ast = parse_one_match(TWO_MSGS);
    st.mut_storage().push_msg(test_msg("foo", Obj::new())).unwrap();
    st.mut_storage().push_msg(test_msg("bar", Obj::new())).unwrap();
    st.mut_storage().push_msg(test_msg("foo", Obj::new())).unwrap();
    st.mut_storage().push_msg(test_msg("bar", Obj::new())).unwrap();
    let m_exc = Match::new_from_ast(&m_ast);

    for i in 0..2 {
        let mut txn = match st.eval(m_exc.clone()).unwrap() {
            Some(mut txn) => {
                assert_eq!(txn.seq, i);
                txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("foo"));
                    assert!(ctx.msgs.contains_key("bar"));
                    assert_eq!(ctx.msgs.len(), 2);
                });
                txn
            }
            None => panic!("expected match"),
        };
        assert!(st.commit(&mut txn).is_ok());
    }
    match st.eval(m_exc.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn mem_match_equal() {
    match_equal(mem_state)
}

#[test]
fn durable_match_equal() {
    match_equal(durable_state)
}

fn match_equal<T: Storage>(f: StateFactory<T>) {
    setup();
    let m_ast = parse_one_match(SIMPLE_EQUAL);
    {
        let (st, _cleanup) = f();
        let mut st = st;
        st.mut_storage()
            .save(0,
                  [("foo".to_string(), Value::from_str("bar"))].iter().cloned().collect())
            .unwrap();
        let m_exc = Match::new_from_ast(&m_ast);
        let mut txn = match st.eval(m_exc.clone()).unwrap() {
            Some(txn) => {
                assert_eq!(txn.seq, 1);
                txn
            }
            None => panic!("expected match"),
        };
        assert!(st.commit(&mut txn).is_ok());
        // foo is now unset
        match st.eval(m_exc.clone()).unwrap() {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
    {
        let (st, _cleanup) = f();
        let mut st = st;
        st.mut_storage()
            .save(0,
                  [("foo".to_string(), Value::from_str("blah"))].iter().cloned().collect())
            .unwrap();
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(m_exc.clone()).unwrap() {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn mem_rollback_vars() {
    rollback_vars(mem_state)
}

#[test]
fn durable_rollback_vars() {
    rollback_vars(durable_state)
}

fn rollback_vars<T: Storage>(f: StateFactory<T>) {
    setup();
    let (st, _cleanup) = f();
    let mut st = st;
    st.mut_storage()
        .save(0,
              [("foo".to_string(), Value::from_str("bar"))].iter().cloned().collect())
        .unwrap();
    let m_ast = parse_one_match(SIMPLE_EQUAL);
    let m_exc = Match::new_from_ast(&m_ast);
    let txn = match st.eval(m_exc.clone()).unwrap() {
        Some(txn) => {
            assert_eq!(txn.seq, 1);
            txn
        }
        None => panic!("expected match"),
    };
    debug!(target: "rollback_vars", "BEGIN rollback");
    assert!(st.rollback(txn).is_ok());
    debug!(target: "rollback_vars", "OK rollback");
    // foo should be still set
    let mut txn = match st.eval(m_exc.clone()).unwrap() {
        Some(txn) => {
            assert_eq!(txn.seq, 1);
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
    // foo is now unset
    match st.eval(m_exc.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn mem_match_not_equal() {
    match_not_equal(mem_state)
}

#[test]
fn durable_match_not_equal() {
    match_not_equal(durable_state)
}

fn match_not_equal<T: Storage>(f: StateFactory<T>) {
    setup();
    let m_ast = parse_one_match(SIMPLE_NOT_EQUAL);
    {
        let (st, _cleanup) = f();
        let mut st = st;
        st.mut_storage()
            .save(0,
                  [("foo".to_string(), Value::from_str("blah"))].iter().cloned().collect())
            .unwrap();
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(m_exc.clone()).unwrap() {
            Some(txn) => {
                assert_eq!(txn.seq, 1);
            }
            None => panic!("expected match"),
        }
    }
    {
        let (st, _cleanup) = f();
        let mut st = st;
        st.mut_storage()
            .save(0,
                  [("foo".to_string(), Value::from_str("bar"))].iter().cloned().collect())
            .unwrap();
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(m_exc.clone()).unwrap() {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn mem_simple_commit_progression() {
    simple_commit_progression(mem_state)
}

#[test]
fn durable_simple_commit_progression() {
    simple_commit_progression(durable_state)
}

fn simple_commit_progression<T: Storage>(f: StateFactory<T>) {
    setup();
    let m_exc_ne = Match::new_from_ast(&parse_one_match(SIMPLE_NOT_EQUAL));
    let m_exc_eq = Match::new_from_ast(&parse_one_match(SIMPLE_EQUAL));
    let (st, _cleanup) = f();
    let mut st = st;
    st.mut_storage()
        .save(0,
              [("foo".to_string(), Value::from_str("blah"))].iter().cloned().collect())
        .unwrap();
    // foo starts out != bar so we expect a match and apply
    let mut txn = match st.eval(m_exc_ne.clone()).unwrap() {
        Some(txn) => {
            assert_eq!(txn.seq, 1);
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
    // above match sets foo == bar so m_exc_ne no longer matches
    match st.eval(m_exc_ne.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    }

    // now let's match on foo == bar, should match committed state now
    let mut txn = match st.eval(m_exc_eq.clone()).unwrap() {
        Some(txn) => {
            assert_eq!(txn.seq, 2);
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
}

#[test]
fn mem_match_is_set() {
    match_is_set(mem_state)
}

#[test]
fn durable_match_is_set() {
    match_is_set(durable_state)
}

fn match_is_set<T: Storage>(f: StateFactory<T>) {
    setup();
    let m_ast = parse_one_match(SIMPLE_IS_SET);
    {
        let (st, _cleanup) = f();
        let mut st = st;
        st.mut_storage()
            .save(0,
                  [("foo".to_string(), Value::from_str("bar"))].iter().cloned().collect())
            .unwrap();
        let m_exc = Match::new_from_ast(&m_ast);
        let mut txn = match st.eval(m_exc.clone()).unwrap() {
            Some(txn) => {
                assert_eq!(txn.seq, 1);
                txn
            }
            None => panic!("expected match"),
        };
        assert!(st.commit(&mut txn).is_ok());

        match st.eval(m_exc.clone()).unwrap() {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
    {
        let (st, _cleanup) = f();
        let mut st = st;
        st.mut_storage()
            .save(0,
                  [("bar".to_string(), Value::from_str("foo"))].iter().cloned().collect())
            .unwrap();
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(m_exc.clone()).unwrap() {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn mem_preserve_unmatched_message() {
    preserve_unmatched_message(mem_state)
}

#[test]
fn durable_preserve_unmatched_message() {
    preserve_unmatched_message(durable_state)
}

fn preserve_unmatched_message<T: Storage>(f: StateFactory<T>) {
    setup();
    let m_exc_tm = Match::new_from_ast(&parse_one_match(TWO_MSGS));
    let m_exc_tmis = Match::new_from_ast(&parse_one_match(TWO_MSGS_IS_SET));
    let m_exc_om = Match::new_from_ast(&parse_one_match(OTHER_MSG));
    let (st, _cleanup) = f();
    let mut st = st;
    st.mut_storage().push_msg(test_msg("foo", Obj::new())).unwrap();
    st.mut_storage().push_msg(test_msg("bar", Obj::new())).unwrap();
    st.mut_storage().push_msg(test_msg("other", Obj::new())).unwrap();
    // Doesn't match because baz is not set
    match st.eval(m_exc_tmis.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    };
    st.mut_storage()
        .save(0,
              [("baz".to_string(), Value::from_str("blah"))].iter().cloned().collect())
        .unwrap();
    // Matches foo and bar messages with baz now set
    let mut txn = match st.eval(m_exc_tmis.clone()).unwrap() {
        Some(txn) => {
            assert_eq!(txn.seq, 1);
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
    // Now the messages are gone
    match st.eval(m_exc_tm.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    };
    match st.eval(m_exc_tmis.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    };
    // 'other' message still left in context
    let mut txn = match st.eval(m_exc_om.clone()).unwrap() {
        Some(txn) => {
            assert_eq!(txn.seq, 2);
            txn
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&mut txn).is_ok());
    match st.eval(m_exc_om.clone()).unwrap() {
        Some(_) => panic!("unexpected match"),
        None => {}
    };
}
