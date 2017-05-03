use parser::{Identifier, Number};
use memory::MemoryBlock;

use operations::{Error, OperationsResult};
use operations::item_type::{ItemType};
use operations::scope::{TypeId, ScopeStack, ScopeItem};

use super::{Target, call};

pub fn store_number(
    scope: &mut ScopeStack,
    value: Number,
    target: Target,
) -> OperationsResult {
    match target {
        Target::TypedBlock {type_id, memory} => {
            store_numeric_literal(scope, value, type_id, memory, "{signed integer}").or_else(|err| {
                if value >= 0 {
                    store_numeric_literal(scope, value, type_id, memory, "{unsigned integer}")
                }
                else {
                    Err(err)
                }
            })
        },

        Target::Array {item, size, ..} => Err(Error::MismatchedTypes {
            expected: ItemType::Array {
                item: Some(item),
                size: Some(size),
            },
            //TODO: Update this when more numeric types are added
            found: scope.get_type(scope.primitives().u8()).clone(),
        }),
    }
}

/// Attempts to store a specific type of numeric literal
fn store_numeric_literal(
    scope: &mut ScopeStack,
    value: Number,
    target_type: TypeId,
    target_memory: MemoryBlock,
    literal_type: &'static str,
) -> OperationsResult {
    let converter_name = Identifier::from(format!("std::convert::From<{}>", literal_type).as_str());

    call(
        scope,
        converter_name.clone(),
        vec![ScopeItem::NumericLiteral(value)],
        Target::TypedBlock {type_id: target_type, memory: target_memory},
    ).map_err(|err| match err {
        // No literal converter defined, so the literal must not match the type
        Error::UnresolvedName(ref name) if *name == converter_name => {
            Error::MismatchedLiteral {
                expected: scope.get_type(target_type).clone(),
                found: literal_type.into(),
            }
        },
        Error::MismatchedTypes {found, ..} => match found {
            ItemType::Function {..} => unreachable!("A literal converter was defined but did not have the correct type"),
            _ => unreachable!("A literal converter was defined that was not a function"),
        },
        err => err,
    })
}
