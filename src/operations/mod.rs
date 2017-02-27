mod operation;
mod scope;
mod item_type;
mod program;
mod statement;
mod declaration;
mod type_definition;
mod expression;

pub use self::operation::*;

use parser::Program;

use self::scope::ScopeStack;

pub fn from_ast(ast: Program) -> Vec<Operation> {
    let mut global_scope = ScopeStack::new();
    program::into_operations(ast, &mut global_scope)
}
