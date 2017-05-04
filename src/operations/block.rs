use parser::{Statement, Block};

use super::{Operation, OperationsResult, statement, expression, Target};
use super::scope::ScopeStack;

pub fn into_operations(scope: &mut ScopeStack, mut block: Block, target: Target) -> OperationsResult {
    scope.push_scope();

    // The last statement in a block is always used for the return type of the block
    // This works because we automatically insert a UnitLiteral at the end of blocks terminated
    // by a semicolon in the parser
    let last = block.pop().expect("The parser did not fulfill its guarantee of a last statement");

    let mut ops = Vec::new();

    for stmt in block.into_iter() {
        ops.extend(statement::into_operations(scope, stmt)?);
    }

    if let Statement::Expression {expr} = last {
        ops.extend(expression::into_operations(scope, expr, target)?);
    }
    else {
        // The parser guarantees that the last statement will always be an expression
        unreachable!();
    }

    scope.pop_scope();

    Ok(vec![Operation::Block {
        body: ops,
    }])
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::{Statement, Expression};
    use memory::MemoryBlock;

    #[test]
    fn empty_block() {
        let mut scope = ScopeStack::new();
        let unit_type = scope.primitives().unit();
        let block = vec![Statement::Expression {expr: Expression::UnitLiteral}];

        let ops = into_operations(&mut scope, block, Target::TypedBlock {
            type_id: unit_type,
            memory: MemoryBlock::default(),
        }).unwrap();
        assert_eq!(ops.len(), 1);
    }

    #[test]
    #[ignore]
    fn nested_scopes() {
        //let mut scope = ScopeStack::new();
        //let block: Block = vec![
        //    //TODO: Test something like this
        //    // {
        //    //     {
        //    //          let foo: u8 = 5;
        //    //     }
        //    //     // This should fail:
        //    //     foo
        //    // }
        //];
        //
        //let ops = into_operations(&mut scope, block).unwrap();
        //assert_eq!(ops.len(), 1);
    }
}
