use std::collections::{VecDeque, HashMap};

use memory::{StaticAllocator, MemoryBlock};

use super::item_type::ItemType;

/// Represents a single level of scope
pub type Scope = HashMap<String, MemoryBlock>;

pub struct ScopeStack {
    stack: VecDeque<Scope>,
    allocator: StaticAllocator,
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        ScopeStack {
            stack: {
                let mut queue = VecDeque::new();
                queue.push_back(Scope::new());
                queue
            },
            allocator: StaticAllocator::new(),
        }
    }

    /// Declares a name with the given type, allocates enough space for that type
    /// The name is declared in the "current" scope which is at the top of the stack
    /// Returns the allocated memory block
    pub fn declare(&mut self, name: String, typ: ItemType) -> MemoryBlock {
        unimplemented!();
    }

    /// Allocate a memory block that is large enough for the given type
    /// Does not associate memory block with a name which means it cannot be looked up later
    /// Returns the allocated memory block
    pub fn allocate(&mut self, typ: ItemType) -> MemoryBlock {
        let size = typ.required_size();
        self.allocator.allocate(size)
    }
}
