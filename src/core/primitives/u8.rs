use parser::Identifier;
use operations::{Operation, Error};
use operations::item_type::{ItemType, FuncArgType};
use operations::scope::{ScopeStack, TypeId};

pub fn define_u8(scope: &mut ScopeStack) -> TypeId {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let u8_type = scope.declare_type(
        Identifier::from("u8"),
        ItemType::Primitive(1)
    );

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

    u8_type
}
