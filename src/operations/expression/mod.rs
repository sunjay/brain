mod number;
mod identifier;
mod call;
mod byte_literal;

pub use self::call::call;

use parser::Expression;
use memory::MemoryBlock;

use operations::OperationsResult;
use operations::scope::{TypeId, ScopeStack, ArraySize};

use self::identifier::{store_identifier, store_identifier_array};
use self::number::store_number;
use self::byte_literal::store_byte_literal;
use self::call::call_with_exprs;

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
        Expression::Call {method, args} => call_with_exprs(scope, *method, args, target_type, target),
        _ => unimplemented!(),
    }
}

/// Generates operations for evaluating the given expression
/// and storing its result in the given target memory block
/// that represents an array
/// NOTE: Assumes that the target memory block is **zero**
/// so that it can be mutated
pub fn into_operations_array(
    scope: &mut ScopeStack,
    expr: Expression,
    item_type: TypeId,
    size: ArraySize,
    target: MemoryBlock,
) -> OperationsResult {
    match expr {
        Expression::ByteLiteral(bytes) => store_byte_literal(scope, bytes, item_type, size, target),
        Expression::Identifier(name) => store_identifier_array(scope, name, item_type, size, target),
        _ => unimplemented!(),
    }
}
