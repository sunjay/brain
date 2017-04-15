use super::item_type::ItemType;
use super::operation::Operations;

use parser::Identifier;

pub type OperationsResult = Result<Operations, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Unresolved name: `name`
    UnresolvedName(Identifier),

    /// No field `field` on type `target_type`
    UnresolvedField {
        /// The target of the field acccess
        target_type: ItemType,
        /// The field name that was attempted
        field: Identifier,
    },

    /// `name` is not a valid type
    InvalidType(Identifier),

    /// Invalid left-hand side expression used in assignment
    /// Usually because `name` is not a variable
    /// It might be a type or function or something
    InvalidLeftHandSide(Identifier),

    /// Mismatched types:
    ///     expected type: `expected`
    ///     found type: `found`
    MismatchedTypes {
        expected: ItemType,
        found: ItemType,
    },

    /// Same as MismatchedTypes but for literals
    /// Found is a special literal type like `{unsigned integer}`
    /// Mismatched types:
    ///     expected type: `expected`
    ///     found type: `found`
    MismatchedLiteral {
        expected: ItemType,
        found: String,
    },

    /// Overflowing literal: literal out of range for `typ`
    OverflowingLiteral {
        typ: ItemType,
    },

    /// Unsupported array type declaration
    /// * Nested array types are NOT currently supported
    /// * Array sizes that are not numeric literals are NOT currently supported
    /// * Negative array sizes are NOT supported
    /// * Cannot infer without expression
    UnsupportedArrayType {
        name: Identifier,
    },
}
