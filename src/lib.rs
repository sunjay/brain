#![recursion_limit = "1024"]

#[macro_use]
extern crate pest;

mod parser;
mod codegen;
mod memory;

pub use parser::*;
