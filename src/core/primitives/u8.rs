use parser::Identifier;
use operations::item_type::ItemType;
use operations::scope::ScopeStack;

pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let u8_type = scope.declare_type(
        Identifier::from("u8"),
        ItemType::Primitive(1)
    );

    // Special function implemented for this type signals to the compiler
    // that this type can be created from a literal
    scope.declare_builtin_function(
        // This name is such that it could never be called directly
        // from the language itself
        Identifier::from("{unsigned integer}"),
        ItemType::Function {
            // The arguments aren't important here since this is just a placeholder
            args: vec![],
            // Return type signifies which type we are declaring supports integer literals
            return_type: u8_type,
        },
        // Empty placeholder
        |_, _| Ok(Vec::new())
    )
}
