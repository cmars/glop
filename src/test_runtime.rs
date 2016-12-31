#![cfg(test)]

use super::ast;
use super::grammar;
use super::runtime;

fn test_match_init() -> ast::Match {
    let mut g = grammar::glop(r#"match (message init) { acknowledge init; }"#).unwrap();
    assert_eq!(g.matches.len(), 1);
    g.matches.pop().unwrap()
}

#[test]
fn match_init_empty_state() {
    let m_ast = test_match_init();
    let mut st = runtime::State::new();
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match m_exc.eval(&mut st) {
        Some(_) => panic!("unexpected match"),
        None => (),
    }
}

#[test]
fn match_init_message() {
    let m_ast = test_match_init();
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
}
