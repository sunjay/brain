use parser::Module;

use super::{OperationsResult, block};
use super::scope::ScopeStack;

pub fn into_operations(module: Module, scope: &mut ScopeStack) -> OperationsResult {
    block::into_operations(module.body, scope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_module() {
        let mut scope = ScopeStack::new();
        let module = Module::new();

        let ops = into_operations(module, &mut scope).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
