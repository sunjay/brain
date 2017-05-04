use parser::Module;
use memory::MemoryBlock;

use super::{OperationsResult, block, Target};
use super::scope::ScopeStack;

pub fn into_operations(scope: &mut ScopeStack, module: Module) -> OperationsResult {
    let unit_type = scope.primitives().unit();
    block::into_operations(scope, module.body, Target::TypedBlock {
        type_id: unit_type,
        memory: MemoryBlock::default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_module() {
        let mut scope = ScopeStack::new();
        let module = Module::empty();

        let ops = into_operations(&mut scope, module).unwrap();
        assert_eq!(ops.len(), 1);
    }
}
