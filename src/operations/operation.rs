use memory::{MemoryBlock, Size, CellPosition};

pub type Operations = Vec<Operation>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// Denotes a "block" of operations
    /// Any allocations in this block will be freed at the end of the block
    Block {
        body: Operations,
    },

    /// Allocates the given size in bytes on the tape so that it is not used by any other code
    /// accidentally. The addr is generated automatically and represents the position which will
    /// eventually be determined when the memory layout is generated. Nothing is guaranteed about
    /// the position other than that `size` consecutive cells including the position will be
    /// available for use without conflicts. This is also used to ensure memory is automatically
    /// dropped at the end of its scope (stack frame).
    Allocate(MemoryBlock),

    /// While most allocations can be dropped at the end of their scope, temporary cells
    /// should be dropped as soon as possible so that they are available in the memory
    /// layout again as soon as possible
    /// This way we can avoid a lot of unnecessary move operations over cells
    /// that aren't being used anymore
    /// While this could be an optimization as well, temporary cells in particular are
    /// *known* to have this property since we generate temporary cells in the compiler itself
    /// The temporary cells are guaranteed to last for the duration of the given body
    /// They are then freed immediately afterwards
    TempAllocate {
        // The memory block used as temporary memory in the provided operations
        temp: MemoryBlock,
        body: Operations,
    },

    /// Frees the given memory id and all cells associated with it
    /// Typically not used unless an explicit free is necessary before the end of the scope
    /// Freeing means both marking those cells available and zeroing their values
    /// This memory block should not be used after this
    Free(MemoryBlock),

    /// Increment the value of the given cell by a certain amount (relative to whatever the
    /// current amount in the cell is)
    Increment {
        target: CellPosition,
        amount: usize,
    },

    /// Decrement the value of the given cell by a certain amount (relative to whatever the
    /// current amount in the cell is)
    Decrement {
        target: CellPosition,
        amount: usize,
    },

    /// Read bytes into the given memory block
    /// Note: this generates both read instructions and move right instructions
    Read {
        target: MemoryBlock,
    },

    /// Write bytes into the given memory block
    /// Note: this generates both write instructions and move right instructions
    Write {
        target: MemoryBlock,
    },

    /// Set the value of every cell inside the given memory block to zero
    /// Note: this generates instructions to zero the value and move to the right towards each
    /// consecutive cell
    Zero {
        target: MemoryBlock,
    },

    /// Loop with the given operations as the loop body
    /// cond is the cell position that represents the loop condition
    /// This will be moved to before the loop and at the end of the loop body
    /// It is up to the surrounding operations to determine when to evaluate the condition
    Loop {
        /// This field in particular is VERY important because it creates a guarantee about the
        /// position of the cell pointer both before and after a loop
        /// Without this, solving for the current position in order to generate
        /// movement instructions would be nearly impossible
        cond: CellPosition,
        body: Operations,
    },

    /// Copy `size` cells from the source cell to the target cell using a single temporary cell
    /// WARNING: If size is greater than the allocated size of either memory block, this can result
    /// in buffer overrun.
    Copy {
        source: CellPosition,
        target: CellPosition,
        size: Size,
    },

    /// Relocate the value at the source memory block to the target memory block
    /// leaving only zeros at the source memory block
    /// Both memory blocks must be the same size
    Relocate {
        source: MemoryBlock,
        target: MemoryBlock,
    },
}
