use parser::{Expression, CallArgs};
use memory::MemoryBlock;

use operations::{Error, OperationsResult};
use operations::scope::{TypeId, ScopeStack, ScopeItem, FuncArgs};

/// Evaluates the arguments first, then supplies them to the given method
pub fn call_with_exprs(
    scope: &mut ScopeStack,
    method: Expression,
    arg_exprs: CallArgs,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    let (args, ops): (Vec<_>, Vec<_>) = arg_exprs.into_iter().map(|expr| match expr {
        Expression::ByteLiteral(bytes) => Ok((ScopeItem::ByteLiteral(bytes), Vec::new())),
        Expression::Number(bytes) => Ok((ScopeItem::NumericLiteral(bytes), Vec::new())),
        Expression::Identifier(name) => scope.lookup(&name).first().ok_or_else(|| {
            Error::UnresolvedName(name.clone())
        }).map(|item| ((**item).clone(), Vec::new())),
        Expression::Call {..} => unimplemented!(),
        Expression::Access {..} => unimplemented!(),
        Expression::Branch {..} => unimplemented!(),
    }).collect::<Result<Vec<_>, _>>()?.into_iter().unzip();

    Ok(ops.into_iter().flat_map(|o| o.into_iter()).chain(
        call(scope, method, args, target_type, target)?
    ).collect())
}

/// Call the provided method with the given arguments
pub fn call(
    scope: &mut ScopeStack,
    method: Expression,
    args: FuncArgs,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    //TODO: Refactor store_number to use this
    unimplemented!();
}
