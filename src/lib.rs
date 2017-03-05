#![recursion_limit = "1024"]

#[macro_use]
extern crate pest;

mod parser;
mod operations;
mod memory;
mod prelude;
mod core;

pub use parser::*;
pub use operations::*;
pub use memory::*;
