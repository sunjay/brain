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
        let optimizers = match level {
            OptimizationLevel::Off => vec![],
            OptimizationLevel::On => vec![
                remove_opposites,
            ],
        };

        for optimize in optimizers {
            optimize(&mut self);
        }

        self
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
