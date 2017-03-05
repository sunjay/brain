use parser::{Expression, Identifier, Number};
use memory::MemoryBlock;

use super::{Error, Operation, OperationsResult};
use super::item_type::ItemType;
use super::scope::{TypeId, ScopeStack, ScopeItem, FuncArgs};

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
        Expression::Identifier(name) => store_identifier(scope, name, target_type, target),
        Expression::Number(value) => store_number(scope, value, target_type, target),
        _ => unimplemented!(),
    }
}

/// Call the provided method with the given arguments
pub fn call(scope: &mut ScopeStack, method: Expression, args: FuncArgs) -> OperationsResult {
    unimplemented!();
}

fn store_identifier(
    scope: &mut ScopeStack,
    name: Identifier,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    scope.lookup(&name).first().ok_or_else(|| {
        Error::UnresolvedName(name.clone())
    }).map(|item| (**item).clone()).and_then(|item| match item {
        // There is a non-lexical lifetimes issue here which was introduced by calling store_number() below
        // The clone() above is completely unnecssary and is a hack to work around this problem
        // in the Rust compiler
        // http://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/#problem-case-2-conditional-control-flow

        ScopeItem::Type(..) => Err(Error::UnresolvedName(name)),

        ScopeItem::Constant {type_id, ref bytes} => {
            if target_type == type_id {
                increment_to_value(target, bytes)
            }
            else {
                mismatched_types(scope, target_type, type_id)
            }
        },

        ScopeItem::NumericLiteral(number) => store_number(scope, number, target_type, target),

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

fn store_number(
    scope: &mut ScopeStack,
    value: Number,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    if supports_literal(scope, target_type, "{signed integer}") {
        unreachable!("signed integer literals are not currently supported (https://github.com/brain-lang/brain/issues/11)");
    }
    else if value >= 0 {
        if supports_literal(scope, target_type, "{unsigned integer}") {
            if target.size() != 1 {
                unreachable!("Numeric literals for more than u8 are not supported yet (https://github.com/brain-lang/brain/issues/11)");
            }

            //TODO: Make this code general enough to work for any unsigned literal
            if value >= (1i32 << 8) {
                Err(Error::OverflowingLiteral {
                    typ: scope.get_type(target_type).clone()
                })
            }
            else {
                Ok(vec![
                    Operation::Increment {
                        target: target.position(),
                        amount: value as usize,
                    }
                ])
            }
        }
        else {
            mismatched_literal(scope, target_type, "{unsigned integer}")
        }
    }
    else {
        mismatched_literal(scope, target_type, "{signed integer}")
    }
}

/// Returns true if the target_type supports conversion to the given type of literal
fn supports_literal(scope: &ScopeStack, target_type: TypeId, literal_type: &'static str) -> bool {
    scope.lookup(&Identifier::from(literal_type)).into_iter().any(|x| match *x {
        ScopeItem::BuiltInFunction { id, .. } => {
            match *scope.get_type(id) {
                ItemType::Function { ref args, return_type } => {
                    args.is_empty() && return_type == target_type
                },
                _ => false,
            }
        },
        _ => false,
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

fn mismatched_literal(scope: &ScopeStack, expected: TypeId, found: &'static str) -> OperationsResult {
    Err(Error::MismatchedLiteral {
        expected: scope.get_type(expected).clone(),
        found: found.into(),
    })
}
