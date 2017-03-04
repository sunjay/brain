use super::operation::Operations;

use parser::Identifier;

pub type OperationsResult = Result<Operations, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Unresolved name: `name`
    UnresolvedName(Identifier),
    /// `name` is not a valid type
    InvalidType(Identifier),
}
