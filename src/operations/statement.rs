use parser::Statement;
use super::scope::ScopeStack;

use super::{Operation};

pub fn into_operations(node: Statement, scope: &mut ScopeStack) -> Vec<Operation> {
    match node {
        Statement::Comment(_) => Vec::new(),
        _ => unimplemented!(),
    }
}
