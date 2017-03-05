use parser::Module;

use super::{OperationsResult, block};
use super::scope::ScopeStack;

pub fn into_operations(scope: &mut ScopeStack, module: Module) -> OperationsResult {
    block::into_operations(scope, module.body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_module() {
        let mut scope = ScopeStack::new();
        let module = Module::new();

        let ops = into_operations(&mut scope, module).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
