use std::rc::Rc;
use std::collections::{VecDeque, HashMap};

use parser::{Identifier, Number};
use memory::{StaticAllocator, MemoryBlock};

use super::OperationsResult;
use super::item_type::ItemType;

pub use super::primitives::Primitives;

pub type TypeId = usize;

/// Represents the number of items in an array
/// NOT the number of bytes allocated to the array
pub type ArraySize = usize;

/// The arguments that will get passed to a function
/// Arguments are guaranteed by static analysis to match the type defined for the function
pub type FuncArgs = Vec<ScopeItem>;

/// Represents a type declared in a scope
pub enum ScopeType {
    /// A type, not associated with any memory
    /// Used for a struct/type declaration, not the declaration
    /// of a variable with a type (TypedBlock should be used for that)
    Type(TypeId),

    //TODO: Generic types, etc. will all go here
}

/// Represents a single item in a scope
#[derive(Clone)]
pub enum ScopeItem {
    /// A constant set of bytes inlined whenever used
    /// These items have no memory address
    /// The bytes of the constant are stored directly
    Constant {
        type_id: TypeId,
        /// Size of bytes must match the required size of type_id
        bytes: Vec<u8>,
    },

    /// A constant value that represents a numeric literal without a specific number type
    /// Reserved for use with internal compiler functions
    /// Numeric literals are stored in a temporary memory location of the appropriate type for
    /// non-built-in functions
    NumericLiteral(Number),

    /// A constant value that represents a byte literal
    /// Reserved for use with internal compiler functions
    ByteLiteral(Vec<u8>),

    /// A typed block of memory
    TypedBlock {
        type_id: TypeId,
        memory: MemoryBlock,
    },

    /// A specialization of the generic array type [T; N]
    Array {
        /// The type of the items held by the array
        ///TODO: This type will need to be made more flexible in order to support nested arrays
        item: TypeId,
        /// The declared number of items held by the array
        size: ArraySize,
        /// The block of memory allocated to this array
        /// Size of this block is always sizeof(item) * size
        memory: MemoryBlock,
    },

    /// The implementation of a built-in function
    /// Note that the type signature is stored separately
    BuiltInFunction {
        /// The ID of the type associated with this function
        type_id: TypeId,
        /// Generates operations that represent calling the
        /// function with the given arguments
        /// Function should store the result in the memory block represented by the third
        /// parameter
        operations: Rc<Fn(&mut ScopeStack, FuncArgs, MemoryBlock) -> OperationsResult>,
    },
}

impl ScopeItem {
    pub fn numeric_literal_value(&self) -> Number {
        match *self {
            ScopeItem::NumericLiteral(number) => number,
            _ => panic!("Called `ScopeItem::numeric_literal_value()` on a non `NumericLiteral` value"),
        }
    }

    /// Returns the TypeId of this ScopeItem
    ///
    /// # Panics
    /// Panics if the ScopeItem does not store its TypeId, those variants should be handled
    /// separately
    pub fn type_id(&self) -> TypeId {
        use self::ScopeItem::*;

        match *self {
            Constant { type_id, .. } => type_id,
            TypedBlock { type_id, .. } => type_id,
            BuiltInFunction { type_id, .. } => type_id,
            NumericLiteral(..) | ByteLiteral(..) | Array {..} => panic!("Variant does not store its TypeId"),
        }
    }
}

/// Represents a single level of scope
pub struct Scope {
    types: HashMap<Identifier, ScopeType>,
    items: HashMap<Identifier, ScopeItem>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            types: HashMap::new(),
            items: HashMap::new(),
        }
    }
}

pub struct ScopeStack {
    stack: VecDeque<Scope>,
    allocator: StaticAllocator,
    /// A vector of all the declared types used to produce unique identities
    /// for all types so that types are static and not dependent on the context
    /// in which they are used.
    /// Basically, if we declare a type Foo and a variable with that type,
    /// we don't want a later declaration of Foo change the type of the variable
    /// Also used in functions/closures to uniquely refer to types in that context
    types: Vec<(Identifier, ItemType)>,

    /// Special primitive types
    /// This is used because the compiler needs some assurance that primitive types are who
    /// they say they are. For example, if the compiler needs to refer to `bool`, it needs to
    /// be able to lookup that type without worrying about conflicting with user defined types
    primitives: Primitives,
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
            types: vec![(Identifier::from("()"), ItemType::Unit)],
            primitives: {
                let mut primitives = Primitives::new();
                // 0 is the index of the Unit type in the types array declared above
                primitives.register("unit", 0);
                primitives
            }
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

    /// Returns the type name associated with the given TypeId
    pub fn get_type_name(&self, type_id: TypeId) -> &Identifier {
        // We just unwrap here because this isn't an error that can be generated by the user's
        // mistake. If this fails, it has to be a bug in the compiler.
        &self.types.get(type_id).expect("Invalid TypeId used to lookup type").0
    }

    /// Returns the type associated with the given TypeId
    pub fn get_type(&self, type_id: TypeId) -> &ItemType {
        // We just unwrap here because this isn't an error that can be generated by the user's
        // mistake. If this fails, it has to be a bug in the compiler.
        &self.types.get(type_id).expect("Invalid TypeId used to lookup type").1
    }

    /// Access special primitives
    /// e.g. `scope.primitives().unit()`
    pub fn primitives(&self) -> &Primitives {
        &self.primitives
    }

    /// Register a primitive type
    /// The type will automatically be checked for uniqueness. That is, no other primitive has
    /// been defined with the same TypeID
    /// Primitives can only be declared once per primitive and then never redeclared
    pub fn register_primitive(&mut self, name: &str, type_id: TypeId) {
        self.primitives.register(name, type_id);
    }

    /// Looks up a name starting at the current scope
    /// Returns ALL matches so that the caller can determine which definition is
    /// the correct one
    /// Definitions are returned in order from latest definition to oldest
    /// Always use the first definition that matches the type you are looking for
    pub fn lookup(&self, name: &Identifier) -> Vec<&ScopeItem> {
        self.search_stack(name, |sc| &sc.items)
    }

    pub fn lookup_type(&self, name: &Identifier) -> Vec<&ScopeType> {
        self.search_stack(name, |sc| &sc.types)
    }

    fn search_stack<T, F>(&self, name: &Identifier, scope_table: F) -> Vec<&T>
        where F: Fn(&Scope) -> &HashMap<Identifier, T> {
        self.stack.iter().rev().map(|sc| scope_table(sc).get(name)).fold(Vec::new(), |mut acc, r| match r {
            Some(def) => {
                acc.push(def);
                acc
            },
            None => acc,
        })
    }

    /// Declares a type with the given name
    /// Returns the unique identifier of that type
    pub fn declare_type(&mut self, name: Identifier, typ: ItemType) -> TypeId {
        let type_id = self.insert_type(name, typ);

        type_id
    }

    /// Declares a constant with the given name
    pub fn declare_constant(&mut self, name: Identifier, type_id: TypeId, value: Vec<u8>) {
        debug_assert!({
            let size = self.get_type(type_id).required_size(self);
            value.len() == size
        });

        self.insert_item_into_current(name, ScopeItem::Constant {
            type_id: type_id,
            bytes: value,
        });
    }

    /// Declares a name with the given type and allocates enough space for that type
    /// The name is declared in the "current" scope which is at the top of the stack
    /// Returns the allocated memory block
    pub fn declare(&mut self, name: Identifier, type_id: TypeId) -> MemoryBlock {
        let mem = self.allocate(type_id);
        self.insert_item_into_current(name, ScopeItem::TypedBlock {
            type_id: type_id,
            memory: mem,
        });

        mem
    }

    /// Declares a name with an array type with the given item type and allocates enough space for
    /// that type and all its elements
    /// The array is allocated as a single, contiguous block of memory
    /// The name is declared in the "current" scope which is at the top of the stack
    /// Returns the allocated memory block
    pub fn declare_array(&mut self, name: Identifier, item: TypeId, size: ArraySize) -> MemoryBlock {
        let mem = self.allocate_array(item, size);
        self.insert_item_into_current(name, ScopeItem::Array {
            item: item,
            size: size,
            memory: mem,
        });

        mem
    }

    /// Allocate a contiguous block of memory that can fit `size` of the given `item` types
    /// Does not associate memory block with a name which means it cannot be looked up later
    /// Returns the allocated memory block
    pub fn allocate_array(&mut self, item: TypeId, size: ArraySize) -> MemoryBlock {
        let size = self.get_type(item).required_size(self) * size;
        self.allocator.allocate(size)
    }

    /// Allocate a memory block that is large enough for the given type
    /// Does not associate memory block with a name which means it cannot be looked up later
    /// Returns the allocated memory block
    pub fn allocate(&mut self, type_id: TypeId) -> MemoryBlock {
        let size = self.get_type(type_id).required_size(self);
        self.allocator.allocate(size)
    }

    /// Declares a built in function with the given name and type definition
    /// The name is declared in the "current" scope which is at the top of the stack
    /// The function is guaranteed to be called with arguments that match its given type signature
    /// Functions that can be called on an instance of a type should have that type as the first
    /// parameter as the "self" of that function
    pub fn declare_builtin_function<F: 'static>(&mut self, name: Identifier, typ: ItemType, f: F)
        where F: Fn(&mut ScopeStack, FuncArgs, MemoryBlock) -> OperationsResult {

        // Make sure we are declaring the function as a function type
        debug_assert!(match typ {
            ItemType::Function { .. } => true,
            _ => false,
        });

        let type_id = self.insert_type(name.clone(), typ);
        self.insert_item_into_current(name, ScopeItem::BuiltInFunction {
            type_id: type_id,
            operations: Rc::new(f),
        });
    }

    /// Inserts a type defintion into the types field and returns its new TypeId
    fn insert_type(&mut self, name: Identifier, typ: ItemType) -> TypeId {
        self.types.push((name.clone(), typ));

        let type_id = self.types.len() - 1;
        self.insert_type_into_current(name, ScopeType::Type(type_id));

        type_id
    }

    /// Inserts a ScopeItem into the current scope
    fn insert_item_into_current(&mut self, name: Identifier, item: ScopeItem) {
        // Notice that we insert directly without caring about whether the name already exists
        // It's OK to overwrite existing names because we support rebinding
        let scope = self.stack.back_mut()
            .expect("Attempt to declare item despite having no current scope");
        scope.items.insert(name, item);
    }

    /// Inserts a ScopeType into the current scope
    fn insert_type_into_current(&mut self, name: Identifier, item: ScopeType) {
        // Notice that we insert directly without caring about whether the name already exists
        // It's OK to overwrite existing names because we support rebinding
        let scope = self.stack.back_mut()
            .expect("Attempt to declare type despite having no current scope");
        scope.types.insert(name, item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::Identifier;
    use operations::item_type::ItemType;

    #[test]
    fn defines_unit_primitive() {
        let scope = ScopeStack::new();

        let unit_type_id = scope.primitives().unit();
        assert_eq!(*scope.get_type(unit_type_id), ItemType::Unit);
    }

    #[test]
    fn multiple_definitions() {
        let mut scope = ScopeStack::new();
        let type_id = scope.declare_type(Identifier::from("FooType"), ItemType::Primitive(1));
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 0);

        scope.declare(Identifier::from("foo"), type_id);
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 1);

        // Declaring the same name in the same scope should overwrite the
        // definition
        scope.declare(Identifier::from("foo"), type_id);
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 1);

        scope.push_scope();
        // Declaring foo in another scope should add a definition
        scope.declare(Identifier::from("foo"), type_id);
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 2);

        // Declaring the same name in the same scope should overwrite the
        // definition
        scope.declare(Identifier::from("foo"), type_id);
        assert_eq!(scope.lookup(&Identifier::from("foo")).len(), 2);
    }
}
