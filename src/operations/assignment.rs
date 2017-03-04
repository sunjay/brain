use parser::{Identifier, Expression};

use super::{Error};
use super::{OperationsResult, expression};
use super::scope::{ScopeStack, ScopeItem};

pub fn into_operations(
    lhs: Identifier,
    expr: Expression,
    scope: &mut ScopeStack,
) -> OperationsResult {
    scope.lookup(&lhs).first().ok_or_else(|| {
        Error::UnresolvedName(lhs.clone())
    }).and_then(|item| match **item {
        ScopeItem::TypedBlock {type_id, memory} => Ok((type_id, memory)),
        _ => Err(Error::InvalidLeftHandSide(lhs)),
    }).and_then(|(type_id, memory)| {
        expression::into_operations(expr, type_id, Some(memory), scope)
    })
}
