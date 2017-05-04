use parser::{Identifier, Pattern, TypeDefinition, Expression};

use super::{OperationsResult, expression, Target};
use super::item_type::{ItemType};
use super::scope::{TypeId, ScopeStack, ScopeItem, ScopeType, ArraySize};
use super::Error;

pub fn into_operations(
    scope: &mut ScopeStack,
    pattern: Pattern,
    type_def: TypeDefinition,
    expr: Option<Expression>,
) -> OperationsResult {
    let name = match pattern {
        Pattern::Identifier(name) => name,
    };

    // Need to always declare the variable in the scope before operating on it
    match type_def {
        // We return the first declaration found because we want to use the latest definition
        // of the type that we are defining
        TypeDefinition::Name {name: type_name} => declare_name(scope, name, type_name, expr),
        TypeDefinition::Array {type_def, size} => declare_array(scope, name, *type_def, size, expr),
    }
}

fn declare_name(
    scope: &mut ScopeStack,
    name: Identifier,
    type_name: Identifier,
    expr: Option<Expression>,
) -> OperationsResult {
    resolve_type_id(scope, &type_name).and_then(|type_id| {
        let memory = scope.declare(name, type_id);

        expr.map_or(Ok(Vec::new()), |expr| {
            expression::into_operations(scope, expr, Target::TypedBlock {type_id, memory})
        })
    })
}

fn declare_array(
    scope: &mut ScopeStack,
    name: Identifier,
    item_type_def: TypeDefinition,
    size_expr: Option<Expression>,
    expr: Option<Expression>,
) -> OperationsResult {
    match item_type_def {
        TypeDefinition::Name {name: ref item_name} => resolve_type_id(scope, item_name).and_then(|item| {
            let size = infer_size(scope, item, size_expr, &expr, &name)?;
            let memory = scope.declare_array(name, item, size);

            expr.map_or(Ok(Vec::new()), |expr| {
                expression::into_operations(scope, expr, Target::Array {item, size, memory})
            })
        }),
        //TODO: Deal with infinitely sized (self-referential) types
        TypeDefinition::Array { .. } => {
            Err(Error::UnsupportedArrayType {name: name})
        },
    }
}

/// Attempts to infer the size of the array from various pieces of information
fn infer_size(
    scope: &ScopeStack,
    item_type: TypeId,
    size_expr: Option<Expression>,
    expr: &Option<Expression>,
    name: &Identifier,
) -> Result<ArraySize, Error> {
    //TODO: Do this better. Ideally, this kind of inference would be done in a separate
    // pass with all the other inference that needs to be done.
    match size_expr {
        Some(Expression::Number(value)) if value > 0 => Ok(value as ArraySize),
        // Since no size was declared, try to infer it from the expression
        None if expr.is_some() => match *expr.as_ref().unwrap() {
            Expression::ByteLiteral(ref literal) => Ok(literal.len()),
            Expression::Identifier(ref name) => scope.lookup(name).first().ok_or_else(|| {
                Error::UnresolvedName(name.clone())
            }).and_then(|item| match **item {
                ScopeItem::ByteLiteral(ref bytes) => Ok(bytes.len()),
                ScopeItem::Array {item, size, ..} if item == item_type => Ok(size),
                ref item => Err(Error::MismatchedTypes {
                    expected: ItemType::Array {
                        item: Some(item_type),
                        size: None,
                    },
                    found: match *item {
                        //TODO: Update this when more numeric types are added
                        ScopeItem::NumericLiteral(..) => scope.get_type(scope.primitives().u8()).clone(),
                        ScopeItem::ByteLiteral(..) => unreachable!(),
                        ScopeItem::Array {item, size, ..} => ItemType::Array {item: Some(item), size: Some(size)},
                        ref arg => scope.get_type(arg.type_id()).clone(),
                    },
                }),
            }),
            Expression::Number(..) => Err(Error::MismatchedTypes {
                expected: ItemType::Array {
                    item: Some(item_type),
                    size: None,
                },
                //TODO: Update this when more numeric types are added
                found: scope.get_type(scope.primitives().u8()).clone(),
            }),
            // These are unimplemented until a more robust static analysis is implemented
            //TODO: These can all be inferred, it would just be way too messy to do it here
            Expression::UnitLiteral | Expression::Call {..} | Expression::Access {..} | Expression::Branch {..} => unimplemented!(),
        },
        _ => Err(Error::UnsupportedArrayType {name: name.clone()}),
    }
}

fn resolve_type_id(
    scope: &ScopeStack,
    name: &Identifier,
) -> Result<TypeId, Error> {
    scope.lookup_type(&name).first().ok_or_else(|| {
        Error::UnresolvedName(name.clone())
    }).and_then(|it| match **it {
        ScopeType::Type(id) => Ok(id),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::Identifier;
    use operations::item_type::ItemType;

    #[test]
    fn declaration_only() {
        // When only doing a declaration, no operations should be generated
        // since there is no expression to actually evaluate
        let mut scope = ScopeStack::new();
        scope.declare_type(Identifier::from("u8"), ItemType::Primitive(1));

        let ops = into_operations(
            &mut scope,
            Pattern::Identifier(Identifier::from("foo")),
            TypeDefinition::Name {name: Identifier::from("u8")},
            None
        ).unwrap();

        assert!(!scope.lookup(&Identifier::from("foo")).is_empty(), "No value was declared");
        assert_eq!(ops.len(), 0);
    }
}
