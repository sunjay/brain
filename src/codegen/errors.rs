#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    // Illegal redeclaration of a name
    IllegalRedeclaration {
        name: String,
    },

    // Name used before it was declared
    UndeclaredIdentifier {
        name: String,
    },

    // Tried to declare a zero size variable
    DeclaredZeroSize {
        name: String,
    },

    // Declaration contained a size, but it was invalid
    DeclaredIncorrectSize {
        name: String,
        expected: usize,
        actual: usize,
    },

    // The expression assigned to a variable `name` was the incorrect size
    IncorrectSizedExpression {
        name: String,
        expected: usize,
        actual: usize,
    },

    // Cannot assign a name to itself since that doesn't make any sense
    SelfAssignment {
        name: String,
    },

    // We do not support `in foo[]` since we do not have dynamic strings
    UnspecifiedInputSizeUnsupported {
        name: String,
    }
}
