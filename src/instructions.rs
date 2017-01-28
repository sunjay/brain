// As specified here: http://www.muppetlabs.com/~breadbox/bf/

use std::iter;
use std::convert::Into;
use std::ops::Index;

use instruction::Instruction;
use parser::Program;
use memory::MemoryLayout;
use optimizations::{OptimizationLevel, apply_optimizations};
use codegen;

#[derive(Debug, PartialEq, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    fn new() -> Instructions {
        Instructions(Vec::new())
    }

    /// Generate instructions from the given program syntax tree
    pub fn from_program(program: Program) -> Result<Self, codegen::Error> {
        let mut instrs = Instructions::new();
        let mut mem = MemoryLayout::new();

        for stmt in program {
            codegen::expand(&mut instrs, &mut mem, stmt)?;
        }

        Ok(instrs)
    }

    /// Optimize the instructions based on the given optimization level
    pub fn optimize(&mut self, level: OptimizationLevel) {
        apply_optimizations(self, level);
    }

    /// The number of instructions in this instructions set
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Add instructions that move from a given offset to the other offset
    /// using the fewest instructions possible
    pub fn move_relative(&mut self, from: usize, to: usize) {
        if to > from {
            self.move_right_by(to - from);
        }
        else if from > to {
            self.move_left_by(from - to);
        }
    }

    /// Add an instruction to move one cell to the right
    pub fn move_right(&mut self) {
        self.0.push(Instruction::Right);
    }

    /// Add instructions that move n cells to the right
    pub fn move_right_by(&mut self, n: usize) {
        self.0.extend(iter::repeat(Instruction::Right).take(n));
    }

    /// Add an instruction to move one cell to the left
    pub fn move_left(&mut self) {
        self.0.push(Instruction::Left);
    }

    /// Add instructions that move n cells to the left
    pub fn move_left_by(&mut self, n: usize) {
        self.0.extend(iter::repeat(Instruction::Left).take(n));
    }

    /// Adds instructions that increment/decrement the current cell from the given value
    /// to the other given value
    pub fn increment_relative(&mut self, from: u8, to: u8) {
        if to > from {
            self.increment_by(to - from);
        }
        else if from > to {
            self.decrement_by(from - to);
        }
    }

    /// Add an instruction to increment the current cell once
    pub fn increment(&mut self) {
        self.0.push(Instruction::Increment);
    }

    /// Add instructions that increment the current cell n times
    pub fn increment_by(&mut self, n: u8) {
        self.0.extend(iter::repeat(Instruction::Increment).take(n as usize));
    }

    /// Add an instruction to decrement the current cell once
    pub fn decrement(&mut self) {
        self.0.push(Instruction::Decrement);
    }

    /// Add instructions that decrement the current cell n times
    pub fn decrement_by(&mut self, n: u8) {
        self.0.extend(iter::repeat(Instruction::Decrement).take(n as usize));
    }

    /// Adds instructions which set the cell at the current position to zero regardless of
    /// its current value
    pub fn zero(&mut self) {
        self.jump_forward_if_zero();
        self.decrement();
        self.jump_backward_unless_zero();
    }

    /// Adds instructions which set the next n cells (including the current one) to zero
    /// Example: zero_cells(3) will set the current cell and the next two to zero
    pub fn zero_cells(&mut self, n: usize) {
        for _ in 0..n {
            self.zero();
            self.move_right();
        }

        self.move_left_by(n);
    }

    /// Stores the given bytes in each location starting at the current cell
    /// The pointer ends up at the cell immediately after the last character
    /// written
    /// **IMPORTANT:** Assumes that the current cell and all consecutive cells to be
    /// written into are zero to begin with
    pub fn store_bytes(&mut self, bytes: &[u8]) {
        for &ch in bytes {
            self.increment_by(ch);
            self.move_right();
        }
        // Return back to the reference position
        self.move_left_by(bytes.len());
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
        // Return back to the reference
        self.move_left_by(n - 1);
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

    pub fn remove(&mut self, index: usize) -> Instruction {
        self.0.remove(index)
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

impl Index<usize> for Instructions {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
