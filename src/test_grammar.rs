#![cfg(test)]

use super::ast;
use super::grammar;

#[test]
fn hello_state() {
    let src = r#"state start { log "hello world" } fault { goto start }"#;
    let result = grammar::state(src);
    assert_eq!(result,
               Ok(ast::State::SingularState {
                   name: "start".to_string(),
                   actions: vec![ast::Action::Log(ast::Expression::String("hello world"
                                     .to_string()))],
                   fault: vec![ast::Action::Goto("start".to_string())],
               }),
               "unexpected {:?}",
               result);
}

#[test]
fn expression_int() {
    let result = grammar::expression("3");
    assert_eq!(result,
               Ok(ast::Expression::Int(3)),
               "unexpected {:?}",
               result);
}

#[test]
fn expression_str() {
    let result = grammar::expression(r#""hello world""#);
    assert_eq!(result,
               Ok(ast::Expression::String("hello world".to_string())),
               "unexpected {:?}",
               result);
}

#[test]
fn expression_bool() {
    let result = grammar::expression("true");
    assert_eq!(result,
               Ok(ast::Expression::Bool(true)),
               "unexpected {:?}",
               result);
    let result = grammar::expression("false");
    assert_eq!(result,
               Ok(ast::Expression::Bool(false)),
               "unexpected {:?}",
               result);
}

#[test]
fn expression_id() {
    let result = grammar::expression("foo");
    assert_eq!(result,
               Ok(ast::Expression::Identifier("foo".to_string())),
               "unexpected {:?}",
               result);
}

#[test]
fn expressions() {
    let result = grammar::expression("2 * 3");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                         ast::BinOp::Mul,
                                         Box::new(ast::Expression::Int(3)))),
               "unexpected {:?}",
               result);
    let result = grammar::expression("2 + 3");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                         ast::BinOp::Add,
                                         Box::new(ast::Expression::Int(3)))),
               "unexpected {:?}",
               result);
    let result = grammar::expression("2 / 3");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                         ast::BinOp::Div,
                                         Box::new(ast::Expression::Int(3)))),
               "unexpected {:?}",
               result);
    let result = grammar::expression("2 - 3");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                         ast::BinOp::Sub,
                                         Box::new(ast::Expression::Int(3)))),
               "unexpected {:?}",
               result);
    let result = grammar::expression("2 * 3 + 5");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(
                        ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                           ast::BinOp::Mul,
                                           Box::new(ast::Expression::Int(3)))),
                    ast::BinOp::Add,
                    Box::new(ast::Expression::Int(5)))),
               "unexpected {:?}",
               result);
    let result = grammar::expression("2 * (3 + 5)");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                         ast::BinOp::Mul,
                                         Box::new(
                    ast::Expression::BinOp(Box::new(ast::Expression::Int(3)),
                    ast::BinOp::Add,
                    Box::new(ast::Expression::Int(5)))))),
               "unexpected {:?}",
               result);
    let result = grammar::expression("a == 2 && b != 3 && c > 4 && d < 5");
    assert_eq!(result,
               Ok(ast::Expression::BinOp(Box::new(ast::Expression::Int(2)),
                                         ast::BinOp::Mul,
                                         Box::new(
                    ast::Expression::BinOp(Box::new(ast::Expression::Int(3)),
                    ast::BinOp::Add,
                    Box::new(ast::Expression::Int(5)))))),
               "unexpected {:?}",
               result);
}
