use parser::Identifier;
use operations::item_type::ItemType;
use operations::scope::{ScopeStack, TypeId};

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
