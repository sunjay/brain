use operations::{Error, Operation, OperationsResult};
use operations::scope::ScopeStack;
use operations::item_type::{ItemType};

use super::Target;

pub fn store_byte_literal(
    scope: &mut ScopeStack,
    bytes: &[u8],
    target: Target,
) -> OperationsResult {
    match target {
        Target::TypedBlock {type_id, ..} => Err(Error::MismatchedTypes {
            expected: scope.get_type(type_id).clone(),
            found: ItemType::Array {
                item: Some(scope.primitives().u8()),
                size: Some(bytes.len()),
            },
        }),

        Target::Array {item, size, memory} => {
            let u8_type = scope.primitives().u8();

            if item != u8_type || bytes.len() != size {
                return Err(Error::MismatchedTypes {
                    expected: ItemType::Array {
                        item: Some(item),
                        size: Some(size),
                    },
                    found: ItemType::Array {
                        item: Some(u8_type),
                        size: Some(bytes.len()),
                    },
                });
            }

            Ok(Operation::increment_to_value(memory, bytes))
        },
    }

}
