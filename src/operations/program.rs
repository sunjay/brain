use parser::Program;
use super::scope::ScopeStack;

use super::{Operation, statement};

pub fn into_operations(ast: Program, scope: &mut ScopeStack) -> Vec<Operation> {
    ast.into_iter().fold(Vec::new(), |mut acc, stmt| {
        acc.extend(statement::into_operations(stmt, scope));
        acc
    })
}
