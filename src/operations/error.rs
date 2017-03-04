use super::operation::Operations;

use parser::{Identifier, Span};

pub type OperationsResult = Result<Operations, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Unresolved name: `name`
    UnresolvedName {
        name: Identifier,
        span: Span,
    },
    /// `name` is not a valid type
    InvalidType {
        name: Identifier,
        span: Span,
    },
}
