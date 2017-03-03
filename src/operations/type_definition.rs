use parser::{Pattern, TypeDefinition, Expression};

use super::{Operation};
use super::scope::ScopeStack;
use super::item_type::ItemType;

pub fn resolve_type(type_def: TypeDefinition, scope: &ScopeStack) -> Option<ItemType> {
    match type_def {
        // We return the first declaration found because we want to use the latest definition
        // of the type that we are defining
        TypeDefinition::Name {name} => scope.lookup(&name).first().map(|it| it.type_def()),
        //TODO: Deal with infinitely sized (self-referential) types
        TypeDefinition::Array {type_def, size} => unimplemented!(),
    }
}
