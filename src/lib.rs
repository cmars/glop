#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod ast;
mod cleanup;

pub mod agent;
pub mod crypto;
pub mod error;
pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/glop.rs"));
}
pub mod runtime;
pub mod signal_fix;
pub mod value;

mod test_grammar;
mod test_value;
