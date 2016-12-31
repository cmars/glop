#![cfg(test)]

use super::grammar;

#[test]
fn round_trip_simple() {
    let src = r#"match (message init) {
    set installed false;
    set initialized true;
    acknowledge init;
}

match (installed == false, initialized == true) {
    exec "install-things.bash";
    set installed true;
}

match (message config, is_set initialized) {
    acknowledge config;
}

match (message foo, initialized != baz) {
    acknowledge foo;
    set has_foo true;
    unset bar;
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
    assert!(grammar::glop(r#"match () { set foo bar; }"#).is_err());
}

#[test]
fn err_empty_actions() {
    assert!(grammar::glop(r#"match (foo == "bar") { }"#).is_err());
    assert!(grammar::glop(r#"match (foo == "bar")"#).is_err());
}
