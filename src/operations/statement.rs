use parser::Statement;
use parser::Statement::*;

use super::{Operation, declaration};
use super::scope::ScopeStack;

pub fn into_operations(node: Statement, scope: &mut ScopeStack) -> Vec<Operation> {
    match node {
        Comment(_) => Vec::new(),
        Declaration {pattern, type_def, expr} => {
            declaration::into_operations(pattern, type_def, expr, scope)
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
