// As specified here: http://www.muppetlabs.com/~breadbox/bf/

use std::convert::Into;

use instruction::Instruction;
use parser::Program;
use codegen;

#[derive(Debug, PartialEq, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    fn new() -> Instructions {
        Instructions(Vec::new())
    }

    pub fn from_program(program: Program) -> Result<Self, ()> {
        let mut instrs = Instructions::new();

        for stmt in program {
            codegen::expand(&mut instrs, stmt);
        }

        Ok(instrs)
    }

    pub fn right(&mut self, n: usize) {
        for _ in 0..n {
            self.0.push(Instruction::Right);
        }
    }

    pub fn left(&mut self, n: usize) {
        for _ in 0..n {
            self.0.push(Instruction::Left);
        }
    }

    pub fn increment(&mut self, n: usize) {
        for _ in 0..n {
            self.0.push(Instruction::Increment);
        }
    }

    pub fn decrement(&mut self, n: usize) {
        for _ in 0..n {
            self.0.push(Instruction::Decrement);
        }
    }

    pub fn write(&mut self, n: usize) {
        for _ in 0..n {
            self.0.push(Instruction::Write);
        }
    }

    pub fn read(&mut self, n: usize) {
        for _ in 0..n {
            self.0.push(Instruction::Read);
        }
    }

    pub fn jump_forward_if_zero(&mut self) {
        self.0.push(Instruction::JumpForwardIfZero);
    }

    pub fn jump_backward_unless_zero(&mut self) {
        self.0.push(Instruction::JumpBackwardUnlessZero);
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
