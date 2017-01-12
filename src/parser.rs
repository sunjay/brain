use std::str;
use nom::{IResult};

const COMMENT_START: char = '#';
const STRING_BOUNDARY: char = '"';
const ASSIGNMENT_OPERATOR: char = '=';

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Comment(String),
    Assignment(Option<String>, Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Text(String),
}

named!(pub parse< &str, Vec<Statement> >, many0!(statement));

named!(statement<&str, Statement>, ws!(alt!(
    comment => {|content: &str| Statement::Comment(content.to_owned())} |
    assignment => {|(name, expr): (Option<&str>, Expression)| Statement::Assignment(name.map(|s| s.to_owned()), expr)}
)));

named!(comment<&str, &str>,
    do_parse!(
        char!(COMMENT_START) >>
        content: take_until_and_consume_s!("\n") >>
        (str::from_utf8(content).unwrap())
    )
);

named!(assignment<&str, (Option<&str>, Expression)>,
    do_parse!(
        name: assignment_name >>
        char!(ASSIGNMENT_OPERATOR) >>
        expr: expression >>
        (name, expr)
    )
);

named!(assignment_name< &str, Option<&str> >,
    opt!(
        take_while1_s!(is_valid_assignment_name_char)
    )
);

named!(expression<&str, Expression>,
    alt!(
        expr_text => {|text| Expression::Text(text.to_owned())}
    )
);

named!(expr_text<&str, &str>,
    delimited!(
        char!(STRING_BOUNDARY),
        take_while_s!(is_string_boundary),
        char!(STRING_BOUNDARY)
    )
);

fn is_valid_assignment_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_string_boundary(c: char) -> bool {
    c == STRING_BOUNDARY
}
