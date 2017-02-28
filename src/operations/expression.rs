use parser::{Expression};
use memory::MemoryBlock;

use super::{Operation};
use super::scope::ScopeStack;
use super::item_type::ItemType;

/// Generates operations for evaluating the given expression
/// and storing its result in the given target memory block
pub fn into_operations(
    expr: Expression,
    target_type: &ItemType,
    target: MemoryBlock,
    scope: &mut ScopeStack,
) -> Vec<Operation> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
}
