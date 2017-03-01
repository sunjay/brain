use parser::{Pattern, TypeDefinition, Expression};

use super::{Operation, type_definition, expression};
use super::scope::ScopeStack;
use super::item_type::ItemType;

pub fn into_operations(
    pattern: Pattern,
    type_def: TypeDefinition,
    expr: Option<Expression>,
    scope: &mut ScopeStack,
) -> Vec<Operation> {
    let typ = type_definition::resolve_type(type_def, scope);

    let name = match pattern {
        Pattern::Identifier(name) => name,
    };

    let mem = scope.declare(name, &typ);

    if let Some(expr) = expr {
        expression::into_operations(expr, &typ, mem, scope)
    }
    else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use operations::item_type::ItemType;

    #[test]
    fn declaration_only() {
        // When only doing a declaration, no operations should be generated
        // since there is no expression to actually evaluate
        let mut scope = ScopeStack::new();
        scope.declare("u8".to_owned(), &ItemType::Primitive(1));

        let ops = into_operations(
            Pattern::Identifier("foo".to_owned()),
            TypeDefinition::Name {name: "u8".to_owned()},
            None,
            &mut scope
        );
        assert_eq!(ops.len(), 0);
    }
}
