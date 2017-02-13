use instructions::Instructions;
use memory::MemoryLayout;

/// Generates brainfuck instructions to copy `size` cells from
/// the source position to the target position
pub fn copy_cells(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    source: usize,
    target: usize,
    size: usize
) {
    // We need a hold cell to temporarily hold the value of the source
    // while we move it to the target
    // Once that initial move is done, we move the value of the hold cell back
    // to the source
    // These two moves with a temporary cell simulate a copy in brainfuck
    let hold = mem.next_available_cell();

    // Since size can be more than u8, we need to generate instructions for every cell
    // in a loop like this. We can't just store size in a cell and then use it to do these
    // instructions in a loop
    for i in 0..size {
        instructions.move_right_by(source + i);

        instructions.jump_forward_if_zero();
        instructions.decrement();

        //TODO: This could be a source of optimization since we're potentially
        //TODO: doing extra movement instructions we don't need to
        instructions.move_relative(source + i, hold);
        instructions.increment();

        instructions.move_relative(hold, target + i);
        instructions.increment();

        instructions.move_relative(target + i, source + i);
        instructions.jump_backward_unless_zero();

        // Move from hold back to source leaving everything as it was with
        // source copied into target
        // hold is zero again at the end of this process
        instructions.move_relative(source + i, hold);
        instructions.jump_forward_if_zero();

        instructions.decrement();
        instructions.move_relative(hold, source + i);
        instructions.increment();
        instructions.move_relative(source + i, hold);

        instructions.jump_backward_unless_zero();

        // Return to the starting position
        instructions.move_left_by(hold);
    }
}
