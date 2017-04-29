use codegen::Instructions;
use codegen::Instruction::*;

use super::{Optimize, OptimizationLevel};

impl Optimize for Instructions {
    fn optimize(mut self: Instructions, level: OptimizationLevel) -> Instructions {
        // Optimizers should be grouped by the number of assumptions
        // the optimizer makes about the code
        // The fewer assumptions, the lower the applicable optimization level
        // Optimizers should be ordered appropriately so that they
        // do not conflict or contradict each other
        let optimizers: Vec<fn(&mut Instructions)> = match level {
            OptimizationLevel::On => vec![
                trim_no_side_effects,
                remove_opposites,
            ],
            OptimizationLevel::Off => vec![],
        };

        for optimize in optimizers {
            optimize(&mut self);
        }

        self
    }
}

/// Removes instructions from the end of the given instructions which do not have any side effects.
///
/// The idea here is that lots of instructions like +, -, >, <, and any loop consisting of only
/// those instructions do not do anything to the output of the program. Thus, for our purposes at
/// least, it is safe to remove those instructions completely and avoid running them at all.
///
/// Often, the reason these instructions are there in the first place is because the compiler
/// ensures that certain memory cells are "freed" or "zeroed" after their use. There is no reason
/// to do this at the end of a program, so let's get rid of it entirely.
fn trim_no_side_effects(instructions: &mut Instructions) {
    'outer: while let Some(&instr) = instructions.last() {
        match instr {
            Left | Right | Increment | Decrement => {
                instructions.pop();
            },
            // Stop only if this loop can have side effects
            JumpBackwardUnlessZero => {
                let mut forward = 0;
                for (i, &instr) in instructions.iter().enumerate().rev().skip(1) {
                    match instr {
                        // This loop has side effects
                        Read | Write => break 'outer,
                        // This loop has no side effects
                        JumpForwardIfZero => {
                            forward = i;
                            break;
                        },
                        //TODO: We only support one level of search for this right now
                        JumpBackwardUnlessZero => break 'outer,
                        // Not sure yet otherwise
                        Left | Right | Increment | Decrement => {},
                    }
                }

                instructions.truncate(forward);
            },
            // Should only be reached by finding a JumpBackwardUnlessZero instruction first
            JumpForwardIfZero => unreachable!(),
            // Stop at the last side effect in the program
            Read | Write => break,
        }
    }
}

fn remove_opposites(instructions: &mut Instructions) {
    let mut i = 1;
    while i < instructions.len() {
        let prev = instructions[i - 1];
        let current = instructions[i];

        // Cancel out opposites
        match (prev, current) {
            (Left, Right) | (Right, Left) |
            (Increment, Decrement) | (Decrement, Increment) => {
                instructions.remove(i);
                instructions.remove(i - 1);
                i -= 1;
            },

            // Otherwise just move on
            _ => i += 1,
        }
    }
}
