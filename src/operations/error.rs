use super::operation::Operations;

use parser::Identifier;

pub type OperationsResult = Result<Operations, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Unresolved name: `name`
    UnresolvedName(Identifier),

    /// `name` is not a valid type
    InvalidType(Identifier),

    /// Invalid left-hand side expression used in assignment
    /// Usually because `name` is not a variable
    /// It might be a type or function or something
    InvalidLeftHandSide(Identifier),
}
