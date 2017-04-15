use parser::Identifier;
use operations::{Operation, Error};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::{ScopeStack, ScopeItem, TypeId};

pub fn define_u8(scope: &mut ScopeStack) -> TypeId {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let unit_type = scope.primitives().unit();

    let u8_type = scope.declare_type(
        Identifier::from("u8"),
        ItemType::Primitive(1)
    );
    scope.register_primitive("u8", u8_type);

    scope.declare_builtin_function(
        // Special method for converting from literal
        // This name is such that it could never be called directly
        // from the language itself
        Identifier::from("std::convert::From<{unsigned integer}>"),
        ItemType::Function {
            // This takes a single literal of the type specific within
            // the curly braces {} in the name
            args: vec![FuncArgType::Array {item: u8_type, size: None}],
            // Return type signifies which type we are declaring supports integer literals
            return_type: u8_type,
        },
        move |scope, args, target| {
            let value = args[0].numeric_literal_value();

            // 8 is the size of this numeric type in bytes
            // We use >= here because 0 is reserved for zero so we
            // have (2^size - 1) numbers available
            if value >= (1 << 8) {
                Err(Error::OverflowingLiteral {
                    typ: scope.get_type(u8_type).clone()
                })
            }
            else {
                Ok(vec![
                    Operation::Increment {
                        target: target.position(),
                        amount: value as usize,
                    }
                ])
            }
        }
    );

    scope.declare_builtin_function(
        // Special method for displaying this primitive (used from print/println)
        // This name is such that it could never be called directly
        // from the language itself
        Identifier::from("std::fmt::Display::print"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(u8_type)],
            return_type: unit_type,
        },
        move |_, args, _| {
            let mem = match args[0] {
                ScopeItem::TypedBlock {memory, ..} => memory,
                _ => unreachable!(),
            };

            Ok(vec![
                // Need to display numbers as numbers by first incrementing them to the proper
                // ascii code and then decrementing them again
                Operation::Increment {
                    target: mem.position(),
                    amount: b'0',
                },
                Operation::Write {
                    target: mem,
                },
                // Need to make sure we reverse the operation after or else there will be
                // unintended consequences
                Operation::Decrement {
                    target: mem.position(),
                    amount: b'0',
                },
            ])
        }
    );

    scope.declare_builtin_function(
        Identifier::from("increment"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(u8_type)],
            return_type: unit_type,
        },
        move |_scope, args, _target| {
            let mem = match args[0] {
                ScopeItem::TypedBlock {memory, ..} => memory,
                _ => unreachable!(),
            };

            debug_assert!(mem.size() == 1);

            Ok(vec![
                Operation::Increment {
                    target: mem.position(),
                    amount: 1,
                }
            ])
        }
    );

    scope.declare_builtin_function(
        Identifier::from("decrement"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(u8_type)],
            return_type: unit_type,
        },
        move |_scope, args, _target| {
            let mem = match args[0] {
                ScopeItem::TypedBlock {memory, ..} => memory,
                _ => unreachable!(),
            };

            debug_assert!(mem.size() == 1);

            Ok(vec![
                Operation::Decrement {
                    target: mem.position(),
                    amount: 1,
                }
            ])
        }
    );

    // Method on [u8; _]
    scope.declare_builtin_function(
        // Special method for displaying this primitive (used from print/println)
        // This name is such that it could never be called directly
        // from the language itself
        Identifier::from("std::fmt::Display::print"),
        ItemType::Function {
            args: vec![FuncArgType::Array {item: u8_type, size: None}],
            return_type: unit_type,
        },
        move |_scope, args, _target| {
            let mem = match args[0] {
                ScopeItem::Array {memory, ..} => memory,
                _ => unreachable!(),
            };

            Ok(vec![
                Operation::Write {
                    target: mem,
                }
            ])
        }
    );

    u8_type
}

#[cfg(test)]
mod tests {
    use super::*;

    use operations::scope::{ScopeType};

    #[test]
    fn defines_primitive() {
        let mut scope = ScopeStack::new();
        define_u8(&mut scope);

        let u8_type_id = match **scope.lookup_type(&Identifier::from("u8")).first().unwrap() {
            ScopeType::Type(id) => id,
        };
        assert_eq!(scope.primitives().u8(), u8_type_id);
    }
}
