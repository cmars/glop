#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

mod ast;
mod cleanup;
mod value;
mod script;

pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}
pub mod error;
pub mod runtime;
pub mod agent;

mod test_grammar;
mod test_runtime;
mod test_script;
mod test_value;
