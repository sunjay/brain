use std::fmt;
use std::iter::{once, repeat, FromIterator};
use std::ops::Index;

use memory::{MemoryLayout, CellIndex, MemSize};
use operations::{Operation, Operations};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn remove(&mut self, index: usize) -> Instruction {
        self.0.remove(index)
    }
}

impl Index<usize> for Instructions {
    type Output = Instruction;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for Instructions {
    type Item = Instruction;
    type IntoIter = ::std::vec::IntoIter<Instruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Instruction> for Instructions {
    fn from_iter<I: IntoIterator<Item=Instruction>>(iter: I) -> Self {
        Instructions(iter.into_iter().collect())
    }
}

impl FromIterator<Instruction> for String {
    fn from_iter<I: IntoIterator<Item=Instruction>>(iter: I) -> Self {
        iter.into_iter().map(|instr| instr.to_string()).collect()
    }
}

impl From<Instructions> for String {
    fn from(instrs: Instructions) -> String {
        instrs.into_iter().collect()
    }
}

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
        f.write_str(match *self {
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

impl From<Operations> for Instructions {
    fn from(ops: Operations) -> Instructions {
        let mut current_cell = 0;
        let mut layout = MemoryLayout::new();

        into_instructions_index(ops, &mut layout, &mut current_cell)
    }
}

fn into_instructions_index(
    ops: Operations,
    layout: &mut MemoryLayout,
    current_cell: &mut CellIndex,
) -> Instructions {
    use self::Operation::*;
    ops.into_iter().flat_map(|op| match op {
        Block {body} | TempAllocate {body, ..} => {
            into_instructions_index(body, layout, current_cell)
        },
        Increment {target, amount} => {
            move_to(current_cell, layout.position(&target)).into_iter()
                .chain(repeat(Instruction::Increment).take(amount as usize)).collect()
        },
        Decrement {target, amount} => {
            move_to(current_cell, layout.position(&target)).into_iter()
                .chain(repeat(Instruction::Decrement).take(amount as usize)).collect()
        },
        Read {target} => {
            move_to(current_cell, layout.position(&target.position())).into_iter()
                .chain(consecutive(vec![Instruction::Read], current_cell, target.size())).collect()
        },
        Write {target} => {
            move_to(current_cell, layout.position(&target.position())).into_iter()
                .chain(consecutive(vec![Instruction::Write], current_cell, target.size())).collect()
        },
        Zero {target} => {
            move_to(current_cell, layout.position(&target.position())).into_iter()
                .chain(consecutive(vec![
                    Instruction::JumpForwardIfZero,
                    Instruction::Decrement,
                    Instruction::JumpBackwardUnlessZero,
                ], current_cell, target.size())).collect()
        },
        Loop {cond, body} => {
            move_to(current_cell, layout.position(&cond)).into_iter()
                .chain(once(Instruction::JumpForwardIfZero))
                .chain(into_instructions_index(body, layout, current_cell))
                .chain(move_to(current_cell, layout.position(&cond)))
                .chain(once(Instruction::JumpBackwardUnlessZero))
                .collect()
        },
        Copy {source, target, size} => {
            debug_assert!(source.associated_memory().size() - source.offset() == size);
            debug_assert!(target.associated_memory().size() - target.offset() == size);

            // Algorithm for copying cells:
            // 1. In a loop, decrement the source cell and increment both the target cell and a
            //    temporary cell until the source cell is zero
            // 2. In a loop, decrement the temporary cell and increment the source cell until the
            //    temporary cell is zero
            // 3. Repeat these steps for each cell to be copied

            let source = layout.position(&source);
            let target = layout.position(&target);
            let temp = layout.temporary(1);
            (0..size).flat_map(|i| {
                move_to(current_cell, source + i).into_iter()
                    // Fill the target and the temporary with the value of the source cell
                    .chain(once(Instruction::JumpForwardIfZero))

                    .chain(move_to(current_cell, target + i))
                    .chain(once(Instruction::Increment))

                    .chain(move_to(current_cell, temp.position()))
                    .chain(once(Instruction::Increment))

                    .chain(move_to(current_cell, source + i))
                    .chain(once(Instruction::Decrement))

                    .chain(once(Instruction::JumpBackwardUnlessZero))

                    // Refill the source cell with the temporary
                    .chain(move_to(current_cell, temp.position()))
                    .chain(once(Instruction::JumpForwardIfZero))

                    .chain(move_to(current_cell, source + i))
                    .chain(once(Instruction::Increment))

                    .chain(move_to(current_cell, temp.position()))
                    .chain(once(Instruction::Decrement))

                    .chain(once(Instruction::JumpBackwardUnlessZero))
            }).collect()
        },
        Relocate {source, target} => {
            debug_assert!(source.size() == target.size());
            let size = source.size();

            let source = layout.position(&source.position());
            let target = layout.position(&target.position());

            (0..size).flat_map(|i| {
                move_to(current_cell, source + i).into_iter()
                    .chain(once(Instruction::JumpForwardIfZero))

                    .chain(move_to(current_cell, target + i))
                    .chain(once(Instruction::Increment))

                    .chain(move_to(current_cell, source + i))
                    .chain(once(Instruction::Decrement))

                    .chain(once(Instruction::JumpBackwardUnlessZero))
            }).collect()
        },
    }).collect()
}

fn move_to(current_cell: &mut CellIndex, target: CellIndex) -> Vec<Instruction> {
    let current = *current_cell;
    *current_cell = target;

    if current > target {
        repeat(Instruction::Left).take(current - target).collect()
    }
    else if target > current {
        repeat(Instruction::Right).take(target - current).collect()
    }
    else {
        Vec::new()
    }
}

fn consecutive(instrs: Vec<Instruction>, current_cell: &mut CellIndex, size: MemSize) -> Vec<Instruction> {
    *current_cell += size;
    (0..size).flat_map(|_| instrs.clone().into_iter().chain(once(Instruction::Right))).collect()
}
