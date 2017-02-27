use parser::Program;

use super::{Operation, statement};
use super::scope::ScopeStack;

pub fn into_operations(ast: Program, scope: &mut ScopeStack) -> Vec<Operation> {
    ast.into_iter().fold(Vec::new(), |mut acc, stmt| {
        acc.extend(statement::into_operations(stmt, scope));
        acc
    })
}
