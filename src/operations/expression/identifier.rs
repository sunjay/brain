use parser::Identifier;
use memory::MemoryBlock;

use operations::{Error, Operation, OperationsResult};
use operations::item_type::ItemType;
use operations::scope::{TypeId, ScopeStack, ScopeItem, ArraySize};

use super::Target;
use super::number::store_number;
use super::byte_literal::store_byte_literal;

pub fn store_identifier(
    scope: &mut ScopeStack,
    name: Identifier,
    target: Target,
) -> OperationsResult {
    scope.lookup(&name).first().ok_or_else(|| {
        Error::UnresolvedName(name.clone())
    }).map(|item| (**item).clone()).and_then(|item| match item {
        // There is a non-lexical lifetimes issue here which was introduced by calling store_number() below
        // The clone() above is completely unnecssary and is a hack to work around this problem
        // in the Rust compiler
        // http://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/#problem-case-2-conditional-control-flow

        ScopeItem::Constant {type_id, ref bytes} => store_identifier_constant(scope, type_id, bytes, target),
        ScopeItem::NumericLiteral(value) => store_number(scope, value, target),
        ScopeItem::ByteLiteral(ref bytes) => store_byte_literal(scope, bytes, target),
        ScopeItem::TypedBlock {type_id, memory} => store_identifier_typed_block(scope, type_id, memory, target),
        ScopeItem::Array {item, size, memory} => store_identifier_array(scope, item, size, memory, target),
        ScopeItem::BuiltInFunction { .. } => {
            // This is not supported yet in the syntax so it should be unreachable
            unreachable!();
        },
    })
}

fn store_identifier_constant(
    scope: &mut ScopeStack,
    source_type: TypeId,
    bytes: &[u8],
    target: Target,
) -> OperationsResult {
    match target {
        Target::TypedBlock {type_id, memory} => {
            if source_type == type_id {
                Ok(Operation::increment_to_value(memory, bytes))
            }
            else {
                mismatched_types(scope, type_id, source_type)
            }
        },

        Target::Array {item, size, ..} => Err(Error::MismatchedTypes {
            expected: ItemType::Array {
                item: Some(item),
                size: Some(size),
            },
            found: scope.get_type(source_type).clone(),
        }),
    }
}

fn store_identifier_typed_block(
    scope: &mut ScopeStack,
    source_type: TypeId,
    source_memory: MemoryBlock,
    target: Target,
) -> OperationsResult {
    match target {
        Target::TypedBlock {type_id, memory} => {
            if type_id == source_type {
                // Need to check this invariant or else this can lead to
                // many very subtle bugs
                debug_assert!(source_memory.size() == memory.size());

                Ok(vec![Operation::Copy {
                    source: source_memory.position(),
                    target: memory.position(),
                    size: memory.size(),
                }])
            }
            else {
                mismatched_types(scope, type_id, source_type)
            }
        },

        Target::Array {item, size, ..} => Err(Error::MismatchedTypes {
            expected: ItemType::Array {
                item: Some(item),
                size: Some(size),
            },
            found: scope.get_type(source_type).clone(),
        }),
    }
}

fn store_identifier_array(
    scope: &mut ScopeStack,
    source_item: TypeId,
    source_size: ArraySize,
    source_memory: MemoryBlock,
    target: Target,
) -> OperationsResult {
    match target {
        Target::TypedBlock {type_id, ..} => Err(Error::MismatchedTypes {
            expected: scope.get_type(type_id).clone(),
            found: ItemType::Array {
                item: Some(source_item),
                size: Some(source_size),
            },
        }),

        Target::Array {item, size, memory} => {
             if item == source_item && size == source_size {
                 // Need to check this invariant or else this can lead to
                 // many very subtle bugs
                 debug_assert!(source_memory.size() == memory.size());

                 Ok(vec![Operation::Copy {
                     source: source_memory.position(),
                     target: memory.position(),
                     size: memory.size(),
                 }])
             }
             else {
                 Err(Error::MismatchedTypes {
                     expected: ItemType::Array {
                         item: Some(item),
                         size: Some(size),
                     },
                     found: ItemType::Array {
                         item: Some(source_item),
                         size: Some(source_size),
                     },
                 })
             }
        },
    }
}

fn mismatched_types(scope: &ScopeStack, expected: TypeId, found: TypeId) -> OperationsResult {
    Err(Error::MismatchedTypes {
        expected: scope.get_type(expected).clone(),
        found: scope.get_type(found).clone(),
    })
}
