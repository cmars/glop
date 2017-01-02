pub mod ast;

pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}

pub mod runtime;
mod value;

mod test_grammar;
mod test_runtime;
mod test_value;
