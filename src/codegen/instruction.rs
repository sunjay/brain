use std::fmt;

use memory::{MemoryLayout, CellIndex};
use operations::{Operation, Operations};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instructions(Vec<Instruction>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    // ">" - increment the pointer (move it to the "right")
    Right,
    // "<" - decrement the pointer (move it to the "left")
    Left,
    // "+" - increment the byte at the pointer
    Increment,
    // "-" - decrement the byte at the pointer
    Decrement,
    // "." - output the byte at the pointer
    Write,
    // "," - input a byte and store it in the byte at the pointer
    Read,
    // "[" - jump forward past the matching ] if the byte at the pointer is zero
    JumpForwardIfZero,
    // "]" - jump backward to the matching [ unless the byte at the pointer is zero
    JumpBackwardUnlessZero,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        write!(f, "{}", match *self {
            Right => ">",
            Left => "<",
            Increment => "+",
            Decrement => "-",
            Write => ".",
            Read => ",",
            JumpForwardIfZero => "[",
            JumpBackwardUnlessZero => "]",
        })
    }
}

impl From<(Operations, MemoryLayout)> for Instructions {
    fn from((ops, layout): (Operations, MemoryLayout)) -> Instructions {
        let mut current_cell = 0;

        into_instructions_index(ops, layout, &mut current_cell)
    }
}

fn into_instructions_index(
    ops: Operations,
    layout: MemoryLayout,
    current_cell: &mut CellIndex,
) -> Instructions {
    unimplemented!();
}
