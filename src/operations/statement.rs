use parser::Statement;
use parser::Statement::*;

use super::{OperationsResult, declaration, assignment, expression};
use super::scope::{ScopeStack, UNIT_TYPE_ID};

pub fn into_operations(node: Statement, scope: &mut ScopeStack) -> OperationsResult {
    match node {
        Comment(_) => Ok(Vec::new()),
        Declaration {pattern, type_def, expr} => {
            declaration::into_operations(pattern, type_def, expr, scope)
        },
        Assignment {lhs, expr} => {
            assignment::into_operations(lhs, expr, scope)
        },
        Expression {expr} => {
            expression::into_operations(expr, UNIT_TYPE_ID, None, scope)
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment() {
        // Make sure comments result in no operations
        let mut scope = ScopeStack::new();
        let ops = into_operations(Statement::Comment("foo".to_string()), &mut scope).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
