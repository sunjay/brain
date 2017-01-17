// As specified here: http://www.muppetlabs.com/~breadbox/bf/

use std::fmt;
use std::convert::Into;

use parser::Program;

#[derive(Debug, PartialEq, Clone)]
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
        write!(f, "{}", match self {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn from_program(program: Program) -> Result<Self, ()> {
        let mut instrs = Vec::new();

        //TODO
        instrs.push(Instruction::Right);

        Ok(Instructions(instrs))
    }
}

impl Into<String> for Instructions {
    fn into(self) -> String {
        self.into_iter().fold(String::new(), |acc, instr| acc + &instr.to_string())
    }
}

impl IntoIterator for Instructions {
    type Item = Instruction;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
