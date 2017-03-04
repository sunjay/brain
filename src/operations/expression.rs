use parser::Expression;
use parser::Expression::*;
use memory::MemoryBlock;

use super::OperationsResult;
use super::scope::{TypeId, ScopeStack};

/// Generates operations for evaluating the given expression
/// and storing its result in the given target memory block
/// Providing no memory is ONLY valid if target_type is zero-sized
pub fn into_operations(
    expr: Expression,
    target_type: TypeId,
    target: Option<MemoryBlock>,
    scope: &mut ScopeStack,
) -> OperationsResult {
    match expr {
        //Call {method, args} =>
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
