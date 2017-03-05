use parser::Identifier;
use operations::item_type::ItemType;
use operations::scope::ScopeStack;

pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    let bool_type = scope.declare_type(
        Identifier::from("bool"),
        ItemType::Primitive(1)
    );

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
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::Identifier;
    use scope::ScopeItem;

    #[test]
    fn constants() {
        let mut scope = ScopeStack::new();
        populate_scope(&mut scope);

        let type_id = match **scope.lookup(&Identifier::from("bool")).first().unwrap() {
            ScopeItem::Type(id) => id,
            _ => unreachable!(),
        };

        let true_bytes = match **scope.lookup(&Identifier::from("true")).first().unwrap() {
            ScopeItem::Constant {type_id, ref bytes} => {
                assert_eq!(type_id, type_id);
                assert_eq!(*bytes, vec![1]);
                bytes
            },
            _ => unreachable!(),
        };

        let false_bytes = match **scope.lookup(&Identifier::from("false")).first().unwrap() {
            ScopeItem::Constant {type_id, ref bytes} => {
                assert_eq!(type_id, type_id);
                assert_eq!(*bytes, vec![0]);
                bytes
            },
            _ => unreachable!(),
        };

        // No matter what the value of both, this property must always hold
        assert!(true_bytes != false_bytes);
    }
}
