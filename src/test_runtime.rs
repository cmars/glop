#![cfg(test)]

use super::ast;
use super::grammar;
use super::runtime;

const SIMPLE_INIT: &'static str = r#"match (message init) { acknowledge init; }"#;
const TWO_MSGS: &'static str =
    r#"match (message foo, message bar) { acknowledge foo; acknowledge bar; }"#;
const SIMPLE_EQUAL: &'static str = r#"match (foo == bar) { unset foo; }"#;
const SIMPLE_NOT_EQUAL: &'static str = r#"match (foo != bar) { set foo bar; }"#;

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
    match m_exc.eval(&mut st) {
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
    match m_exc.eval(&mut st) {
        Some(ctx) => {
            assert_eq!(ctx.seq, 1);
            assert!(ctx.msgs.contains_key("init"));
            assert_eq!(ctx.msgs.len(), 1);
        }
        None => panic!("expected match"),
    }
    // match m_exc.eval(&mut st) {
    // Some(_) => panic("unexpected match"),
    // None => {}
    // }
}

#[test]
fn matched_only_init_message() {
    let m_ast = parse_one_match(SIMPLE_INIT);
    let mut st = runtime::State::new();
    st.push_msg("init", runtime::Msg::new());
    st.push_msg("blah",
                [("foo".to_string(), "bar".to_string())].iter().cloned().collect());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match m_exc.eval(&mut st) {
        Some(ctx) => {
            assert_eq!(ctx.seq, 1);
            assert!(ctx.msgs.contains_key("init"));
            assert_eq!(ctx.msgs.len(), 1);
        }
        None => panic!("expected match"),
    }
    // match m_exc.eval(&mut st) {
    // Some(_) => panic("unexpected match"),
    // None => {}
    // }
}

#[test]
fn matched_two_messages() {
    let m_ast = parse_one_match(TWO_MSGS);
    let mut st = runtime::State::new();
    st.push_msg("foo", runtime::Msg::new());
    st.push_msg("bar", runtime::Msg::new());
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match m_exc.eval(&mut st) {
        Some(ctx) => {
            assert_eq!(ctx.seq, 1);
            assert!(ctx.msgs.contains_key("foo"));
            assert!(ctx.msgs.contains_key("bar"));
            assert_eq!(ctx.msgs.len(), 2);
        }
        None => panic!("expected match"),
    }
    // match m_exc.eval(&mut st) {
    // Some(_) => panic("unexpected match"),
    // None => {}
    // }
}

#[test]
fn matched_equal() {
    let m_ast = parse_one_match(SIMPLE_EQUAL);
    {
        let mut st = runtime::State::new();
        st.set_var("foo", "bar");
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match m_exc.eval(&mut st) {
            Some(ctx) => {
                assert_eq!(ctx.seq, 1);
            }
            None => panic!("expected match"),
        }
    }
    {
        let mut st = runtime::State::new();
        st.set_var("foo", "blah");
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match m_exc.eval(&mut st) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}

#[test]
fn matched_not_equal() {
    let m_ast = parse_one_match(SIMPLE_NOT_EQUAL);
    {
        let mut st = runtime::State::new();
        st.set_var("foo", "blah");
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match m_exc.eval(&mut st) {
            Some(ctx) => {
                assert_eq!(ctx.seq, 1);
            }
            None => panic!("expected match"),
        }
    }
    {
        let mut st = runtime::State::new();
        st.set_var("foo", "bar");
        let m_exc = runtime::Match::new_from_ast(&m_ast);
        match m_exc.eval(&mut st) {
            Some(_) => panic!("unexpected match"),
            None => {}
        }
    }
}
