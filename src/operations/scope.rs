use std::rc::Rc;
use std::collections::{VecDeque, HashMap};

use parser::Identifier;
use memory::{StaticAllocator, MemoryBlock};

use super::operation::Operation;
use super::item_type::{ItemType, FunctionTypeDef, FuncArgs};

/// Represents a single item in a scope
pub struct ScopeItem {
    pub type_def: ItemType,
    pub memory: MemoryBlock,
}

/// Represents a single level of scope
pub type Scope = HashMap<Identifier, ScopeItem>;

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
    /// Returns ALL matches so that the caller can determine which definition is
    /// the correct one
    /// Definitions are returned in order from latest definition to oldest
    /// Always use the first definition that matches the type you are looking for
    pub fn lookup(&self, name: &Identifier) -> Vec<&ScopeItem> {
        self.stack.iter().rev().map(|sc| sc.get(name)).fold(Vec::new(), |mut acc, r| match r {
            Some(def) => {
                acc.push(def);
                acc
            },
            None => acc,
        })
    }

    /// Declares a name with the given type, allocates enough space for that type
    /// The name is declared in the "current" scope which is at the top of the stack
    /// Returns the allocated memory block
    pub fn declare(&mut self, name: Identifier, typ: &ItemType) -> MemoryBlock {
        let mem = self.allocate(typ);
        // It's OK to overwrite existing names because we support rebinding
        if let Some(scope) = self.stack.back_mut() {
            scope.insert(name, ScopeItem {
                type_def: typ.clone(),
                memory: mem,
            });
        }
        else {
            panic!("Attempt to declare name despite having no current scope");
        }

        mem
    }

    /// Declares a built in function with the given name and type definition
    /// The name is declared in the "current" scope which is at the top of the stack
    /// Returns the allocated memory block
    fn declare_builtin_function<F: 'static>(&mut self, name: Identifier, func_type: FunctionTypeDef, f: F) -> MemoryBlock
        where F: Fn(FuncArgs, ScopeStack) -> Vec<Operation> {

        self.declare(
            name,
            &ItemType::BuiltInFunction {
                type_def: func_type,
                operations: Rc::new(f),
            }
        )
    }

    /// Allocate a memory block that is large enough for the given type
    /// Does not associate memory block with a name which means it cannot be looked up later
    /// Returns the allocated memory block
    pub fn allocate(&mut self, typ: &ItemType) -> MemoryBlock {
        let size = typ.required_size(self);
        self.allocator.allocate(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::Identifier;
    use operations::item_type::ItemType;

    #[test]
    fn multiple_definitions() {
        let mut scope = ScopeStack::new();
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 0);

        scope.declare(Identifier::from("foo"), &ItemType::Primitive(1));
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 1);

        // Declaring the same name in the same scope should overwrite the
        // definition
        scope.declare(Identifier::from("foo"), &ItemType::Primitive(1));
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 1);

        scope.push_scope();
        // Declaring foo in another scope should add a definition
        scope.declare(Identifier::from("foo"), &ItemType::Primitive(1));
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 2);

        // Declaring the same name in the same scope should overwrite the
        // definition
        scope.declare(Identifier::from("foo"), &ItemType::Primitive(1));
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 2);
    }
}
