use parser::Identifier;
use memory::MemoryBlock;
use operations::{Operation};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::{ScopeStack, ScopeItem, TypeId};

pub fn define_boolean(scope: &mut ScopeStack) -> TypeId {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let bool_type = scope.declare_type(
        Identifier::from("bool"),
        ItemType::Primitive(1)
    );
    scope.register_primitive("bool", bool_type);

    scope.declare_constant(
        Identifier::from("true"),
        bool_type,
        vec![1u8]
    );

    scope.declare_constant(
        Identifier::from("false"),
        bool_type,
        vec![0u8]
    );

    let unit_type = scope.primitives().unit();

    scope.declare_builtin_function(
        // Special method for displaying this primitive (used from print/println)
        // This name is such that it could never be called directly
        // from the language itself
        Identifier::from("std::fmt::Display::print"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(bool_type)],
            return_type: unit_type,
        },
        move |scope, args, _| {
            match args[0] {
                ScopeItem::TypedBlock {memory, ..} => Ok(vec![
                    Operation::Increment {
                        target: memory.position(),
                        amount: b'0',
                    },
                    Operation::Write {
                        target: memory,
                    },
                    // Need to make sure we reverse the operation after so this function has no
                    // side effects
                    Operation::Decrement {
                        target: memory.position(),
                        amount: b'0',
                    },
                ]),
                ScopeItem::Constant {type_id, ref bytes} => {
                    let bool_type = scope.primitives().bool();
                    debug_assert_eq!(type_id, bool_type);

                    let u8_type = scope.primitives().u8();
                    let mem = scope.allocate(u8_type);

                    // This code assumes that this is 1
                    debug_assert_eq!(bytes.len(), 1);
                    let value = b'0' + bytes[0];
                    Ok(vec![Operation::TempAllocate {
                        temp: mem,
                        body: vec![
                            Operation::Increment {
                                target: mem.position(),
                                amount: value,
                            },
                            Operation::Write {
                                target: mem,
                            },
                            Operation::Decrement {
                                target: mem.position(),
                                amount: value,
                            },
                        ],
                        should_zero: false,
                    }])
                },
                _ => unreachable!(),
            }
        }
    );

    scope.declare_builtin_function(
        Identifier::from("std::ops::Not"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(bool_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let bool_type = scope.primitives().bool();
            match args[0] {
                ScopeItem::TypedBlock {memory, ..} => {
                    let temp0 = scope.allocate(bool_type);
                    Ok(vec![Operation::TempAllocate {
                        temp: temp0,
                        body: vec![
                            // Need to copy to the destination because the algorithm consumes
                            // the operand
                            // After this copy the operand becomes `target` (x in the algorithm)
                            Operation::Copy {
                                source: memory.position(),
                                target: target.position(),
                                size: target.size(),
                            },

                            // Algorithm from: https://esolangs.org/wiki/Brainfuck_algorithms#x_.3D_not_x_.28boolean.2C_logical.29
                            //
                            // temp0[-]+
                            // x[-temp0-]temp0[x+temp0-]
                            //
                            // Results in x = !x and temp0 = 0
                            Operation::Increment {
                                target: temp0.position(),
                                amount: 1,
                            },
                            Operation::Loop {
                                cond: target.position(),
                                body: vec![
                                    Operation::Decrement {
                                        target: target.position(),
                                        amount: 1,
                                    },
                                    Operation::Decrement {
                                        target: temp0.position(),
                                        amount: 1,
                                    },
                                ],
                            },
                            Operation::Loop {
                                cond: temp0.position(),
                                body: vec![
                                    Operation::Increment {
                                        target: target.position(),
                                        amount: 1,
                                    },
                                    Operation::Decrement {
                                        target: temp0.position(),
                                        amount: 1,
                                    },
                                ],
                            },
                        ],
                        should_zero: false,
                    }])
                },
                ScopeItem::Constant {type_id, ref bytes} => {
                    debug_assert_eq!(type_id, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(bytes.len(), 1);

                    Ok(match bytes[0] {
                        //TODO: Verify if this makes sense or if we should explicitly zero
                        1 => Vec::new(),
                        0 => vec![
                            Operation::Increment {
                                target: target.position(),
                                amount: 1,
                            },
                        ],
                        _ => unreachable!(),
                    })
                },
                _ => unreachable!(),
            }
        }
    );

    // boolean and operator (operator&&) and boolean or operator (operator||)
    // These operations have special names because they are not regular functions
    // that can be defined or overloaded
    // The reason these are not definable is because they have to support short
    // circuiting. This behaviour cannot be modelled by a trait, so these special
    // operators are not definable by the user.
    scope.declare_builtin_function(
        Identifier::from("operator||"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(bool_type), FuncArgType::Arg(bool_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let bool_type = scope.primitives().bool();

            Ok(match (&args[0], &args[1]) {
                (&ScopeItem::Constant {type_id: xt, bytes: ref xb}, &ScopeItem::Constant {type_id: yt, bytes: ref yb}) => {
                    debug_assert_eq!(xt, bool_type);
                    debug_assert_eq!(yt, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(xb.len(), 1);
                    debug_assert_eq!(yb.len(), 1);

                    // The `> 0` converts the byte-representation into a Rust boolean
                    let x = xb[0] > 0;
                    let y = yb[0] > 0;

                    match x || y {
                        false => Vec::new(),
                        true => vec![Operation::Increment {
                            target: target.position(),
                            amount: 1,
                        }],
                    }
                },
                /// Thanks to certain properties of booleans, we can evaluate certain things during
                /// compilation and avoid a lot of extra computation.
                (&ScopeItem::Constant {type_id: const_type, ref bytes}, &ScopeItem::TypedBlock {type_id: other_type, memory}) |
                (&ScopeItem::TypedBlock {type_id: other_type, memory}, &ScopeItem::Constant {type_id: const_type, ref bytes}) => {
                    debug_assert_eq!(const_type, bool_type);
                    debug_assert_eq!(other_type, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(bytes.len(), 1);

                    match bytes[0] {
                        // If either operand is true, the || operator always returns true
                        1 => vec![Operation::Increment {
                            target: target.position(),
                            amount: 1,
                        }],
                        // If either operand is false, the || operator always returns its other operand
                        0 => vec![Operation::Copy {
                            source: memory.position(),
                            target: target.position(),
                            size: memory.size(),
                        }],
                        _ => unreachable!(),
                    }
                },
                (&ScopeItem::TypedBlock {memory: x, ..}, &ScopeItem::TypedBlock {memory: y, ..}) => {
                    let temp0 = scope.allocate(bool_type);
                    let temp_x = scope.allocate(bool_type);
                    let temp_y = scope.allocate(bool_type);
                    let ops = vec![
                        // The algorithm below consumes x and y so we need to copy them first to
                        // avoid any problems with that
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

                        // Algorithm from: https://esolangs.org/wiki/Brainfuck_algorithms#z_.3D_x_or_y_.28boolean.2C_logical.29
                        //
                        // z[-]
                        // temp0[-]+
                        // x[z+temp0-x-]
                        // temp0[-
                        //  y[z+y-]
                        // ]
                        // y[-]
                        Operation::Increment {
                            target: temp0.position(),
                            amount: 1,
                        },
                        Operation::Loop {
                            cond: temp_x.position(),
                            body: vec![
                                Operation::Increment {
                                    target: target.position(),
                                    amount: 1,
                                },
                                Operation::Decrement {
                                    target: temp0.position(),
                                    amount: 1,
                                },
                                Operation::Decrement {
                                    target: temp_x.position(),
                                    amount: 1,
                                },
                            ],
                        },
                        Operation::Loop {
                            cond: temp0.position(),
                            body: vec![
                                Operation::Decrement {
                                    target: temp0.position(),
                                    amount: 1,
                                },
                                Operation::Loop {
                                    cond: temp_y.position(),
                                    body: vec![
                                        Operation::Increment {
                                            target: target.position(),
                                            amount: 1,
                                        },
                                        Operation::Decrement {
                                            target: temp_y.position(),
                                            amount: 1,
                                        },
                                    ],
                                },
                            ],
                        },
                        Operation::Zero {
                            target: temp_y,
                        },
                    ];

                    vec![Operation::TempAllocate {
                        temp: temp0,
                        body: vec![Operation::TempAllocate {
                            temp: temp_x,
                            body: vec![Operation::TempAllocate {
                                temp: temp_y,
                                body: ops,
                                should_zero: false,
                            }],
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
        Identifier::from("operator&&"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(bool_type), FuncArgType::Arg(bool_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let bool_type = scope.primitives().bool();

            Ok(match (&args[0], &args[1]) {
                (&ScopeItem::Constant {type_id: xt, bytes: ref xb}, &ScopeItem::Constant {type_id: yt, bytes: ref yb}) => {
                    debug_assert_eq!(xt, bool_type);
                    debug_assert_eq!(yt, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(xb.len(), 1);
                    debug_assert_eq!(yb.len(), 1);

                    // The `> 0` converts the byte-representation into a Rust boolean
                    let x = xb[0] > 0;
                    let y = yb[0] > 0;

                    match x && y {
                        false => Vec::new(),
                        true => vec![Operation::Increment {
                            target: target.position(),
                            amount: 1,
                        }],
                    }
                },
                /// Thanks to certain properties of booleans, we can evaluate certain things during
                /// compilation and avoid a lot of extra computation.
                (&ScopeItem::Constant {type_id: const_type, ref bytes}, &ScopeItem::TypedBlock {type_id: other_type, memory}) |
                (&ScopeItem::TypedBlock {type_id: other_type, memory}, &ScopeItem::Constant {type_id: const_type, ref bytes}) => {
                    debug_assert_eq!(const_type, bool_type);
                    debug_assert_eq!(other_type, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(bytes.len(), 1);

                    match bytes[0] {
                        // If either operand is false, the && operator always returns false
                        0 => Vec::new(),
                        // If either operand is true, the && operator always returns its other operand
                        1 => vec![Operation::Copy {
                            source: memory.position(),
                            target: target.position(),
                            size: memory.size(),
                        }],
                        _ => unreachable!(),
                    }
                },
                (&ScopeItem::TypedBlock {memory: x, ..}, &ScopeItem::TypedBlock {memory: y, ..}) => {
                    let temp_x = scope.allocate(bool_type);
                    let temp_y = scope.allocate(bool_type);

                    let ops = vec![
                        // The algorithm below consumes x and y so we need to copy them first to
                        // avoid any problems with that
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

                        // Algorithm from: https://esolangs.org/wiki/Brainfuck_algorithms#z_.3D_x_and_y_.28boolean.2C_logical.29
                        //
                        // z[-]
                        // x[
                        //  y[z+y-]
                        //  x-
                        // ]
                        // y[-]
                        Operation::Loop {
                            cond: temp_x.position(),
                            body: vec![
                                Operation::Loop {
                                    cond: temp_y.position(),
                                    body: vec![
                                        Operation::Increment {
                                            target: target.position(),
                                            amount: 1,
                                        },
                                        Operation::Decrement {
                                            target: temp_y.position(),
                                            amount: 1,
                                        },
                                    ],
                                },
                                Operation::Decrement {
                                    target: temp_x.position(),
                                    amount: 1,
                                },
                            ],
                        },
                        Operation::Zero {
                            target: temp_y,
                        },
                    ];

                    vec![Operation::TempAllocate {
                        temp: temp_x,
                        body: vec![Operation::TempAllocate {
                            temp: temp_y,
                            body: ops,
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
        Identifier::from("std::cmp::PartialEq::eq"),
        ItemType::Function {
            args: vec![FuncArgType::Arg(bool_type), FuncArgType::Arg(bool_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let bool_type = scope.primitives().bool();

            let cmp_eq = |x: MemoryBlock, y: MemoryBlock| vec![
                // Algorithm:
                //
                // z = x == y
                // x[y-x-]z+y[z-y[-]]
                Operation::Loop {
                    cond: x.position(),
                    body: vec![
                        Operation::Decrement {
                            target: y.position(),
                            amount: 1,
                        },
                        Operation::Decrement {
                            target: x.position(),
                            amount: 1,
                        },
                    ],
                },
                Operation::Increment {
                    target: target.position(),
                    amount: 1,
                },
                Operation::Loop {
                    cond: y.position(),
                    body: vec![
                        Operation::Decrement {
                            target: target.position(),
                            amount: 1,
                        },
                        Operation::Zero {
                            target: y,
                        },
                    ],
                },
            ];

            Ok(match (&args[0], &args[1]) {
                (&ScopeItem::Constant {type_id: xt, bytes: ref xb}, &ScopeItem::Constant {type_id: yt, bytes: ref yb}) => {
                    debug_assert_eq!(xt, bool_type);
                    debug_assert_eq!(yt, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(xb.len(), 1);
                    debug_assert_eq!(yb.len(), 1);

                    // The `> 0` converts the byte-representation into a Rust boolean
                    let x = xb[0] > 0;
                    let y = yb[0] > 0;

                    match x == y {
                        false => Vec::new(),
                        true => vec![Operation::Increment {
                            target: target.position(),
                            amount: 1,
                        }],
                    }
                },
                /// Thanks to certain properties of booleans, we can evaluate certain things during
                /// compilation and avoid a lot of extra computation.
                (&ScopeItem::Constant {type_id: const_type, ref bytes}, &ScopeItem::TypedBlock {type_id: other_type, memory}) |
                (&ScopeItem::TypedBlock {type_id: other_type, memory}, &ScopeItem::Constant {type_id: const_type, ref bytes}) => {
                    debug_assert_eq!(const_type, bool_type);
                    debug_assert_eq!(other_type, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(bytes.len(), 1);

                    let temp_x = scope.allocate(bool_type);
                    let ops = match bytes[0] {
                        0 => Vec::new(),
                        1 => vec![Operation::Increment {
                            target: temp_x.position(),
                            amount: 1,
                        }],
                        _ => unreachable!(),
                    };

                    let temp_y = scope.allocate(bool_type);

                    vec![Operation::TempAllocate {
                        temp: temp_x,
                        body: vec![Operation::TempAllocate {
                            temp: temp_y,
                            body: ops.into_iter().chain(vec![
                                Operation::Copy {
                                    source: memory.position(),
                                    target: temp_y.position(),
                                    size: memory.size(),
                                },
                            ]).chain(cmp_eq(temp_x, temp_y)).collect(),
                            should_zero: false,
                        }],
                        should_zero: false,
                    }]
                },
                (&ScopeItem::TypedBlock {memory: x, ..}, &ScopeItem::TypedBlock {memory: y, ..}) => {
                    let temp_x = scope.allocate(bool_type);
                    let temp_y = scope.allocate(bool_type);

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
                            ].into_iter().chain(cmp_eq(temp_x, temp_y)).collect(),
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
            args: vec![FuncArgType::Arg(bool_type), FuncArgType::Arg(bool_type)],
            return_type: bool_type,
        },
        move |scope, args, target| {
            let bool_type = scope.primitives().bool();

            let cmp_ne = |x: MemoryBlock, y: MemoryBlock| vec![
                // Algorithm:
                //
                // z = x != y
                // x[y-x-]y[z+y[-]]
                Operation::Loop {
                    cond: x.position(),
                    body: vec![
                        Operation::Decrement {
                            target: y.position(),
                            amount: 1,
                        },
                        Operation::Decrement {
                            target: x.position(),
                            amount: 1,
                        },
                    ],
                },
                Operation::Loop {
                    cond: y.position(),
                    body: vec![
                        Operation::Increment {
                            target: target.position(),
                            amount: 1,
                        },
                        Operation::Zero {
                            target: y,
                        },
                    ],
                },
            ];

            Ok(match (&args[0], &args[1]) {
                (&ScopeItem::Constant {type_id: xt, bytes: ref xb}, &ScopeItem::Constant {type_id: yt, bytes: ref yb}) => {
                    debug_assert_eq!(xt, bool_type);
                    debug_assert_eq!(yt, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(xb.len(), 1);
                    debug_assert_eq!(yb.len(), 1);

                    // The `> 0` converts the byte-representation into a Rust boolean
                    let x = xb[0] > 0;
                    let y = yb[0] > 0;

                    match x != y {
                        false => Vec::new(),
                        true => vec![Operation::Increment {
                            target: target.position(),
                            amount: 1,
                        }],
                    }
                },
                /// Thanks to certain properties of booleans, we can evaluate certain things during
                /// compilation and avoid a lot of extra computation.
                (&ScopeItem::Constant {type_id: const_type, ref bytes}, &ScopeItem::TypedBlock {type_id: other_type, memory}) |
                (&ScopeItem::TypedBlock {type_id: other_type, memory}, &ScopeItem::Constant {type_id: const_type, ref bytes}) => {
                    debug_assert_eq!(const_type, bool_type);
                    debug_assert_eq!(other_type, bool_type);
                    // This code assumes that this is 1
                    debug_assert_eq!(bytes.len(), 1);

                    let temp_x = scope.allocate(bool_type);
                    let ops = match bytes[0] {
                        0 => Vec::new(),
                        1 => vec![Operation::Increment {
                            target: temp_x.position(),
                            amount: 1,
                        }],
                        _ => unreachable!(),
                    };

                    let temp_y = scope.allocate(bool_type);

                    vec![Operation::TempAllocate {
                        temp: temp_x,
                        body: vec![Operation::TempAllocate {
                            temp: temp_y,
                            body: ops.into_iter().chain(vec![
                                Operation::Copy {
                                    source: memory.position(),
                                    target: temp_y.position(),
                                    size: memory.size(),
                                },
                            ]).chain(cmp_ne(temp_x, temp_y)).collect(),
                            should_zero: false,
                        }],
                        should_zero: false,
                    }]
                },
                (&ScopeItem::TypedBlock {memory: x, ..}, &ScopeItem::TypedBlock {memory: y, ..}) => {
                    let temp_x = scope.allocate(bool_type);
                    let temp_y = scope.allocate(bool_type);

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
                            ].into_iter().chain(cmp_ne(temp_x, temp_y)).collect(),
                            should_zero: false,
                        }],
                        should_zero: false,
                    }]
                },
                _ => unreachable!(),
            })
        }
    );

    bool_type
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::Identifier;
    use operations::scope::{ScopeType, ScopeItem};

    #[test]
    fn defines_primitive() {
        let mut scope = ScopeStack::new();
        define_boolean(&mut scope);

        let bool_type_id = match **scope.lookup_type(&Identifier::from("bool")).first().unwrap() {
            ScopeType::Type(id) => id,
        };
        assert_eq!(scope.primitives().bool(), bool_type_id);
    }

    #[test]
    fn constants() {
        let mut scope = ScopeStack::new();
        define_boolean(&mut scope);

        let bool_type_id = match **scope.lookup_type(&Identifier::from("bool")).first().unwrap() {
            ScopeType::Type(id) => id,
        };

        let true_bytes = match **scope.lookup(&Identifier::from("true")).first().unwrap() {
            ScopeItem::Constant {type_id, ref bytes} => {
                assert_eq!(type_id, bool_type_id);
                assert_eq!(*bytes, vec![1]);
                bytes
            },
            _ => unreachable!(),
        };

        let false_bytes = match **scope.lookup(&Identifier::from("false")).first().unwrap() {
            ScopeItem::Constant {type_id, ref bytes} => {
                assert_eq!(type_id, bool_type_id);
                assert_eq!(*bytes, vec![0]);
                bytes
            },
            _ => unreachable!(),
        };

        // No matter what the value of both, this property must always hold
        assert!(true_bytes != false_bytes);
    }
}
