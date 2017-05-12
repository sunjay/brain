use std::iter::once;

use parser::{Expression, Block};

use operations::{Operation, OperationsResult, expression, block};
use operations::scope::{ScopeStack};

use super::Target;

pub fn branch(
    scope: &mut ScopeStack,
    condition: Expression,
    body: Block,
    otherwise: Option<Block>,
    target: Target,
) -> OperationsResult {
    let bool_type = scope.primitives().bool();
    let cond = scope.allocate(bool_type);

    let cond_ops = expression::into_operations(scope, condition, Target::TypedBlock {
        type_id: bool_type,
        memory: cond,
    })?;
    let if_body = block::into_operations(scope, body, target)?;
    let else_body = match otherwise {
        Some(else_body) => block::into_operations(scope, else_body, target)?,
        None => Vec::new(),
    };

    Ok(cond_ops.into_iter().chain(once(Operation::Branch {cond, if_body, else_body})).collect())
}
