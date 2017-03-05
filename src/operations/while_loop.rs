use parser::{Expression, Block};

use super::{Error};
use super::{Operation, OperationsResult, expression, block};
use super::scope::{ScopeStack, ScopeItem};

pub fn into_operations(
    scope: &mut ScopeStack,
    condition: Expression,
    body: Block,
) -> OperationsResult {
    let bool_type = scope.bool_type_id();
    let cond_mem = scope.allocate(bool_type);

    let cond_ops = expression::into_operations(scope, condition, bool_type, Some(cond_mem))?;
    let loop_body = block::into_operations(scope, body)?;

    // While loops need to evaluate the condition both before the loop and at the end
    // of the loop body
    let mut ops = Vec::new();
    ops.extend(cond_ops.clone());
    ops.push(Operation::Loop {
        cond: cond_mem.position(),
        body: loop_body.into_iter().chain(cond_ops).collect(),
    });

    Ok(vec![Operation::TempAllocate {
        temp: cond_mem,
        body: ops,
    }])
}
