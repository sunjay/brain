use parser::Identifier;
use operations::item_type::ItemType;
use operations::scope::{ScopeStack, TypeId};

pub fn define_array(scope: &mut ScopeStack) -> TypeId {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let array_type = scope.declare_type(
        Identifier::from("[T; N]"),
        ItemType::Array {item: None},
    );
    scope.set_array_type_id(array_type);

    array_type
}
