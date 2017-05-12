use memory::{MemoryBlock, MemSize, CellPosition};

pub type Operations = Vec<Operation>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// Denotes a "block" of operations
    ///
    /// Any allocations made in the body will be freed after its last operation.
    Block {
        body: Operations,
    },

    /// The compiler often needs to temporarily allocate and use some memory for what is often a
    /// very short period of time. It can be wasteful to wait until the end of the stack frame to
    /// reuse that memory. This operation makes it possible to optimize the use of memory allocated
    /// to temporary cells.
    ///
    /// Not only does this save on memory, it also saves move instructions. No instructions are
    /// wasted going back and forth over cells that will never be used again.
    TempAllocate {
        /// The memory block used as temporary memory in the provided operations.
        temp: MemoryBlock,
        /// temp should only be used in these operations.
        /// It will be freed afterwards.
        body: Operations,
        // If this is true, temp will be zeroed explicitly using the Zero operation
        // If this is false, we assume that you zeroed temp already before this ran and do nothing
        // other than removing it from the memory layout so the memory can be reused
        should_zero: bool,
    },

    /// Increment the value of the given cell by a certain amount (relative to whatever the
    /// current amount in the cell is)
    Increment {
        target: CellPosition,
        // u8 since cells are byte-sized
        amount: u8,
    },

    /// Decrement the value of the given cell by a certain amount (relative to whatever the
    /// current amount in the cell is)
    Decrement {
        target: CellPosition,
        // u8 since cells are byte-sized
        amount: u8,
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

    /// Executes either if_body or else_body depending on the value of cond_mem
    Branch {
        /// A memory block allocated using the bool primitive
        cond: MemoryBlock,
        /// Operations executed if cond_mem is non-zero (true)
        if_body: Operations,
        /// Operations executed if cond_mem is zero (false)
        else_body: Operations,
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
        size: MemSize,
    },

    /// Relocate the value at the source memory block to the target memory block
    /// leaving only zeros at the source memory block
    /// Both memory blocks must be the same size
    Relocate {
        source: MemoryBlock,
        target: MemoryBlock,
    },
}

impl Operation {
    pub fn increment_to_value(mem: MemoryBlock, value: &[u8]) -> Operations {
        debug_assert!(mem.size() == value.len());

        value.iter().enumerate().map(|(i, &byte)| {
            Operation::Increment {
                target: mem.position_at(i),
                amount: byte,
            }
        }).collect()
    }
}
