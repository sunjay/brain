use parser::{Expression, Identifier};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::ScopeStack;
use operations::expression;

pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    define_stdin(scope);
    define_stdout(scope);
}

fn define_stdin(scope: &mut ScopeStack) {
    let type_name = Identifier::from("std::io::Stdin");
    let stdin_type = scope.declare_type(
        type_name.clone(),
        ItemType::Struct {},
    );
    scope.declare(Identifier::from("stdin"), stdin_type);

    let unit_type = scope.unit_type_id();

    scope.declare_builtin_function(
        type_name.clone().concat(Identifier::from("read_exact")),
        ItemType::Function {
            args: vec![
                FuncArgType::Arg(stdin_type),
                // Need an arg here for the thing being read into
                unimplemented!(),
            ],
            return_type: unit_type,
        },
        |scope, args, _| {
            unimplemented!();
        }
    );
}

fn define_stdout(scope: &mut ScopeStack) {
    let type_name = Identifier::from("std::io::Stdout");
    let stdout_type = scope.declare_type(
        type_name.clone(),
        ItemType::Struct {},
    );
    scope.declare(Identifier::from("stdout"), stdout_type);

    let unit_type = scope.unit_type_id();

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
}
