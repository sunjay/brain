use std::str::{self, FromStr};

use nom::simple_errors::Err;

const LINE_COMMENT_START: &'static str = "//";
const BLOCK_COMMENT_START: &'static str = "/*";
const BLOCK_COMMENT_END: &'static str = "*/";
const STRING_BOUNDARY: &'static str = "\"";
const OUTPUT_KEYWORD: &'static str = "out";
const STATEMENT_TERMINATOR: &'static str = ";";

#[derive(Debug, PartialEq, Clone)]
pub struct Program(Vec<Statement>);

impl IntoIterator for Program {
    type Item = Statement;
    type IntoIter = ::std::vec::IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Program {
    type Err = Err;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_all_statements(input.as_bytes()).to_result().map(|statements| Program(statements))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Comment(String),
    Output(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    StringLiteral(String),
}

named!(parse_all_statements< Vec<Statement> >, complete!(do_parse!(
    statements: fold_many0!(statement, Vec::new(), |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    }) >>
    // Ensures that we reach EOF after all the statements
    eof!() >>
    (statements)
)));

named!(statement<Statement>, ws!(alt!(
    comment => {|content: &str| Statement::Comment(content.to_owned())} |
    output => {|expr: Expression| Statement::Output(expr)}
)));

named!(comment<&str>, alt!(line_comment | block_comment));

named!(line_comment<&str>,
    map_res!(
        do_parse!(
            tag!(LINE_COMMENT_START) >>
            content: take_until_and_consume!("\n") >>
            (content)
        ),
        |s: &'a [u8]| str::from_utf8(s)
    )
);

named!(block_comment<&str>,
    map_res!(
        delimited!(
            tag!(BLOCK_COMMENT_START),
            take_until!(BLOCK_COMMENT_END),
            tag!(BLOCK_COMMENT_END)
        ),
        |s: &'a [u8]| str::from_utf8(s)
    )
);

named!(output<Expression>,
    do_parse!(
        tag!(OUTPUT_KEYWORD) >>
        expr: expression >>
        tag!(STATEMENT_TERMINATOR) >>
        (expr)
    )
);

named!(expression<Expression>,
    ws!(alt!(
        expr_string_literal => {|text: &str| Expression::StringLiteral(text.to_owned())}
    ))
);

named!(expr_string_literal<&str>,
    map_res!(
        delimited!(
            tag!(STRING_BOUNDARY),
            take_until!(STRING_BOUNDARY),
            tag!(STRING_BOUNDARY)
        ),
        |s: &'a [u8]| str::from_utf8(s)
    )
);
