use std::collections::HashMap;

use super::{MemId, MemSize, MemoryBlock, CellPosition};

/// Represents a zero-indexed position in the brainfuck tape
pub type CellIndex = usize;

// Represents a contiguous block of brainfuck memory cells starting at the given position.
// This position directly maps to a cell in the brainfuck tape.
#[derive(Debug, Clone, Copy)]
pub struct Cells {
    position: CellIndex,
    size: MemSize,
}

impl Cells {
    pub fn position(&self) -> CellIndex {
        self.position
    }

    pub fn position_at(&self, index: CellIndex) -> CellIndex {
        debug_assert!(index < self.size,
            "Attempt to access a position outside of the bounds of the given Cells");

        self.position + index
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

        self.remove_cells(cells);
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

    /// Allocates a temporary cell which is only valid to use until the end of the given callback
    pub fn temporary<F, T>(&mut self, size: MemSize, callback: F) -> T
        where F: FnOnce(Cells) -> T {
        let position = self.allocate(size);
        let cells = Cells {position, size};
        let res = callback(cells);
        self.remove_cells(cells);
        res
    }

    /// Allocates temporary cells that are placed consecutively after the given memory block
    pub fn consecutive<F, T>(&mut self, target: &MemoryBlock, size: MemSize, callback: F) -> T
        where F: FnOnce(&mut MemoryLayout, CellIndex, Cells) -> T {
        // allocate the target first, if it hasn't already been allocated
        let (target_position, target_size) = {
            let target_cells = self.get(target);
            (target_cells.position(), target_cells.size())
        };
        // allocate the requested temporary cells
        let position = self.allocate(size);
        // assert that the cells are consecutive
        //TODO: See if there is a better way to deal with the cells not being consecutive
        assert_eq!(position - target_position, target_size);

        let cells = Cells {position, size};
        let res = callback(self, target_position, cells);
        self.remove_cells(cells);
        res
    }

    fn maybe_layout(&mut self, mem: &MemoryBlock) {
        let key = mem.id();
        if self.table.contains_key(&key) {
            // Invariant: the previously stored size should
            // be the same as the current one
            debug_assert!(self.table.get(&key).unwrap().size() == mem.size());
        }
        else {
            let size = mem.size();
            // This should ONLY be incremented the **first** time this is inserted
            let position = self.allocate(size);
            self.table.insert(key, Cells {position, size});
        }
    }

    fn allocate(&mut self, size: MemSize) -> CellIndex {
        let position = self.size;
        self.size += size;
        position
    }

    fn remove_cells(&mut self, cells: Cells) {
        // We currently just free up that space for reuse if its at the end of the buffer
        // This means that there can be "holes" leftover in the layout after the removal
        // Ideally this memory layout would be implemented as some sort of efficient memory pool
        // where we could find the nearest space and put the cells there
        // Then this remove operation would just remove that item in the memory pool
        if cells.position() + cells.size() == self.size {
            self.size -= cells.size();
        }
    }
}
