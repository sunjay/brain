use parser::Program;

use super::{OperationsResult, statement};
use super::scope::ScopeStack;

pub fn into_operations(ast: Program, scope: &mut ScopeStack) -> OperationsResult {
    let mut ops = Vec::new();

    for stmt in ast.into_iter() {
        ops.extend(statement::into_operations(stmt, scope)?);
    }

    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Program;

    #[test]
    fn empty_program() {
        let program = Program::new();
        let mut scope = ScopeStack::new();

        let ops = into_operations(program, &mut scope).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
