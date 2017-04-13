use parser::{Identifier, Number};
use memory::MemoryBlock;

use operations::{Error, OperationsResult};
use operations::item_type::{ItemType};
use operations::scope::{TypeId, ScopeStack, ScopeType, ScopeItem};

use super::call;

pub fn store_number(
    scope: &mut ScopeStack,
    value: Number,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    store_numeric_literal(scope, value, target_type, target, "{signed integer}").or_else(|err| {
        if value >= 0 {
            store_numeric_literal(scope, value, target_type, target, "{unsigned integer}")
        }
        else {
            Err(err)
        }
    })
}

/// Attempts to store a specific type of numeric literal
fn store_numeric_literal(
    scope: &mut ScopeStack,
    value: Number,
    target_type: TypeId,
    target: MemoryBlock,
    literal_type: &'static str,
) -> OperationsResult {
    let converter_name = Identifier::from(&*format!("std::convert::From<{}>", literal_type));

    let u8_type = scope.primitives().u8();

    let mut operations = None;
    for item in scope.lookup(&converter_name) {
        operations = match *item {
            ScopeItem::BuiltInFunction { type_id, ref operations } => {
                match *scope.get_type(type_id) {
                    ItemType::Function { ref args, return_type } => {
                        if args.len() == 1 && args[0].is_array_of(u8_type) && return_type == target_type {
                            Some(operations.clone())
                        }
                        else {
                            None
                        }
                    },
                    _ => unreachable!("A literal converter was defined that was not a Function"),
                }
            },
            _ => unreachable!("A literal converter was defined that was not a BuiltInFunction"),
        };
    }

    operations.ok_or_else(|| {
        Error::MismatchedLiteral {
            expected: scope.get_type(target_type).clone(),
            found: literal_type.into(),
        }
    }).and_then(|operations| {
        operations(scope, vec![ScopeItem::NumericLiteral(value)], target)
    })
}
