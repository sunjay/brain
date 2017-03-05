use parser::Program;

use super::{OperationsResult, module};
use super::scope::ScopeStack;

pub fn into_operations(program: Program, scope: &mut ScopeStack) -> OperationsResult {
    module::into_operations(program.root_mod, scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Program;

    #[test]
    fn empty_program() {
        let mut scope = ScopeStack::new();
        let program = Program::new();

        let ops = into_operations(program, &mut scope).unwrap();
        assert_eq!(ops.len(), 0);
    }
}
