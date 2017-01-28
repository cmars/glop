mod ast;
mod cleanup;
mod value;
mod script;
mod agent;

pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}
pub mod error;
pub mod runtime;

mod test_grammar;
mod test_runtime;
mod test_script;
mod test_value;
