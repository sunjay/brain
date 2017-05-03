use std::iter::once;

use parser::{Identifier, Expression};

use super::{Error};
use super::{Operation, OperationsResult, expression, Target};
use super::scope::{ScopeStack, ScopeItem};

pub fn into_operations(
    scope: &mut ScopeStack,
    lhs: Identifier,
    expr: Expression,
) -> OperationsResult {
    scope.lookup(&lhs).first().ok_or_else(|| {
        Error::UnresolvedName(lhs.clone())
    }).map(|item| (**item).clone()).and_then(|item| match item {
        // There is a non-lexical lifetimes issue here which was introduced by calling into_operations*() below
        // The clone() above is completely unnecssary and is a hack to work around this problem
        // in the Rust compiler
        // http://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/#problem-case-2-conditional-control-flow

        ScopeItem::TypedBlock {type_id, memory} => {
            Ok(once(Operation::Zero {target: memory}).chain(
                expression::into_operations(scope, expr, Target::TypedBlock {type_id, memory})?.into_iter()
            ).collect())
        },
        ScopeItem::Array {item, size, memory} => {
            Ok(once(Operation::Zero {target: memory}).chain(
                expression::into_operations(scope, expr, Target::Array {item, size, memory})?.into_iter()
            ).collect())
        },
        ScopeItem::Constant {..} | ScopeItem::NumericLiteral(..) | ScopeItem::ByteLiteral(..) | ScopeItem::BuiltInFunction {..} => {
            Err(Error::InvalidLeftHandSide(lhs))
        },
    })
}
