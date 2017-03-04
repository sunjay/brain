use std::convert::From;
use std::str::FromStr;

use pest::prelude::*;

use super::{Rdp, ParseError};
use operations::{self, OperationsResult};

#[derive(Debug, PartialEq, Clone)]
pub struct Program(Vec<Statement>);

impl Program {
    pub fn new() -> Program {
        Program(Vec::new())
    }

    pub fn into_operations(self) -> OperationsResult {
        operations::from_ast(self)
    }
}

impl From<Vec<Statement>> for Program {
    fn from(statements: Vec<Statement>) -> Self {
        Program(statements)
    }
}

impl IntoIterator for Program {
    type Item = Statement;
    type IntoIter = ::std::vec::IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parser = Rdp::new(StringInput::new(input));

        if parser.program() {
            Ok(parser.ast())
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
    StringLiteral(String),
    Identifier(Identifier),
    Number(Number),
    Call {
        method: Box<Expression>,
        args: FuncArgs,
    },
    Access {
        target: Box<Expression>,
        field: Box<Expression>,
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

pub type Block = Vec<Statement>;
pub type Number = i32;
pub type FuncArgs = Vec<FuncArg>;
pub type FuncArg = Expression;
