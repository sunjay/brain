use std::rc::Rc;

use memory::Size;

use super::operation::Operation;
use super::scope::{ScopeItem, ScopeStack};

pub type FuncArgs = Vec<ScopeItem>;

#[derive(Clone)]
pub struct FunctionTypeDef {
    args: Vec<ItemType>,
    return_type: Box<ItemType>,
}

/// An item is anything that can be declared
#[derive(Clone)]
pub enum ItemType {
    /// A struct has a single definition with any number of
    /// fields and generics
    /// Structs can have impls which contain methods for that
    /// struct
    Struct {/*TODO*/},

    /// A fixed-size array
    Array {
        /// The type of the elements in this array
        type_name: String,
        /// The number of elements that this array can hold
        size: usize,
    },

    BuiltInFunction {
        type_def: FunctionTypeDef,
        /// Generates operations that represent calling the
        /// function with the given arguments
        operations: Rc<Fn(FuncArgs, ScopeStack) -> Vec<Operation>>,
    }
}

impl ItemType {
    /// Computes the required size in cells of this type
    pub fn required_size(&self, scope: &ScopeStack) -> Size {
        unimplemented!(); //TODO
    }
}
