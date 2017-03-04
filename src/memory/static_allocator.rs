// Kept private because MemoryBlocks cannot be created outside of this static allocator
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Id(usize);

/// Size of a memory block in cells
/// One cell is typically equivalent to a byte, however that depends on the implementation of
/// the brainfuck interpreter being used. This definition of size is interpreter-agnostic.
pub type Size = usize;

/// Index of a position within a MemoryBlocks, must be within the allocated size of the MemoryBlock
/// to prevent buffer overrun
/// Indexes start at zero
pub type Index = usize;

/// MemoryBlock of the first cell in a memory block of the given size
/// Use CellPositions to represent locations within a memory block pointed to by an MemoryBlock
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemoryBlock {
    id: Id,
    size: Size,
}

impl MemoryBlock {
    /// Returns the size of this memory block
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns the cell position of the first cell within this MemoryBlock
    pub fn position(&self) -> CellPosition {
        CellPosition(self.id, 0)
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

        CellPosition(self.id, index)
    }
}

/// The position of a cell within an MemoryBlock
/// Index = 0 indicates the start of the memory block
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CellPosition(Id, Index);

pub struct StaticAllocator {
    next_id: usize,
}

impl StaticAllocator {
    pub fn new() -> StaticAllocator {
        StaticAllocator {
            next_id: 0,
        }
    }

    /// Allocates a memory block of the given size and gives it a unique ID
    /// so that this memory block can be referred to uniquely
    pub fn allocate(&mut self, size: Size) -> MemoryBlock {
        let blk = MemoryBlock {
            id: Id(self.next_id),
            size: size,
        };
        self.next_id += 1;

        blk
    }
}
