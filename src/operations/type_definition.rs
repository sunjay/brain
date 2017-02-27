use parser::{Pattern, TypeDefinition, Expression};

use super::{Operation};
use super::scope::ScopeStack;
use super::item_type::ItemType;

pub fn resolve_type(type_def: TypeDefinition, scope: &mut ScopeStack) -> ItemType {
    unimplemented!();
}
