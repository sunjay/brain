use parser::Identifier;
use operations::item_type::ItemType;
use operations::scope::ScopeStack;

pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let array_type = scope.declare_type(
        Identifier::from("[T; N]"),
        ItemType::Primitive(1)
    );
    scope.set_array_type_id(array_type);
}
