pub mod scope;
pub mod item_type;
pub mod program;
pub mod module;
pub mod block;
pub mod statement;
pub mod declaration;
pub mod assignment;
pub mod while_loop;
pub mod expression;

mod operation;
mod primitives;
mod error;

pub use self::expression::Target;
pub use self::operation::*;
pub use self::error::*;

use parser::Program;

use self::scope::ScopeStack;

pub fn from_ast(global_scope: &mut ScopeStack, ast: Program) -> OperationsResult {
    program::into_operations(global_scope, ast)
}
