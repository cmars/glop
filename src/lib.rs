pub mod ast;

pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}

mod tests;
