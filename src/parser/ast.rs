use std::str::FromStr;

use pest::prelude::*;

use super::{Rdp, ParseError};
use operations::{self, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct Program(Vec<Statement>);

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program(statements)
    }

    pub fn into_operations(self) -> Vec<Operation> {
        operations::from_ast(self)
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

pub type Identifier = String;
pub type Block = Vec<Statement>;
pub type Number = i32;
pub type FuncArgs = Vec<FuncArg>;
pub type FuncArg = Expression;
