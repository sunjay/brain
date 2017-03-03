use parser::{Pattern, TypeDefinition, Expression};

use super::{Operation};
use super::scope::ScopeStack;
use super::item_type::ItemType;

pub fn resolve_type(type_def: TypeDefinition, scope: &ScopeStack) -> Option<ItemType> {
    match type_def {
        TypeDefinition::Name {name} => scope.lookup(&name).first().map(|it| it.type_def.clone()),
        //TODO: Deal with infinitely sized (self-referential) types
        TypeDefinition::Array {type_def, size} => unimplemented!(),
    }
}
