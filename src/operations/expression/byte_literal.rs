use memory::MemoryBlock;

use operations::{Error, Operation, OperationsResult};
use operations::scope::{TypeId, ScopeStack, ArraySize};

pub fn store_byte_literal(
    scope: &mut ScopeStack,
    bytes: Vec<u8>,
    item_type: TypeId,
    size: ArraySize,
    target: MemoryBlock,
) -> OperationsResult {
    unimplemented!();
}
