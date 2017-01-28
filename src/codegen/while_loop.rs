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
    loop_condition(instructions, mem, &condition)?;

    instructions.jump_forward_if_zero();

    loop_body(instructions, mem, body)?;

    loop_condition(instructions, mem, &condition)?;

    instructions.jump_backward_unless_zero();

    Ok(())
}

pub fn loop_condition(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    condition: &WhileCondition,
) -> Result<(), Error> {
    let size = match condition {
        &WhileCondition::Input {ref name, slice} => {
            // We only need to undeclare if we did the declaration in the loop condition
            if slice.is_some() {
                // Since we're going to read input anyway, we don't need to
                // zero out this cell because it will be completely overwritten in the read
                mem.undeclare(name);
            }

            read_input(instructions, mem, name.clone(), slice)?;
            let (_, size) = mem.get_cell_contents(name).expect("read_input didn't declare name");
            size
        },
        &WhileCondition::Expression(Expression::StringLiteral(_)) => {
            return Err(Error::LoopStringLiteralUnsupported {});
        },
        &WhileCondition::Expression(Expression::Identifier(ref name)) => {
            let (_, size) = mem.get_cell_contents(name).ok_or_else(|| Error::UndeclaredIdentifier {name: name.clone()})?;
            size
        },
    };

    if size != 1 {
        return Err(Error::ConditionSizeInvalid {
            expected: 1,
            actual: size,
        });
    }

    Ok(())
}

fn loop_body(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    body: Vec<Statement>,
) -> Result<(), Error> {
    for stmt in body {
        expand(instructions, mem, stmt)?;
    }

    Ok(())
}
