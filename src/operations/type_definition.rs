use parser::TypeDefinition;
use parser::TypeDefinition::*;

use super::Error;
use super::scope::{TypeId, ScopeStack, ScopeItem};

/// Attempts to resolve the TypeId of a given type definition
pub fn resolve_type_id(type_def: TypeDefinition, scope: &ScopeStack) -> Result<TypeId, Error> {
    match type_def {
        // We return the first declaration found because we want to use the latest definition
        // of the type that we are defining
        Name {name} => scope.lookup(&name).first().ok_or_else(|| {
            Error::UnresolvedName(name.clone())
        }).and_then(|it| match **it {
            ScopeItem::Type(id) => Ok(id),
            ScopeItem::BuiltInFunction {id, ..} => Ok(id),
            _ => Err(Error::InvalidType(name)),
        }),
        //TODO: Deal with infinitely sized (self-referential) types
        Array {type_def, size} => unimplemented!(),
    }
}
