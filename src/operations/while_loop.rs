use std::iter::once;

use parser::{Expression, Block};
use memory::MemoryBlock;

use super::{Operation, OperationsResult, expression, Target, block};
use super::scope::ScopeStack;

pub fn into_operations(
    scope: &mut ScopeStack,
    condition: Expression,
    body: Block,
) -> OperationsResult {
    let unit_type = scope.primitives().unit();
    let bool_type = scope.primitives().bool();
    let cond_mem = scope.allocate(bool_type);

    let cond_ops = expression::into_operations(scope, condition, Target::TypedBlock {
        type_id: bool_type,
        memory: cond_mem,
    })?;
    let loop_body = block::into_operations(scope, body, Target::TypedBlock {
        type_id: unit_type,
        memory: MemoryBlock::default(),
    })?;

    // While loops need to evaluate the condition both before the loop and at the end
    // of the loop body
    let mut ops = Vec::new();
    ops.extend(cond_ops.clone());
    ops.push(Operation::Loop {
        cond: cond_mem.position(),
        body: loop_body.into_iter().chain(once(Operation::Zero {
            target: cond_mem,
        })).chain(cond_ops).collect(),
    });

    Ok(vec![Operation::TempAllocate {
        temp: cond_mem,
        body: ops,
        should_zero: true,
    }])
}
