use memory::MemoryBlock;

use operations::{Error, Operation, OperationsResult};
use operations::scope::{TypeId, ScopeStack, ArraySize};
use operations::item_type::{ItemType};

pub fn store_byte_literal(
    scope: &mut ScopeStack,
    bytes: Vec<u8>,
    item_type: TypeId,
    size: ArraySize,
    target: MemoryBlock,
) -> OperationsResult {
    let u8_type = scope.primitives().u8();

    if item_type != u8_type || bytes.len() != size {
        return Err(Error::MismatchedTypes {
            expected: ItemType::Array {
                item: Some(u8_type),
                size: Some(bytes.len()),
            },
            found: ItemType::Array {
                item: Some(item_type),
                size: Some(size),
            },
        });
    }

    Ok(Operation::increment_to_value(target, &bytes))
}
