use parser::{Expression, Identifier};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::{ScopeStack, TypeId};
use operations::expression;

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
            unimplemented!();
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

    let write_method_name = type_name.clone().concat(Identifier::from("print"));
    scope.declare_builtin_function(
        write_method_name.clone(),
        ItemType::Function {
            args: vec![
                FuncArgType::Arg(stdout_type),
                FuncArgType::Variadic(None),
            ],
            return_type: unit_type,
        },
        |scope, args, _| {
            unimplemented!();
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
        move |scope, args, _| {
            let mut ops = expression::call(scope, Expression::Identifier(write_method_name.clone()), args)?;

            // Write a newline using a temporary cell
            //TODO: ops.extend(...);
            unimplemented!();

            Ok(ops)
        }
    );

    stdout_type
}
