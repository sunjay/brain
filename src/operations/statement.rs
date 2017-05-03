use parser::Statement;
use parser::Statement::*;
use memory::MemoryBlock;

use super::{
    OperationsResult,
    declaration,
    assignment,
    while_loop,
    expression,
    Target,
};
use super::scope::ScopeStack;

pub fn into_operations(scope: &mut ScopeStack, node: Statement) -> OperationsResult {
    match node {
        Comment(_) => Ok(Vec::new()),
        Declaration {pattern, type_def, expr} => {
            declaration::into_operations(scope, pattern, type_def, expr)
        },
        Assignment {lhs, expr} => {
            assignment::into_operations(scope, lhs, expr)
        },
        WhileLoop {condition, body} => {
            while_loop::into_operations(scope, condition, body)
        },
        Expression {expr} => {
            let unit_type = scope.primitives().unit();
            expression::into_operations(scope, expr, Target::TypedBlock {
                type_id: unit_type,
                memory: MemoryBlock::default(),
            })
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment() {
        // Make sure comments result in no operations
        let mut scope = ScopeStack::new();
        let ops = into_operations(&mut scope, Statement::Comment("foo".to_string())).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
