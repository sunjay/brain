use std::iter::Peekable;
use std::slice::Iter;

use memory::MemSize;

use super::scope::{ScopeStack, TypeId, ArraySize};

/// Possible types for function arguments
#[derive(Debug, Clone, PartialEq, Eq)]
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
        /// implemented since our dynamic handling any length of array will not work for most things
        size: Option<ArraySize>,
    },

    /// Zero or more values of the specified type
    /// If the type is None, that means that there is no specific type being required
    /// so any type can be passed as an argument
    /// No arguments are allowed in a Function type after a variadic argument. This is mostly
    /// unenforced at the moment. If a function breaks this rule, it will never match during call()
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    /// Unit type
    /// The unit type is a type with a single zero-size value.
    /// Both the type and the value are specified: ()
    Unit,

    /// A primitive type is a raw interpretation of some memory cells
    /// These primitive types are built-in and cannot be declared
    /// within the language itself
    Primitive(MemSize),

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
        /// The exact, required size of the array argument
        /// If size is None, this represents any size of array (i.e. the wildcard size `_`)
        ///TODO: Remove the ability for this to be optional when slices and references are
        /// implemented since our dynamic handling any length of array will not work for most things
        size: Option<ArraySize>,
    },

    /// Definition of a function's type
    Function {
        args: Vec<FuncArgType>,
        //TODO: Support array return types
        return_type: TypeId,
    },
}

impl ItemType {
    /// Computes the required size in cells of an *instance* of this type
    pub fn required_size(&self, scope: &ScopeStack) -> MemSize {
        match *self {
            ItemType::Unit => MemSize::default(),
            ItemType::Primitive(size) => size,
            ItemType::Struct { .. } => MemSize::default(), //TODO: Update this when fields are supported
            ItemType::Array {item: Some(item), size: Some(size)} => {
                scope.get_type(item).required_size(scope) * size
            },
            ItemType::Function { .. } => MemSize::default(),
            _ => unreachable!(),
        }
    }

    /// Returns true if this item type matches the given function signature (args, return type)
    /// Returns false if this item type is not a function
    /// Note: Variadic matching is only done one-way
    ///
    /// That means that this will return false:
    ///     self = Function {args: [Arg(1)], return_type: 0}
    ///     expected_args = [Variadic(1)]
    ///     return_type = 0
    ///
    /// However, this will return true (as expected):
    ///     self = Function {args: [Variadic(1)], return_type: 0}
    ///     expected_args = [Arg(1)]
    ///     return_type = 0
    pub fn matches_signature(&self, expected_args: &Vec<FuncArgType>, expected_return_type: TypeId) -> bool {
        let mut expected_args = expected_args.iter().peekable();
        match *self {
            ItemType::Function {ref args, return_type} => (
                return_type == expected_return_type &&
                // All the args must match an argument in expected_args
                args.iter().all(|arg| match *arg {
                    FuncArgType::Arg(type_id) => match expected_args.peek() {
                        Some(&&FuncArgType::Arg(arg_id)) if type_id == arg_id => {
                            expected_args.next();
                            true
                        },
                        _ => false,
                    },
                    FuncArgType::Array {item, size} => match expected_args.peek() {
                        Some(&&FuncArgType::Array {item: arg_item, size: arg_size}) if item == arg_item && size == arg_size => {
                            expected_args.next();
                            true
                        },
                        _ => false,
                    },
                    FuncArgType::Variadic(None) => {
                        // Exhaust the entire iterator
                        while let Some(_) = expected_args.next() {}
                        true
                    },
                    FuncArgType::Variadic(Some(type_id)) => match expected_args.peek() {
                        Some(&&FuncArgType::Variadic(Some(arg_id))) if type_id == arg_id => {
                            expected_args.next();
                            true
                        },
                        Some(&&FuncArgType::Arg(_)) => matches_variadic(type_id, &mut expected_args),
                        _ => false,
                    },
                })
            ),
            _ => false,
        }
    }
}

/// Tries to match as many of the expected_args as possible based on the given type_id
/// Returns false if this does not exhaust the iterator
fn matches_variadic(type_id: TypeId, expected_args: &mut Peekable<Iter<FuncArgType>>) -> bool {
    while let Some(&&FuncArgType::Arg(arg_id)) = expected_args.peek() {
        if arg_id == type_id {
            expected_args.next();
        }
        else {
            return false;
        }
    }

    // If the iterator is not exhausted, it means we hit something we did not expect
    expected_args.peek().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_signature() {
        // Should never match non-function (and should not panic either)
        assert_eq!(test_matches_signature(
            ItemType::Unit,
            &Vec::new(),
            0
        ), false);

        assert_eq!(test_matches_signature(
            ItemType::Function {
                args: vec![
                    FuncArgType::Arg(0),
                    FuncArgType::Arg(2),
                    FuncArgType::Array {item: 0, size: None},
                    FuncArgType::Arg(0),
                ],
                return_type: 22,
            },
            // Does not match!!
            &Vec::new(),
            22,
        ), false);

        assert_eq!(test_matches_signature(
            ItemType::Function {
                args: vec![
                    FuncArgType::Arg(0),
                    FuncArgType::Arg(2),
                    FuncArgType::Array {item: 0, size: None},
                    FuncArgType::Arg(0),
                ],
                return_type: 22,
            },
            &vec![
                FuncArgType::Arg(0),
                FuncArgType::Arg(2),
                FuncArgType::Array {item: 0, size: None},
                FuncArgType::Arg(0),
            ],
            // Does not match!!
            23,
        ), false);

        assert_eq!(test_matches_signature(
            ItemType::Function {
                args: vec![
                    FuncArgType::Arg(0),
                    FuncArgType::Arg(2),
                    FuncArgType::Array {item: 0, size: None},
                    FuncArgType::Arg(0),
                ],
                return_type: 22,
            },
            &vec![
                FuncArgType::Arg(0),
                FuncArgType::Arg(2),
                FuncArgType::Array {item: 0, size: None},
                FuncArgType::Arg(0),
            ],
            22,
        ), true);

        assert_eq!(test_matches_signature(
            ItemType::Function {
                args: vec![
                    FuncArgType::Arg(0),
                    FuncArgType::Variadic(None),
                ],
                return_type: 22,
            },
            &vec![
                FuncArgType::Arg(0),
                FuncArgType::Arg(2),
                FuncArgType::Array {item: 0, size: None},
                FuncArgType::Arg(0),
            ],
            22,
        ), true);

        // To test that variadic arguments with a specified type does not match anything
        // other than its specified type
        assert_eq!(test_matches_signature(
            ItemType::Function {
                args: vec![
                    FuncArgType::Arg(0),
                    FuncArgType::Variadic(Some(2)),
                ],
                return_type: 22,
            },
            &vec![
                FuncArgType::Arg(0),
                FuncArgType::Arg(2),
                FuncArgType::Arg(2),
                FuncArgType::Arg(2),
                FuncArgType::Array {item: 0, size: None},
            ],
            22,
        ), false);

        assert_eq!(test_matches_signature(
            ItemType::Function {
                args: vec![
                    FuncArgType::Arg(0),
                    FuncArgType::Variadic(None),
                ],
                return_type: 22,
            },
            &vec![
                FuncArgType::Arg(0),
            ],
            22,
        ), true);
    }

    fn test_matches_signature(
        typ: ItemType,
        expected_args: &Vec<FuncArgType>,
        expected_return_type: TypeId,
    ) -> bool {
        typ.matches_signature(expected_args, expected_return_type)
    }
}
