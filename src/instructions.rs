// As specified here: http://www.muppetlabs.com/~breadbox/bf/

use std::convert::Into;
use std::iter;

use instruction::Instruction;
use parser::Program;
use codegen;

#[derive(Debug, PartialEq, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    fn new() -> Instructions {
        Instructions(Vec::new())
    }

    /// Generate instructions from the given program syntax tree
    pub fn from_program(program: Program) -> Result<Self, ()> {
        let mut instrs = Instructions::new();

        for stmt in program {
            codegen::expand(&mut instrs, stmt);
        }

        Ok(instrs)
    }

    /// Add an instruction to move one cell to the right
    pub fn move_right(&mut self) {
        self.0.push(Instruction::Right);
    }

    /// Add an instruction to move one cell to the left
    pub fn move_left(&mut self) {
        self.0.push(Instruction::Left);
    }

    /// Add an instruction to increment the current cell once
    pub fn increment(&mut self) {
        self.0.push(Instruction::Increment);
    }

    /// Add instructions that increment the current cell n times
    pub fn increment_by(&mut self, n: usize) {
        self.0.extend(iter::repeat(Instruction::Increment).take(n));
    }

    /// Add an instruction to decrement the current cell once
    pub fn decrement(&mut self) {
        self.0.push(Instruction::Decrement);
    }

    /// Add instructions that decrement the current cell n times
    pub fn decrement_by(&mut self, n: usize) {
        self.0.extend(iter::repeat(Instruction::Decrement).take(n));
    }

    /// Add an instruction that will write the current cell
    pub fn write(&mut self) {
        self.0.push(Instruction::Write);
    }

    /// Add instructions that will write the next n consecutive cells
    /// starting at the current cell
    pub fn write_consecutive(&mut self, n: usize) {
        let write_next = [Instruction::Write, Instruction::Right];
        // by putting -1 here, we don't move to the right after writing n times
        self.0.extend(write_next.iter().cycle().take(n * write_next.len() - 1));
    }

    /// Add an instruction that will read a single byte into the current cell
    pub fn read(&mut self) {
        self.0.push(Instruction::Read);
    }

    /// Add instructions that will read input into the next n consecutive cells
    /// starting at the current cell
    pub fn read_consecutive(&mut self, n: usize) {
        let read_next = [Instruction::Read, Instruction::Right];
        // by putting -1 here, we don't move to the right after reading n times
        self.0.extend(read_next.iter().cycle().take(n * read_next.len() - 1));
    }

    /// Add an instruction which will only jump forward to the matching
    /// jump backward instruction if the current cell is zero
    pub fn jump_forward_if_zero(&mut self) {
        self.0.push(Instruction::JumpForwardIfZero);
    }

    /// Add an instruction which will only jump backward to the previous
    /// matching jump forward instruction if the current cell is not zero
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
