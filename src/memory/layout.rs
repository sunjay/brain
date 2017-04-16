use std::collections::HashMap;

use operations::{Operation, Operations};

use super::{MemId, MemSize, MemoryBlock};

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
    /// Intentionally left private since memory layouts should
    /// typically not be empty
    fn new() -> MemoryLayout {
        MemoryLayout {
            table: HashMap::new(),
            size: 0,
        }
    }

    /// Returns the total size of the memory layout
    pub fn size(&self) -> MemSize {
        self.size
    }

    /// Gets the brainfuck cells associated to the given memory identifier
    pub fn get(&self, id: &MemId) -> &Cells {
        // If a memory identifier does not exist in the layout, it's likely a bug
        self.table.get(id)
            .expect("Attempt to get memory that is not part of the layout")
    }

    /// Populates a memory layout based on the given operations.
    ///
    /// This layout algorithm performs no drops. That means that
    /// every single memory block will get its own allocated space
    /// on the brainfuck tape. Nothing will need to be deallocated
    /// or "zeroed". This layout usually results in a larger amount
    /// of memory usage and more move instructions depending on the
    /// program.
    ///
    /// It does benefit from avoiding instructions involved in
    /// ensuring that a memory cell is zero.
    /// The layout algorithm is also fairly simple overall.
    fn layout_no_drop(&mut self, ops: &Operations) {
        use self::Operation::*;
        for op in ops {
            match *op {
                // Note that we ignore the TempAllocate::temp MemoryBlock.
                // We only want to allocate a spot for that on the layout when it
                // is used.
                Block {ref body} | TempAllocate {ref body, ..} => {
                    self.layout_no_drop(body);
                },
                Increment {ref target, ..} | Decrement {ref target, ..} | Loop {cond: ref target, ..} => {
                    self.maybe_layout(target.associated_memory());
                },
                Read {target} | Write {target} | Zero {target} => {
                    self.maybe_layout(target);
                },
                Copy {ref source, ref target, ..} => {
                    self.maybe_layout(source.associated_memory());
                    self.maybe_layout(target.associated_memory());
                },
                Relocate {source, target} => {
                    self.maybe_layout(source);
                    self.maybe_layout(target);
                },
            }
        }
    }

    fn maybe_layout(&mut self, mem: MemoryBlock) {
        let key = mem.id();
        if self.table.contains_key(&key) {
            // Invariant: the previously stored size should
            // be the same as the current one
            debug_assert!(self.table.get(&key).unwrap().size() == mem.size());
        }
        else {
            // This should ONLY be incremented the **first** time this is inserted
            let mem_size = mem.size();
            self.size += mem_size;

            let next_index = self.size;
            self.table.insert(key, Cells {
                position: next_index,
                size: mem_size,
            });
        }
    }
}

impl<'a> From<&'a Operations> for MemoryLayout {
    fn from(ops: &'a Operations) -> MemoryLayout {
        let mut layout = MemoryLayout::new();
        layout.layout_no_drop(ops);
        layout
    }
}
