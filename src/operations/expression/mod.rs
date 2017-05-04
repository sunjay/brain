mod number;
mod identifier;
mod call;
mod byte_literal;
mod branch;

pub use self::call::call;

use parser::Expression;
use memory::MemoryBlock;

use operations::OperationsResult;
use operations::scope::{TypeId, ScopeStack, ArraySize};

use self::identifier::{store_identifier};
use self::number::store_number;
use self::byte_literal::store_byte_literal;
use self::call::call_with_exprs;
use self::branch::branch;

#[derive(Debug, Clone, Copy)]
pub enum Target {
    TypedBlock {
        type_id: TypeId,
        memory: MemoryBlock,
    },
    Array {
        item: TypeId,
        size: ArraySize,
        memory: MemoryBlock,
    },
}

/// Generates operations for evaluating the given expression
/// and storing its result in the given target memory block
/// NOTE: Assumes that the target memory block is **zero**
/// so that it can be mutated
pub fn into_operations(
    scope: &mut ScopeStack,
    expr: Expression,
    target: Target,
) -> OperationsResult {
    match expr {
        Expression::UnitLiteral => Ok(Vec::new()),
        Expression::Identifier(name) => store_identifier(scope, name, target),
        Expression::Number(value) => store_number(scope, value, target),
        Expression::Call {method, args} => call_with_exprs(scope, *method, args, target),
        Expression::ByteLiteral(ref bytes) => store_byte_literal(scope, bytes, target),
        Expression::Branch {condition, body, otherwise} => {
            branch(scope, *condition, body, otherwise, target)
        },
        _ => unimplemented!(),
    }
}
