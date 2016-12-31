#![cfg(test)]

use super::ast;
use super::grammar;
use super::runtime;

fn match_init() -> ast::Match {
    let mut g = grammar::glop(r#"match (message init) { acknowledge init; }"#).unwrap();
    assert_eq!(g.matches.len(), 1);
    g.matches.pop().unwrap()
}

#[test]
fn match_init_empty_state() {
    let m_ast = match_init();
    let mut st = runtime::State::new();
    let m_exc = runtime::Match::new_from_ast(&m_ast);
    match m_exc.eval(&mut st) {
        Some(_) => panic!("unexpected match"),
        None => (),
    }
}
