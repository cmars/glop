#![cfg(test)]

use super::*;
use super::super::grammar;
use self::value::{Identifier, Obj, Value};

const SIMPLE_INIT: &'static str = r#"when (message init) { acknowledge init; }"#;
const TWO_MSGS: &'static str =
    r#"when (message foo, message bar) { acknowledge foo; acknowledge bar; }"#;
const SIMPLE_EQUAL: &'static str = r#"when (foo == bar) { unset foo; }"#;
const SIMPLE_NOT_EQUAL: &'static str = r#"when (foo != bar) { set foo bar; }"#;
const SIMPLE_IS_SET: &'static str = r#"when (is_set foo) { unset foo; }"#;

fn parse_one_match(s: &str) -> ast::Match {
    let mut g = grammar::glop(s).unwrap();
    assert_eq!(g.matches.len(), 1);
    g.matches.pop().unwrap()
}

#[test]
fn unmatched_init_empty_state() {
    let m_ast = parse_one_match(SIMPLE_INIT);
    let mut st = MemState::new();
    let m_exc = Match::new_from_ast(&m_ast);
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => (),
    }
}

#[test]
fn matched_init_message() {
    let m_ast = parse_one_match(SIMPLE_INIT);
    let mut st = MemState::new();
    st.push_msg("init", Obj::new());
    let m_exc = Match::new_from_ast(&m_ast);
    let actions = match st.eval(&m_exc) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("init"));
                    assert_eq!(ctx.msgs.len(), 1);
                    Ok(())
                })
                .is_ok());
            txn.apply(&m_exc).unwrap()
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&actions).is_ok());
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn matched_only_init_message() {
    let m_ast = parse_one_match(SIMPLE_INIT);
    let mut st = MemState::new();
    st.push_msg("init", Obj::new());
    st.push_msg("blah",
                [("foo".to_string(), Value::Str("bar".to_string()))]
                    .iter()
                    .cloned()
                    .collect());
    let m_exc = Match::new_from_ast(&m_ast);
    let actions = match st.eval(&m_exc) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            assert!(txn.with_context(|ref mut ctx| {
                    assert!(ctx.msgs.contains_key("init"));
                    assert_eq!(ctx.msgs.len(), 1);
                    Ok(())
                })
                .is_ok());
            txn.apply(&m_exc).unwrap()
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&actions).is_ok());
    match st.eval(&m_exc) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
}

#[test]
fn matched_two_messages() {
    let m_ast = parse_one_match(TWO_MSGS);
    let mut st = MemState::new();
    st.push_msg("foo", Obj::new());
    st.push_msg("bar", Obj::new());
    st.push_msg("foo", Obj::new());
    st.push_msg("bar", Obj::new());
    let m_exc = Match::new_from_ast(&m_ast);
    for i in 0..2 {
        let actions = match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, i);
                assert!(txn.with_context(|ref mut ctx| {
                        assert!(ctx.msgs.contains_key("foo"));
                        assert!(ctx.msgs.contains_key("bar"));
                        assert_eq!(ctx.msgs.len(), 2);
                        Ok(())
                    })
                    .is_ok());
                txn.apply(&m_exc).unwrap()
            }
            None => panic!("expected match"),
        };
        assert!(st.commit(&actions).is_ok());
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
        let mut st = MemState::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("bar"));
        let m_exc = Match::new_from_ast(&m_ast);
        let actions = match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, 0);
                txn.apply(&m_exc).unwrap()
            }
            None => panic!("expected match"),
        };
        assert!(st.commit(&actions).is_ok());
        // foo is now unset
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
    {
        let mut st = MemState::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("blah"));
        let m_exc = Match::new_from_ast(&m_ast);
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
        let mut st = MemState::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("blah"));
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, 0);
            }
            None => panic!("expected match"),
        }
    }
    {
        let mut st = MemState::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("bar"));
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn simple_commit_progression() {
    let m_exc_ne = Match::new_from_ast(&parse_one_match(SIMPLE_NOT_EQUAL));
    let m_exc_eq = Match::new_from_ast(&parse_one_match(SIMPLE_EQUAL));
    let mut st = MemState::new();
    st.set_var(&Identifier::from_str("foo"), Value::from_str("blah"));
    // foo starts out != bar so we expect a match and apply
    let actions = match st.eval(&m_exc_ne) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 0);
            txn.apply(&m_exc_ne).unwrap()
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&actions).is_ok());
    // above match sets foo == bar so m_exc_ne no longer matches
    match st.eval(&m_exc_ne) {
        Some(_) => panic!("unexpected match"),
        None => {}
    }
    // now let's match on foo == bar, should match committed state now
    let actions = match st.eval(&m_exc_eq) {
        Some(ref mut txn) => {
            assert_eq!(txn.seq, 1);
            txn.apply(&m_exc_eq).unwrap()
        }
        None => panic!("expected match"),
    };
    assert!(st.commit(&actions).is_ok());
}

#[test]
fn match_is_set() {
    let m_ast = parse_one_match(SIMPLE_IS_SET);
    {
        let mut st = MemState::new();
        st.set_var(&Identifier::from_str("foo"), Value::from_str("bar"));
        let m_exc = Match::new_from_ast(&m_ast);
        let actions = match st.eval(&m_exc) {
            Some(ref mut txn) => {
                assert_eq!(txn.seq, 0);
                txn.apply(&m_exc).unwrap()
            }
            None => panic!("expected match"),
        };
        assert!(st.commit(&actions).is_ok());
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
    {
        let mut st = MemState::new();
        st.set_var(&Identifier::from_str("bar"), Value::from_str("foo"));
        let m_exc = Match::new_from_ast(&m_ast);
        match st.eval(&m_exc) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}
