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
        ItemType::Array {item: None, size: None},
    );
    scope.register_primitive("array", array_type);

    array_type
}

#[cfg(test)]
mod tests {
    use super::*;

    use operations::scope::{ScopeType};

    #[test]
    fn defines_primitive() {
        let mut scope = ScopeStack::new();
        define_array(&mut scope);

        let array_type_id = match **scope.lookup_type(&Identifier::from("[T; N]")).first().unwrap() {
            ScopeType::Type(id) => id,
        };
        assert_eq!(scope.primitives().array(), array_type_id);
    }
}
