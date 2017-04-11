use parser::{Expression};

use operations::{OperationsResult};
use operations::scope::{ScopeStack, FuncArgs};

/// Call the provided method with the given arguments
pub fn call(scope: &mut ScopeStack, method: Expression, args: FuncArgs) -> OperationsResult {
    //TODO: Refactor store_number to use this
    unimplemented!();
}
