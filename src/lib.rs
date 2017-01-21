pub mod ast;

pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}

mod cleanup;
mod value;
mod script;
pub mod runtime;

mod test_grammar;
mod test_runtime;
mod test_value;
