use parser::Identifier;
use operations::item_type::ItemType;
use operations::scope::ScopeStack;

pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let stdin_type = scope.declare_type(
        Identifier::from("std::io::Stdin"),
        ItemType::Struct {},
    );
    scope.declare(Identifier::from("stdin"), stdin_type);

    let stdout_type = scope.declare_type(
        Identifier::from("std::io::Stdout"),
        ItemType::Struct {},
    );
    scope.declare(Identifier::from("stdout"), stdout_type);
}
