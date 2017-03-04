use parser::{Expression, Block};

use super::{Error};
use super::{Operation, OperationsResult, expression};
use super::scope::{ScopeStack, ScopeItem};

pub fn into_operations(
    condition: Expression,
    body: Block,
    scope: &mut ScopeStack,
) -> OperationsResult {
    unimplemented!();
}
