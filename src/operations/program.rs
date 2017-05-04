use parser::Program;

use super::{OperationsResult, module};
use super::scope::ScopeStack;

pub fn into_operations(scope: &mut ScopeStack, program: Program) -> OperationsResult {
    module::into_operations(scope, program.root_mod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Program;

    #[test]
    fn empty_program() {
        let mut scope = ScopeStack::new();
        let program = Program::empty();

        let ops = into_operations(&mut scope, program).unwrap();
        assert_eq!(ops.len(), 1);
    }
}
