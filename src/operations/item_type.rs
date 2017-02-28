use memory::Size;

use super::scope::ScopeStack;

/// An item is anything that can be declared
#[derive(Debug, PartialEq, Clone)]
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
}

impl ItemType {
    /// Computes the required size in cells of this type
    pub fn required_size(&self, scope: &ScopeStack) -> Size {
        unimplemented!(); //TODO
    }
}
