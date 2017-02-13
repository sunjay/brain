use parser::{Expression, WhileCondition, Statement};
use instructions::Instructions;
use memory::MemoryLayout;

use super::errors::Error;
use super::statements::expand;
use super::input::read_input;

pub fn while_loop(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    condition: WhileCondition,
    body: Vec<Statement>,
) -> Result<(), Error> {
    let cond_cell = loop_condition(instructions, mem, &condition)?;

    // Make sure we're at the right cell for the result of the condition
    instructions.move_right_by(cond_cell);
    instructions.jump_forward_if_zero();
    instructions.move_left_by(cond_cell);

    for stmt in body {
        expand(instructions, mem, stmt)?;
    }

    let cond_cell = loop_condition(instructions, mem, &condition)?;

    // Make sure we're at the right cell for the result of the condition
    instructions.move_right_by(cond_cell);
    instructions.jump_backward_unless_zero();
    instructions.move_left_by(cond_cell);

    Ok(())
}

pub fn loop_condition(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    condition: &WhileCondition,
) -> Result<usize, Error> {
    let (position, size) = match condition {
        &WhileCondition::Input {ref name, slice} => {
            // We only need to undeclare if we did the declaration in the loop condition
            if slice.is_some() {
                // Since we're going to read input anyway, we don't need to
                // zero out this cell because it will be completely overwritten in the read
                mem.undeclare(name);
            }

            read_input(instructions, mem, name.clone(), slice)?;
            mem.get_cell_contents(name).expect("read_input didn't declare name")
        },

        &WhileCondition::Expression(Expression::StringLiteral(_)) => {
            return Err(Error::LoopStringLiteralUnsupported {});
        },

        &WhileCondition::Expression(Expression::Identifier(ref name)) => {
            mem.get_cell_contents(name).ok_or_else(|| Error::UndeclaredIdentifier {name: name.clone()})?
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
