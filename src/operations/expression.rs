use parser::{Expression, Identifier};
use memory::MemoryBlock;

use super::{Error, Operation, OperationsResult};
use super::scope::{TypeId, ScopeStack, ScopeItem};

/// Generates operations for evaluating the given expression
/// and storing its result in the given target memory block
/// NOTE: Assumes that the target memory block is **zero**
/// so that it can be mutated
pub fn into_operations(
    scope: &mut ScopeStack,
    expr: Expression,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    match expr {
        Expression::Identifier(name) => expr_identifier(scope, name, target_type, target),
        _ => unimplemented!(),
    }
}

fn expr_identifier(
    scope: &mut ScopeStack,
    name: Identifier,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    scope.lookup(&name).first().ok_or_else(|| {
        Error::UnresolvedName(name.clone())
    }).and_then(|item| match **item {
        ScopeItem::Type(..) => Err(Error::UnresolvedName(name)),

        ScopeItem::Constant {type_id, ref bytes} => {
            if target_type == type_id {
                increment_to_value(target, bytes)
            }
            else {
                mismatched_types(scope, target_type, type_id)
            }
        },

        ScopeItem::TypedBlock {type_id, memory} => {
            if target_type == type_id {
                // Need to check this invariant or else this can lead to
                // many very subtle bugs
                debug_assert!(memory.size() == target.size());

                Ok(vec![Operation::Copy {
                    source: memory.position(),
                    target: target.position(),
                    size: memory.size(),
                }])
            }
            else {
                mismatched_types(scope, target_type, type_id)
            }
        },

        ScopeItem::BuiltInFunction { .. } => {
            // This is not supported for now
            unreachable!();
        },
    })
}

fn increment_to_value(mem: MemoryBlock, value: &Vec<u8>) -> OperationsResult {
    debug_assert!(mem.size() == value.len());

    Ok(value.iter().enumerate().map(|(i, &byte)| {
        Operation::Increment {
            target: mem.position_at(i),
            amount: byte as usize,
        }
    }).collect())
}

fn mismatched_types(scope: &ScopeStack, expected: TypeId, found: TypeId) -> OperationsResult {
    Err(Error::MismatchedTypes {
        expected: scope.get_type(expected).clone(),
        found: scope.get_type(found).clone(),
    })
}
