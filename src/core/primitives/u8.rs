use parser::Identifier;
use operations::{Operation, Error};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::{ScopeStack, ScopeItem, TypeId};

pub fn define_u8(scope: &mut ScopeStack, bool_type: TypeId) -> TypeId {
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
            args: vec![FuncArgType::Arg(u8_type)],
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
                        amount: value as u8,
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

    scope.declare_builtin_function(
        Identifier::from("std::cmp::PartialEq::eq"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(u8_type), FuncArgType::Arg(u8_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let u8_type = scope.primitives().u8();

            Ok(match (&args[0], &args[1]) {
                (&ScopeItem::TypedBlock {memory: x, ..}, &ScopeItem::TypedBlock {memory: y, ..}) => {
                    let temp_x = scope.allocate(u8_type);
                    let temp_y = scope.allocate(u8_type);

                    vec![Operation::TempAllocate {
                        temp: temp_x,
                        body: vec![Operation::TempAllocate {
                            temp: temp_y,
                            body: vec![
                                Operation::Copy {
                                    source: x.position(),
                                    target: temp_x.position(),
                                    size: x.size(),
                                },
                                Operation::Copy {
                                    source: y.position(),
                                    target: temp_y.position(),
                                    size: y.size(),
                                },

                                // Algorithm:
                                //
                                // z = x == y
                                // x[y-x-]z+y[z-y[-]]
                                Operation::Loop {
                                    cond: temp_x.position(),
                                    body: vec![
                                        Operation::Decrement {
                                            target: temp_y.position(),
                                            amount: 1,
                                        },
                                        Operation::Decrement {
                                            target: temp_x.position(),
                                            amount: 1,
                                        },
                                    ],
                                },
                                Operation::Increment {
                                    target: target.position(),
                                    amount: 1,
                                },
                                Operation::Loop {
                                    cond: temp_y.position(),
                                    body: vec![
                                        Operation::Decrement {
                                            target: target.position(),
                                            amount: 1,
                                        },
                                        Operation::Zero {
                                            target: temp_y,
                                        },
                                    ],
                                },
                            ],
                            should_zero: false,
                        }],
                        should_zero: false,
                    }]
                },
                _ => unreachable!(),
            })
        }
    );

    scope.declare_builtin_function(
        Identifier::from("std::cmp::PartialEq::ne"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(u8_type), FuncArgType::Arg(u8_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let u8_type = scope.primitives().bool();

            Ok(match (&args[0], &args[1]) {
                (&ScopeItem::TypedBlock {memory: x, ..}, &ScopeItem::TypedBlock {memory: y, ..}) => {
                    let temp_x = scope.allocate(u8_type);
                    let temp_y = scope.allocate(u8_type);

                    vec![Operation::TempAllocate {
                        temp: temp_x,
                        body: vec![Operation::TempAllocate {
                            temp: temp_y,
                            body: vec![
                                Operation::Copy {
                                    source: x.position(),
                                    target: temp_x.position(),
                                    size: x.size(),
                                },
                                Operation::Copy {
                                    source: y.position(),
                                    target: temp_y.position(),
                                    size: y.size(),
                                },

                                // Algorithm:
                                //
                                // z = x != y
                                // x[y-x-]y[z+y[-]]
                                Operation::Loop {
                                    cond: temp_x.position(),
                                    body: vec![
                                        Operation::Decrement {
                                            target: temp_y.position(),
                                            amount: 1,
                                        },
                                        Operation::Decrement {
                                            target: temp_x.position(),
                                            amount: 1,
                                        },
                                    ],
                                },
                                Operation::Loop {
                                    cond: temp_y.position(),
                                    body: vec![
                                        Operation::Increment {
                                            target: target.position(),
                                            amount: 1,
                                        },
                                        Operation::Zero {
                                            target: temp_y,
                                        },
                                    ],
                                },
                            ],
                            should_zero: false,
                        }],
                        should_zero: false,
                    }]
                },
                _ => unreachable!(),
            })
        }
    );

    // Need this so that this next method definition does not overwrite the previous one
    scope.push_scope();

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
        move |scope, args, _| {
            match args[0] {
                ScopeItem::Array {memory, ..} => Ok(vec![
                    Operation::Write {
                        target: memory,
                    },
                ]),
                ScopeItem::ByteLiteral(ref bytes) => {
                    let u8_type = scope.primitives().u8();
                    let mem = scope.allocate(u8_type);

                    Ok(vec![
                        Operation::TempAllocate {
                            temp: mem,
                            body: bytes.iter().flat_map(|&ch| vec![
                                Operation::Increment {
                                    target: mem.position(),
                                    amount: ch,
                                },
                                Operation::Write {
                                    target: mem,
                                },
                                Operation::Decrement {
                                    target: mem.position(),
                                    amount: ch,
                                },
                            ]).collect(),
                            should_zero: false,
                        },
                    ])
                },
                _ => unreachable!(),
            }
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
        // Just put a random type ID for bool_type argument since it doesn't matter for this test
        define_u8(&mut scope, 1000);

        let u8_type_id = match **scope.lookup_type(&Identifier::from("u8")).first().unwrap() {
            ScopeType::Type(id) => id,
        };
        assert_eq!(scope.primitives().u8(), u8_type_id);
    }
}
