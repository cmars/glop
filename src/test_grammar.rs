#![cfg(test)]

use super::grammar;

#[test]
fn round_trip_simple() {
    let src = r#"when (message init) {
    var set installed false;
    var set initialized true;
    msg pop init;
}

when (installed == false, initialized == true) {
    script #!/bin/bash
set -ex
echo "hello world"
!#
    var set installed true;
}

when (message config, is_set initialized) {
    msg pop config;
}

when (message foo, initialized != baz) {
    var set has_foo true;
    var unset bar;
}

"#;
    let g = grammar::glop(src).unwrap();
    assert_eq!(format!("{}", g), src);
}

#[test]
fn err_empty() {
    assert!(grammar::glop("").is_err());
}

#[test]
fn err_empty_conditions() {
    assert!(grammar::glop(r#"when () { set foo bar; }"#).is_err());
}

#[test]
fn err_empty_actions() {
    assert!(grammar::glop(r#"when (foo == "bar") { }"#).is_ok());
    assert!(grammar::glop(r#"when (foo == "bar")"#).is_err());
}
