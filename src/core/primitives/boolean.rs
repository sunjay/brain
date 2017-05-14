use parser::Identifier;
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
    //TODO

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
