use std::rc::Rc;

use memory::Size;

use super::operation::Operation;
use super::scope::{ScopeItem, ScopeStack};

/// The arguments that will get passed to a function
/// Distinct to the function's actual type because these are
/// ScopeItems not type defintions
/// Arguments can be assumed to match the type of that function
pub type FuncArgs = Vec<ScopeItem>;

#[derive(Clone)]
pub enum FuncArgType {
    /// A single argument of the given type
    Arg(ItemType),
    /// Any number of arguments of the given type
    Variadic(ItemType),
}

/// An item is anything that can be declared
#[derive(Clone)]
pub enum ItemType {
    /// Any type
    ///TODO: Replace with type bounds when generics are implemented in #45
    Any,

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
        name: String,
        //TODO: fields, generics, etc.
    },

    /// A fixed-size array
    Array {
        /// The type of the elements in this array
        type_name: String,
        /// The number of elements that this array can hold
        size: usize,
    },

    /// Definition of a function's type
    Function {
        args: Vec<FuncArgType>,
        return_type: Box<ItemType>,
    },
}

impl ItemType {
    /// Computes the required size in cells of this type
    pub fn required_size(&self, scope: &ScopeStack) -> Size {
        match *self {
            ItemType::Primitive(size) => size,
            ItemType::Struct { .. } => Size::default(),
            _ => unimplemented!(),
        }
    }
}
