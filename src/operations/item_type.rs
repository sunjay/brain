use memory::Size;

use super::scope::{ScopeStack, TypeId, ArraySize};

/// Possible types for function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum FuncArgType {
    /// A single value of the specified type
    Arg(TypeId),

    /// A single value that is an array containing the specified item type
    Array {
        item: TypeId,
        /// The exact, required size of the array argument
        /// If size is None, the function can dynamically handle any size, otherwise only this size
        /// will be accepted
        ///TODO: Remove the ability for this to be optional when slices and references are
        /// implemented since dynamically handling any length of array will not work for most things
        size: Option<ArraySize>,
    },

    /// Zero or more values of the specified type
    /// If the type is None, that means that there is no specific type being required
    /// so any type can be passed as an argument
    ///TODO: This is mostly a hack to allow for generics before we support them. It would be nice
    /// to eventually implement this properly when #45 comes along
    Variadic(Option<TypeId>),
}

impl FuncArgType {
    /// Returns true if this function argument is an array with the given item type
    pub fn is_array_of(&self, target: TypeId) -> bool {
        match *self {
            FuncArgType::Array {item, ..} => item == target,
            _ => false,
        }
    }
}

/// An item is anything that can be declared
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    /// Unit type
    /// The unit type is a type with a single zero-size value.
    /// Both the type and the value are specified: ()
    Unit,

    /// A primitive type is a raw interpretation of some memory cells
    /// These primitive types are built-in and cannot be declared
    /// within the language itself
    Primitive(Size),

    /// A struct has a single definition with any number of
    /// fields and generics
    /// Structs can have impls which contain methods for that
    /// struct
    Struct {
        //TODO: fields, generics, etc.
    },

    /// A declaration of an array type, optionally specialized for the given item type
    Array {
        /// The type of the items stored in this array
        /// If this is None, this is the base, generic array type
        item: Option<TypeId>,
    },

    /// Definition of a function's type
    Function {
        args: Vec<FuncArgType>,
        return_type: TypeId,
    },
}

impl ItemType {
    /// Computes the required size in cells of an *instance* of this type
    pub fn required_size(&self, scope: &ScopeStack) -> Size {
        match *self {
            ItemType::Unit => Size::default(),
            ItemType::Primitive(size) => size,
            ItemType::Struct { .. } => Size::default(), //TODO: Update this when fields are supported
            _ => unimplemented!(),
        }
    }
}
