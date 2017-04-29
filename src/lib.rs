#![recursion_limit = "1024"]

#[macro_use]
extern crate pest;

pub mod parser;
pub mod operations;
pub mod memory;
pub mod core;
pub mod prelude;
pub mod codegen;
pub mod optimizations;
