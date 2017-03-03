use parser::Statement;
use parser::Statement::*;

use super::{Operation, declaration, expression};
use super::scope::{ScopeStack, UNIT_TYPE_ID};

pub fn into_operations(node: Statement, scope: &mut ScopeStack) -> Vec<Operation> {
    match node {
        Comment(_) => Vec::new(),
        Declaration {pattern, type_def, expr} => {
            declaration::into_operations(pattern, type_def, expr, scope)
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
        let ops = into_operations(Statement::Comment("foo".to_string()), &mut scope);
        assert_eq!(ops.len(), 0);
    }
}
