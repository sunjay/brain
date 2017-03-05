use parser::Block;

use super::{OperationsResult, statement};
use super::scope::ScopeStack;

pub fn into_operations(scope: &mut ScopeStack, block: Block) -> OperationsResult {
    scope.push_scope();

    let mut ops = Vec::new();

    for stmt in block.into_iter() {
        ops.extend(statement::into_operations(scope, stmt)?);
    }

    scope.pop_scope();

    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Block;

    #[test]
    fn empty_program() {
        let mut scope = ScopeStack::new();
        let block = Block::new();

        let ops = into_operations(&mut scope, block).unwrap();
        assert_eq!(ops.len(), 0);
    }

    #[test]
    #[ignore]
    fn nested_scopes() {
        let mut scope = ScopeStack::new();
        let block: Block = vec![
            //TODO: Test something like this
            // {
            //     {
            //          let foo: u8 = 5;
            //     }
            //     // This should fail:
            //     foo
            // }
        ];

        let ops = into_operations(&mut scope, block).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
