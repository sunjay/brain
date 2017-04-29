use std::collections::HashMap;

use super::{MemId, MemSize, MemoryBlock, CellPosition};

/// Represents a zero-indexed position in the brainfuck tape
pub type CellIndex = usize;

// Represents a contiguous block of brainfuck memory cells starting at the given position.
// This position directly maps to a cell in the brainfuck tape.
#[derive(Debug)]
pub struct Cells {
    position: CellIndex,
    size: MemSize,
}

impl Cells {
    pub fn position(&self) -> CellIndex {
        self.position
    }

    pub fn size(&self) -> MemSize {
        self.size
    }
}

#[derive(Debug)]
pub struct MemoryLayout {
    table: HashMap<MemId, Cells>,
    // The total size of the entire memory layout
    size: MemSize,
}

impl MemoryLayout {
    /// Constructs an empty memory layout
    pub fn new() -> MemoryLayout {
        MemoryLayout {
            table: HashMap::new(),
            size: 0,
        }
    }

    /// Returns the total size of the memory layout
    pub fn size(&self) -> MemSize {
        self.size
    }

    /// Removes a memory block from the memory layout
    ///
    /// NOTE: This **DOES NOT** guarantee that the associated cells have been zeroed. That is up
    /// to you.
    pub fn remove(&mut self, mem: &MemoryBlock) {
        let cells = self.table.remove(&mem.id()).expect("Removed memory block that was already removed or never present");
        // We currently just free up that space for reuse if its at the end of the buffer
        // Ideally this memory layout would be implemented as some sort of efficient memory pool
        // where we could find the nearest space and put the cells there
        // Then this remove operation would just remove that item in the memory pool
        if cells.position() + cells.size() == self.size {
            self.size -= cells.size();
        }
    }

    /// Gets the brainfuck cells associated to the given memory block
    pub fn get(&mut self, mem: &MemoryBlock) -> &Cells {
        self.maybe_layout(mem);
        self.table.get(&mem.id()).unwrap()
    }

    /// Gets the CellIndex of the given CellPosition based on its position in the memory layout
    pub fn position(&mut self, pos: &CellPosition) -> CellIndex {
        let cells = self.get(&pos.associated_memory());
        debug_assert!(pos.offset() < cells.size());
        cells.position() + pos.offset()
    }

    /// Allocates a temporary cell which is only valid to use up to the next call to get() or position()
    pub fn temporary(&self, size: MemSize) -> Cells {
        //TODO: Do we need a better way to implement this?
        // This seems like it's just asking for trouble...
        Cells {
            position: self.size,
            size: size,
        }
    }

    fn maybe_layout(&mut self, mem: &MemoryBlock) {
        let key = mem.id();
        if self.table.contains_key(&key) {
            // Invariant: the previously stored size should
            // be the same as the current one
            debug_assert!(self.table.get(&key).unwrap().size() == mem.size());
        }
        else {
            let mem_size = mem.size();
            self.table.insert(key, Cells {
                position: self.size,
                size: mem_size,
            });

            // This should ONLY be incremented the **first** time this is inserted
            self.size += mem_size;
        }
    }
}
