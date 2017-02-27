use parser::Program;
use memory::StaticAllocator;

use super::{Operation, statement};

pub fn into_operations(ast: Program, mem: &mut StaticAllocator) -> Vec<Operation> {
    ast.into_iter().fold(Vec::new(), |mut acc, stmt| {
        acc.extend(statement::into_operations(stmt, mem));
        acc
    })
}
