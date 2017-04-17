// Exposes no public constructor since MemoryBlocks cannot be created outside of this static allocator
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MemId(usize);

/// Size of a memory block in cells
/// One cell is typically equivalent to a byte, however that depends on the implementation of
/// the brainfuck interpreter being used. This definition of size is interpreter-agnostic.
pub type MemSize = usize;

/// Index of a position within a MemoryBlocks, must be within the allocated size of the MemoryBlock
/// to prevent buffer overrun
/// Indexes start at zero
pub type Index = usize;

/// MemoryBlock of the first cell in a memory block of the given size
/// Use CellPositions to represent locations within a memory block pointed to by an MemoryBlock
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemoryBlock {
    id: MemId,
    size: MemSize,
}

impl MemoryBlock {
    /// Returns the unique identifier of this memory block
    /// This CANNOT be used to construct other memory blocks
    pub fn id(&self) -> MemId {
        self.id
    }

    /// Returns the size of this memory block
    pub fn size(&self) -> MemSize {
        self.size
    }

    /// Returns the cell position of the first cell within this MemoryBlock
    pub fn position(&self) -> CellPosition {
        self.position_at(0)
    }

    /// Returns the position of the cell at the given index within this MemoryBlock
    ///
    /// # Panics
    /// If the index is out of bounds, this method will panic. This is meant to catch compiler
    /// errors early. If the usage of this method is well designed, the panic should never happen
    /// Only panics when compiled in debug mode
    pub fn position_at(&self, index: Index) -> CellPosition {
        debug_assert!(index < self.size,
            "Attempt to access a position outside of the memory allocated for a MemoryBlock");

        CellPosition(*self, index)
    }
}

impl Default for MemoryBlock {

    /// The default memory block is useful whenever you need no memory but still need to pass
    /// a memory block to a function
    /// This is useful because it will cause the compiler to panic if anyone ever attempts to
    /// mutate this block (since it is zero sized)
    /// This saves the allocator from wasting too many IDs on zero-sized blocks of memory
    fn default() -> Self {
        MemoryBlock {
            id: MemId(0),
            size: 0,
        }
    }
}

/// The position of a cell within an MemoryBlock
/// Index = 0 indicates the start of the memory block
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CellPosition(MemoryBlock, Index);

impl CellPosition {
    pub fn associated_memory(&self) -> MemoryBlock {
        self.0
    }

    pub fn id(&self) -> MemId {
        self.0.id()
    }

    pub fn offset(&self) -> Index {
        self.1
    }
}

pub struct StaticAllocator {
    next_id: usize,
}

impl StaticAllocator {
    pub fn new() -> StaticAllocator {
        StaticAllocator {
            next_id: 1,
        }
    }

    /// Allocates a memory block of the given size and gives it a unique ID
    /// so that this memory block can be referred to uniquely
    pub fn allocate(&mut self, size: MemSize) -> MemoryBlock {
        if size == 0 {
            return MemoryBlock::default();
        }

        let blk = MemoryBlock {
            id: MemId(self.next_id),
            size: size,
        };
        self.next_id += 1;

        blk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocating_zero_returns_default() {
        let mut allocator = StaticAllocator::new();

        // Allocate zero memory
        let mem = allocator.allocate(0);

        // Make sure the default gets returned
        let default = MemoryBlock::default();
        assert_eq!(mem, default);

        // Make sure it keeps happening
        let mem = allocator.allocate(0);
        assert_eq!(mem, default);
    }

    #[test]
    fn cannot_allocate_default_memory_block() {
        let mut allocator = StaticAllocator::new();

        // Allocate a non-zero amount of memory
        let mem = allocator.allocate(1);

        // Make sure the default doesn't accidentally get returned
        let default = MemoryBlock::default();
        assert!(mem != default);
    }
}
