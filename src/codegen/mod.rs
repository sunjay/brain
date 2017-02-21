//! **THE MOST IMPORTANT RULE:** ALL OPERATIONS MUST RETURN TO THE CELL WHERE THEY STARTED.
//! That means that if you move right by 10, you must move left by 10 at the end of your operation
//! The extra movement instructions will be optimized away as needed
//! This constraint exists because it makes writing code generation for brainfuck sane
//! You don't have to know where the pointer currently is because you can always trust this reference
//! This constraint does not need to hold *during* an operation. Only
//! enforce it before and after. We just need a consistent reference between operations.
//! That is all.

//mod errors;
//mod cells;
//mod statements;
//mod declarations;
//mod input;
//mod output;
//mod if_condition;
//mod while_loop;
//
//pub use self::errors::*;
//pub use self::statements::*;

//TODO: Remove everything below this comment
use parser::{Statement};
use instructions::Instructions;
use memory::MemoryLayout;

#[derive(Debug, PartialEq, Clone)]
pub struct Error;
pub fn expand(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    stmt: Statement
) -> Result<(), Error> {
    unimplemented!();
}
