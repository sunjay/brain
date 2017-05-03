use parser::{Expression, Block};

use operations::{Error, Operation, OperationsResult, expression, block};
use operations::scope::{TypeId, ScopeStack, ArraySize};

use super::Target;

pub fn branch(
    scope: &mut ScopeStack,
    condition: Expression,
    body: Block,
    otherwise: Option<Block>,
    target: Target,
) -> OperationsResult {
    // Algorithm from: https://esolangs.org/wiki/Brainfuck_algorithms#if_.28x.29_.7B_code1_.7D_else_.7B_code2_.7D
    //
    // temp0 and temp1 are consecutive in memory following cond
    // temp0[-]+
    // temp1[-]
    // cond[
    //  if_block
    //  x>-]>
    // [<
    //  else_block
    //  x>->]<<

    let bool_type = scope.primitives().bool();
    // Three consecutive temporary cells:
    // 1. the boolean result of the condition expression
    // 2. temp0 - used to go into the else block when necessary
    // 3. temp1 - used *not* to go into the else block when necessary
    //
    // The basic idea of the algorithm is that you can control which of two adjacent loops run
    // by "sending" them either temp0 or temp1 based on the condition result
    let temp_cells = scope.allocate_array(bool_type, 3);
    let cond_mem = temp_cells.position_at(0);
    let temp0 = temp_cells.position_at(1);
    let temp1 = temp_cells.position_at(2);

    // Probably shouldn't be using associated_memory() here. It's only safe because cond_mem is at
    // the beginning of the MemoryBlock. This relies on internal implementation details of
    // into_operations--which is an extremely brittle way of doing this.
    let cond_ops = expression::into_operations(scope, condition, Target::TypedBlock {
        type_id: bool_type,
        memory: cond_mem.associated_memory()
    })?;
    let if_block = unimplemented!();
}
