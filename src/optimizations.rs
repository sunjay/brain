use instructions::Instructions;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OptimizationLevel {
    // May be room for more optimizations later
    Off,
    On,
}

pub fn apply_optimizations(instructions: &mut Instructions, level: OptimizationLevel) {
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
        optimize(instructions);
    }
}

fn remove_opposites(instructions: &mut Instructions) {
    //TODO
}
