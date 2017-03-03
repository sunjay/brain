use super::operation::Operations;

pub type OperationsResult = Result<Operations, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
}
