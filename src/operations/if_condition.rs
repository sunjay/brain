use parser::{Expression, Statement};
use instructions::Instructions;
use memory::MemoryLayout;

use super::errors::Error;
use super::cells::copy_cells;
use super::statements::expand;

pub fn if_condition(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    condition: Expression,
    body: Vec<Statement>,
) -> Result<(), Error> {
    let cond_cell = eval_condition_into_cell(instructions, mem, &condition)?;

    // Make sure we're at the right cell for the result of the condition
    instructions.move_right_by(cond_cell);

    instructions.jump_forward_if_zero();
    // Clear the condition cell so we can use it to break the loop
    instructions.zero();
    mem.free(cond_cell, 1);
    instructions.move_left_by(cond_cell);

    for stmt in body {
        expand(instructions, mem, stmt)?;
    }

    // Prevent jumping again
    instructions.move_right_by(cond_cell);
    instructions.jump_backward_unless_zero();
    instructions.move_left_by(cond_cell);

    Ok(())
}

pub fn eval_condition_into_cell(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    condition: &Expression,
) -> Result<usize, Error> {
    let (position, size) = match condition {
        &Expression::StringLiteral(_) => {
            return Err(Error::LoopStringLiteralUnsupported {});
        },
        &Expression::Identifier(ref name) => {
            let (ident_pos, size) = mem.get_cell_contents(name).ok_or_else(|| Error::UndeclaredIdentifier {name: name.clone()})?;

            // We eventually need to clear this cell, so let's copy the value so we don't
            // clear something we weren't expecting to
            //TODO: This copy could be something to optimize away if the name isn't used later
            let temp = mem.allocate(1);
            copy_cells(instructions, mem, ident_pos, temp, size);
            (temp, size)
        },
    };

    // We check for this error here because it doesn't really matter whether we do it before
    // any code generation or not
    if size != 1 {
        return Err(Error::ConditionSizeInvalid {
            expected: 1,
            actual: size,
        });
    }

    Ok(position)
}
