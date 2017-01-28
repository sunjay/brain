use parser::{Statement};
use instructions::Instructions;
use memory::MemoryLayout;

use super::errors::Error;
use super::output::output_expr;
use super::input::read_input;
use super::declarations::declare;
use super::while_loop::while_loop;

/// Expands the given statement into instructions
pub fn expand(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    stmt: Statement
) -> Result<(), Error> {
    match stmt {
        Statement::Comment(_) => Ok(()),
        Statement::Output(exprs) => {
            for expr in exprs {
                output_expr(instructions, mem, expr)?;
            }
            Ok(())
        },
        Statement::Input {name, slice} => read_input(instructions, mem, name, slice),
        Statement::Declaration {name, slice, expr} => declare(instructions, mem, name, slice, expr),
        Statement::WhileLoop {condition, body} => while_loop(instructions, mem, condition, body),
    }
}
