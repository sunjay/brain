use std::collections::{VecDeque, HashMap};

use memory::{StaticAllocator, MemoryBlock};

use super::item_type::ItemType;

/// Represents a single level of scope
pub type Scope = HashMap<String, (ItemType, MemoryBlock)>;

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

    /// Pushes a new level of scope onto the stack
    /// This scope will become the current scope
    pub fn push_scope(&mut self) {
        self.stack.push_back(Scope::new());
    }

    /// Removes and returns the top level scope (current scope)
    ///
    /// # Panics
    /// Panics if there is no scope in the stack
    pub fn pop_scope(&mut self) -> Scope {
        self.stack.pop_back().unwrap()
    }

    /// Looks up a name starting at the current scope
    pub fn lookup(&self, name: &str) -> Option<&(ItemType, MemoryBlock)> {
        self.stack.iter().rev().map(|sc| sc.get(name)).find(|r| r.is_some()).unwrap_or(None)
    }

    /// Declares a name with the given type, allocates enough space for that type
    /// The name is declared in the "current" scope which is at the top of the stack
    /// Returns the allocated memory block
    pub fn declare(&mut self, name: String, typ: ItemType) -> MemoryBlock {
        let mem = self.allocate(&typ);
        // It's OK to overwrite existing names because we support rebinding
        if let Some(scope) = self.stack.back_mut() {
            scope.insert(name, (typ, mem));
        }
        else {
            panic!("Attempt to declare name despite having no current scope");
        }

        mem
    }

    /// Allocate a memory block that is large enough for the given type
    /// Does not associate memory block with a name which means it cannot be looked up later
    /// Returns the allocated memory block
    pub fn allocate(&mut self, typ: &ItemType) -> MemoryBlock {
        let size = typ.required_size();
        self.allocator.allocate(size)
    }
}
