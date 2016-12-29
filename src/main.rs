extern crate glop;

use std::process::Command;
use std::process::Stdio;
use std::io::Write;

use glop::ast;
use glop::grammar;

fn main() {
    let g = grammar::glop(r#"
match (message init) {
    set installed false;
    acknowledge init;
}

match (installed == false) {
    exec "install-things.bash";
    set installed true;
}
"#)
        .unwrap();
    println!("{}", g);

    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .spawn()
        .expect("command failed");
    child.stdin.as_mut().unwrap().write_all("hello".as_bytes());
    let output = child.wait_with_output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
