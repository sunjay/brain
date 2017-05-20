use std::collections::VecDeque;

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
            OptimizationLevel::Off => vec![],
            OptimizationLevel::L1 => vec![
                remove_opposites,
            ],
            OptimizationLevel::L2 => vec![
                truncate_no_side_effects,
                remove_opposites,
            ],
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
fn truncate_no_side_effects(instructions: &mut Instructions) {
    // In this algorithm, we're going to search for the last instruction that has a side effect,
    // and remove (truncate) everything past it
    let mut last_side_effect = None;
    // If we find a side effect inside a loop, we want to leave the loop intact
    let mut jump_stack = VecDeque::new();

    for (i, &instr) in instructions.iter().enumerate().rev() {
        match instr {
            // no side effects
            Left | Right | Increment | Decrement => {},
            // side effects
            Read | Write => {
                last_side_effect = Some(i);
                break;
            },
            JumpBackwardUnlessZero => jump_stack.push_back(i),
            JumpForwardIfZero => {jump_stack.pop_back().unwrap();},
        }
    }

    if let Some(last_side_effect) = last_side_effect {
        if jump_stack.is_empty() {
            instructions.truncate(last_side_effect + 1);
        }
        else {
            instructions.truncate(jump_stack.front().unwrap() + 1);
        }
    }
    else {
        // no side effects, so the instructions may as well not be there at all
        //TODO: Determine if we should actually be doing this?
        //TODO: Maybe it would be better to just leave them as is if there are no side effects
        //TODO: Even if that means that nothing can possibly be read or written at runtime
        instructions.clear();
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

        // Sometimes this can result in getting back to the start
        if i == 0 {
            i = 1;
        }
    }
}
