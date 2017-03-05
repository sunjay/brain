pub mod scope;
pub mod item_type;
pub mod program;
pub mod module;
pub mod block;
pub mod statement;
pub mod declaration;
pub mod assignment;
pub mod while_loop;
pub mod type_definition;
pub mod expression;

mod operation;
mod error;

pub use self::operation::*;
pub use self::error::*;

use parser::Program;

use self::scope::ScopeStack;

pub fn from_ast(ast: Program) -> OperationsResult {
    let mut global_scope = ScopeStack::new();
    program::into_operations(ast, &mut global_scope)
}
