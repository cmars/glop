use std::process::Command;
use std::process::Stdio;
use std::io::Write;

pub mod ast;
pub mod glop;

fn main() {
    let g = glop::parse_Glop("
match (message init) {
      set installed false;
        acknowledge;
};

match (installed == false) {
      shell \"install-things.bash\";
        set installed true;
};
").unwrap();

    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .spawn()
        .expect("command failed");
    child.stdin.as_mut().unwrap().write_all("hello".as_bytes());
    let output = child.wait_with_output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
