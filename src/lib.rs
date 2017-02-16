#![recursion_limit = "1024"]

#[macro_use]
extern crate pest;

mod parser;
mod instruction;
mod instructions;
mod codegen;
mod memory;
mod optimizations;

pub use parser::*;
pub use instruction::*;
pub use instructions::*;
pub use optimizations::OptimizationLevel;
