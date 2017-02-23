use std::str::FromStr;

use pest::prelude::*;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Program(Vec<Statement>);

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
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
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parser = Rdp::new(StringInput::new(input));

        println!("{:?}", (parser.program(), parser.end()));
        println!("{:#?}", parser.queue());
        println!("{:?}", parser.stack());
        let (expected, pos) = parser.expected();
        let (line, col) = parser.input().line_col(pos);
        println!("Expected: {:?} at line {} col {}", expected, line, col);

        println!("{:#?}", parser.parse_program());
        unimplemented!();
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
    ConditionGroup {
        // Condition expression and body block
        branches: Vec<(Expression, Block)>,
        // "default" or "else" branch body
        default: Option<Block>,
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
        //TODO: This probably isn't the right type since this should accept anything and then get
        // statically checked to ensure the correct number was put here
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
    }
}

pub type Identifier = String;
pub type Block = Vec<Statement>;
pub type Number = i32;
pub type FuncArgs = Vec<FuncArg>;
pub type FuncArg = Expression;
