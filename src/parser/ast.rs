use std::iter::FromIterator;
use std::convert::From;
use std::str::FromStr;

use pest::prelude::*;

use super::{Rdp, ParseError};
use operations::{self, OperationsResult};
use operations::scope::ScopeStack;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub root_mod: Module,
}

impl Program {
    pub fn empty() -> Program {
        Program {
            root_mod: Module::empty(),
        }
    }

    pub fn into_operations(self, global_scope: &mut ScopeStack) -> OperationsResult {
        operations::from_ast(global_scope, self)
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parser = Rdp::new(StringInput::new(input));

        if parser.module() {
            Ok(Program {
                root_mod: parser.module_ast(),
            })
        }
        else {
            let (expected, pos) = parser.expected();
            let (line, col) = parser.input().line_col(pos);
            Err(ParseError {
                line: line,
                col: col,
                expected: expected,
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub body: Block,
}

impl Module {
    pub fn empty() -> Module {
        Module {
            body: vec![Statement::Expression {expr: Expression::UnitLiteral}],
        }
    }
}

impl From<Block> for Module {
    fn from(block: Block) -> Self {
        Module {
            body: block
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Comment(String),
    Declaration {
        pattern: Pattern,
        type_def: TypeDefinition,
        expr: Option<Expression>,
    },
    Assignment {
        lhs: Identifier,
        expr: Expression,
    },
    WhileLoop {
        condition: Expression,
        body: Block,
    },
    Expression {
        expr: Expression,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeDefinition {
    Name {
        name: Identifier,
    },
    Array {
        type_def: Box<TypeDefinition>,
        size: Option<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    UnitLiteral,
    ByteLiteral(Vec<u8>),
    Identifier(Identifier),
    Number(Number),
    Call {
        method: Box<Expression>,
        args: CallArgs,
    },
    Access {
        // target can be another field access, an identifier, or even a literal, etc.
        target: Box<Expression>,
        // field can only ever be an Identifier
        // Numbers may also be fields when #40 is implemented
        // https://github.com/brain-lang/brain/issues/40
        field: Identifier,
    },
    Branch {
        /// Condition to be executed to determine which block
        /// is run
        condition: Box<Expression>,
        /// executed if the condition is non-zero
        body: Block,
        /// (optional) executed if the condition is zero
        otherwise: Option<Block>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier(Vec<String>);

impl Identifier {
    // NOTE: Do not implement ::new() for Identifier
    // Having an empty Identifier does not make any sense!

    // Concatenates this identifier with another identifier and returns
    // a new identifier
    pub fn concat<T>(self, other: T) -> Identifier where T: IntoIterator<Item=String> {
        self.into_iter().chain(other).collect()
    }
}

impl FromStr for Identifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Identifier(s.split("::").map(|s| s.to_owned()).collect()))
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(s: &'a str) -> Identifier {
        s.parse().unwrap()
    }
}

impl IntoIterator for Identifier {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<String> for Identifier {
    fn from_iter<I: IntoIterator<Item=String>>(iter: I) -> Self {
        Identifier(iter.into_iter().collect())
    }
}

pub type Block = Vec<Statement>;
pub type Number = i32;
pub type CallArgs = Vec<CallArg>;
pub type CallArg = Expression;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_identifiers() {
        let ident = Identifier::from("foo::bar::Bar::spam")
            .concat(Identifier::from("car::bar::star"));

        let concated = Identifier::from("foo::bar::Bar::spam::car::bar::star");

        assert_eq!(ident, concated);
    }
}
