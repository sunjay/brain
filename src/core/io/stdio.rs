use parser::Identifier;
use memory::{MemoryBlock};

use operations::{Operation, expression, Target};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::{ScopeStack, ScopeItem, TypeId};

pub fn define_stdin(scope: &mut ScopeStack, u8_type: TypeId) -> TypeId {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let type_name = Identifier::from("std::io::Stdin");
    let stdin_type = scope.declare_type(
        type_name.clone(),
        ItemType::Struct {},
    );
    scope.declare(Identifier::from("stdin"), stdin_type);

    let unit_type = scope.primitives().unit();

    scope.declare_builtin_function(
        type_name.clone().concat(Identifier::from("read_exact")),
        ItemType::Function {
            args: vec![
                FuncArgType::Arg(stdin_type),
                // Need an arg here for the thing being read into
                FuncArgType::Array {item: u8_type, size: None},
            ],
            return_type: unit_type,
        },
        |scope, args, _| {
            match args[1] {
                ScopeItem::Array {item, memory: target, ..} => {
                    debug_assert_eq!(scope.primitives().u8(), item);

                    //TODO: This is currently an unchecked operation but it really shouldn't be
                    Ok(vec![Operation::Read {target}])
                },
                _ => unreachable!(),
            }
        }
    );

    stdin_type
}

pub fn define_stdout(scope: &mut ScopeStack) -> TypeId {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let type_name = Identifier::from("std::io::Stdout");
    let stdout_type = scope.declare_type(
        type_name.clone(),
        ItemType::Struct {},
    );
    scope.declare(Identifier::from("stdout"), stdout_type);

    let unit_type = scope.primitives().unit();

    let print_method_name = type_name.clone().concat(Identifier::from("print"));
    scope.declare_builtin_function(
        print_method_name.clone(),
        ItemType::Function {
            args: vec![
                FuncArgType::Arg(stdout_type),
                FuncArgType::Variadic(None),
            ],
            return_type: unit_type,
        },
        |scope, args, _| {
            let unit_type = scope.primitives().unit();
            args.into_iter().skip(1).map(|arg| expression::call(
                scope,
                Identifier::from("std::fmt::Display::print"),
                vec![arg],
                Target::TypedBlock {
                    type_id: unit_type,
                    memory: MemoryBlock::default(),
                }
            )).collect::<Result<Vec<_>, _>>().map(|op_vecs| {
                op_vecs.into_iter().flat_map(|ops| ops).collect()
            })
        }
    );

    scope.declare_builtin_function(
        type_name.clone().concat(Identifier::from("println")),
        ItemType::Function {
            args: vec![
                FuncArgType::Arg(stdout_type),
                FuncArgType::Variadic(None),
            ],
            return_type: unit_type,
        },
        move |scope, args, target| {
            let mut ops = expression::call(
                scope,
                print_method_name.clone(),
                args,
                Target::TypedBlock {
                    type_id: unit_type,
                    memory: target,
                }
            )?;

            // Write a newline using a temporary cell
            let u8_type = scope.primitives().u8();
            let mem = scope.allocate(u8_type);
            ops.push(Operation::TempAllocate {
                temp: mem,
                body: vec![
                    Operation::Increment {
                        target: mem.position(),
                        amount: b'\n',
                    },
                    Operation::Write {
                        target: mem,
                    },
                ],
                should_zero: true,
            });

            Ok(ops)
        }
    );

    stdout_type
}
