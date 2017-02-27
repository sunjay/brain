mod operation;
mod program;
mod statement;

pub use self::operation::*;

use parser::Program;
use memory::StaticAllocator;

pub fn from_ast(ast: Program) -> Vec<Operation> {
    let mut allocator = StaticAllocator::new();
    program::into_operations(ast, &mut allocator)
}
