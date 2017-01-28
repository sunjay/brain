//! **THE MOST IMPORTANT RULE:** ALL OPERATIONS MUST RETURN TO THE CELL WHERE THEY STARTED.
//! That means that if you move right by 10, you must move left by 10 at the end of your operation
//! The extra movement instructions will be optimized away as needed
//! This constraint exists because it makes writing code generation for brainfuck sane
//! You don't have to know where the pointer currently is because you can always trust this reference
//! This constraint does not need to hold *during* an operation. Only
//! enforce it before and after. We just need a consistent reference between operations.
//! That is all.

mod declarations;
mod input;
mod output;
mod errors;


use parser::{Statement};
use instructions::Instructions;
use memory::MemoryLayout;

use self::output::output_expr;
use self::input::read_input;
use self::declarations::declare;
pub use self::errors::*;


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
        Statement::WhileLoop {condition, body} => {
            println!("{:?}", (condition, body));
            Ok(())
        },
    }
}
