use memory::Size;

use super::scope::{ScopeStack, TypeId};

/// Possible types for function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum FuncArgType {
    /// Any type
    ///TODO: Replace with type bounds when generics are implemented in #45
    Any,

    /// A single value of the specified type
    Arg(TypeId),

    /// Zero or more values of the specified type
    Variadic(TypeId),
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

    /// A fixed-size array
    Array {
        /// The type of the elements in this array
        type_id: TypeId,
        /// The number of elements that this array can hold
        size: usize,
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
