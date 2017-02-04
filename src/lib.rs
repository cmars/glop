#[macro_use]
extern crate serde_derive;

mod ast;
mod cleanup;
mod value;

pub mod agent;
pub mod error;
pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}
pub mod runtime;
pub mod script;
pub mod signal_fix;

mod test_grammar;
mod test_runtime;
mod test_script;
mod test_value;
