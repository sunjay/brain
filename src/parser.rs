use std::str::{self, FromStr};

use nom::{is_alphabetic, is_digit, digit, Err};

const STRING_BOUNDARY: &'static str = "\"";
const STATEMENT_TERMINATOR: &'static str = ";";
const ASSIGNMENT_OPERATOR: &'static str = "=";
const LINE_COMMENT: &'static str = "//";

const START_BLOCK_COMMENT: &'static str = "/*";
const END_BLOCK_COMMENT: &'static str = "*/";
const BEGIN_SLICE: &'static str = "[";
const END_SLICE: &'static str = "]";
const BEGIN_BLOCK: &'static str = "{";
const END_BLOCK: &'static str = "}";

const OUTPUT_KEYWORD: &'static str = "out";
const INPUT_KEYWORD: &'static str = "in";
const WHILE_KEYWORD: &'static str = "while";

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
    type Err = Err<String>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_all_statements(input.as_bytes()).to_result().map(|statements| {
            Program(statements)
        }).map_err(|e| match e {
            Err::Code(kind) => {
                println!("Code({:?})", kind);
                unimplemented!();
            },
            Err::Node(kind, _) => {
                println!("Node({:?}, ..)", kind);
                unimplemented!();
            },
            Err::Position(kind, input) => {
                println!("Position({:?}, {:?})", kind, str::from_utf8(input));
                Err::Position(kind, str::from_utf8(input).unwrap().to_string())
            },
            Err::NodePosition(kind, input, _) => {
                println!("NodePosition({:?}, {:?}, ..)", kind, str::from_utf8(input));
                unimplemented!();
            }
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Comment(String),
    Output(Vec<Expression>),
    Input {
        name: String,
        slice: Option<Slice>,
    },
    Declaration {
        name: String,
        slice: Option<Slice>,
        expr: Expression,
    },
    WhileLoop {
        condition: WhileCondition,
        body: Vec<Statement>,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Slice {
    SingleValue(usize),
    //Range(Option<usize>, Option<usize>),
    Unspecified,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WhileCondition {
    Input {
        name: String,
        slice: Option<Slice>,
    },
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    StringLiteral(String),
    Identifier(String),
}

named!(parse_all_statements< Vec<Statement> >, complete!(do_parse!(
    statements: many0!(statement) >>
    // Ensures that we reach EOF after all the statements
    eof!() >>
    (statements)
)));

named!(statement<Statement>, ws!(alt!(
    comment => {|content: &str| Statement::Comment(content.to_owned())} |
    outputs => {|exprs: Vec<Expression>| Statement::Output(exprs)} |
    input => {|(name, slice): (String, Option<Slice>)| {
        Statement::Input {name: name, slice: slice}
    }} |
    declaration => {|(name, slice, expr): (String, Option<Slice>, Expression)| {
        Statement::Declaration {name: name, slice: slice, expr: expr}
    }} |
    while_loop => {|(cond, body): (WhileCondition, Vec<Statement>)| {
        Statement::WhileLoop {condition: cond, body: body}
    }}
)));

named!(comment<&str>, alt!(line_comment | block_comment));

named!(line_comment<&str>,
    map_res!(
        do_parse!(
            tag!(LINE_COMMENT) >>
            content: take_until_and_consume!("\n") >>
            (content)
        ),
        |s: &'a [u8]| str::from_utf8(s)
    )
);

named!(block_comment<&str>,
    map_res!(
        delimited!(
            tag!(START_BLOCK_COMMENT),
            take_until!(END_BLOCK_COMMENT),
            tag!(END_BLOCK_COMMENT)
        ),
        |s: &'a [u8]| str::from_utf8(s)
    )
);

named!(outputs< Vec<Expression> >,
    ws!(do_parse!(
        tag!(OUTPUT_KEYWORD) >>
        expr: many1!(expression) >>
        tag!(STATEMENT_TERMINATOR) >>
        (expr)
    ))
);

named!(input<(String, Option<Slice>)>,
    ws!(do_parse!(
        tag!(INPUT_KEYWORD) >>
        declaration: type_declaration >>
        tag!(STATEMENT_TERMINATOR) >>
        (declaration.0, declaration.1)
    ))
);

named!(declaration<(String, Option<Slice>, Expression)>,
    ws!(do_parse!(
        declaration: type_declaration >>
        tag!(ASSIGNMENT_OPERATOR) >>
        expr: expression >>
        tag!(STATEMENT_TERMINATOR) >>
        (declaration.0, declaration.1, expr)
    ))
);

named!(while_loop<(WhileCondition, Vec<Statement>)>,
    ws!(do_parse!(
        tag!(WHILE_KEYWORD) >>
        cond: while_condition >>
        statements: block_statements >>
        (cond, statements)
    ))
);

named!(while_condition<WhileCondition>,
    alt_complete!(
        map!(ws!(do_parse!(
            tag!(INPUT_KEYWORD) >>
            declaration: type_declaration >>
            (declaration.0, declaration.1)
        )), |(name, slice): (String, Option<Slice>)| WhileCondition::Input {name: name, slice: slice}) |
        map!(expression, |expr: Expression| WhileCondition::Expression(expr))
    )
);

named!(block_statements< Vec<Statement> >,
    ws!(do_parse!(
        tag!(BEGIN_BLOCK) >>
        statements: many0!(statement) >>
        tag!(END_BLOCK) >>
        (statements)
    ))
);

named!(type_declaration<(String, Option<Slice>)>,
    do_parse!(
        declaration: identifier_slice >>
        (declaration.0, declaration.1)
    )
);

named!(identifier_slice<(String, Option<Slice>)>,
    ws!(do_parse!(
        name: identifier >>
        slice: opt!(slice_variants) >>
        (name, slice)
    ))
);

named!(slice_variants<Slice>,
    alt_complete!(
        do_parse!(
            tag!(BEGIN_SLICE) >>
            tag!(END_SLICE) >>
            (Slice::Unspecified)
        ) |
        delimited!(
            tag!(BEGIN_SLICE),
            slice_single_value,
            tag!(END_SLICE)
        )
    )
);

named!(slice_single_value<Slice>,
    map!(ws!(index_value), |value: usize| Slice::SingleValue(value))
);

named!(index_value<usize>,
    map_res!(digit_s, |n: &str| n.parse())
);

named!(digit_s<&str>,
    map_res!(digit, |n: &'a [u8]| str::from_utf8(n))
);

named!(expression<Expression>,
    ws!(alt!(
        expr_string_literal => {|text: String| Expression::StringLiteral(text)} |
        identifier => {|ident: String| Expression::Identifier(ident)}
    ))
);

named!(identifier<String>,
    map_res!(
        do_parse!(
            // must start with a non-digit
            start: take_while!(is_identifier_start) >>
            rest: take_while!(is_identifier_char) >>
            (start, rest)
        ),
        |(start, rest): (&[u8], &[u8])| {
            str::from_utf8(start).and_then(|start| {
                str::from_utf8(rest).map(|rest| {
                    format!("{}{}", start, rest)
                })
            })
        }
    )
);

named!(expr_string_literal<String>,
    map_res!(
        delimited!(
            tag!(STRING_BOUNDARY),
            string_text,
            tag!(STRING_BOUNDARY)
        ),
        |s: Vec<u8>| String::from_utf8(s)
    )
);

named!(string_text<Vec<u8>>,
    fold_many0!(
        unescaped_string_text,
        Vec::new(),
        |mut acc: Vec<u8>, bytes: &[u8]| {
            acc.extend(bytes);
            acc
        }
    )
);

named!(unescaped_string_text<&[u8]>,
    alt!(
        // We need to take until \ so that the unescaping can work
        // We also need to take until " so that we don't go past the string boundary
        take_until_either!("\\\"") |
        tag!("\\\\") => {|_| &b"\\"[..]} |
        tag!("\\\"") => {|_| &b"\""[..]} |
        tag!("\\\'") => {|_| &b"\'"[..]} |
        tag!("\\n") => {|_| &b"\n"[..]} |
        tag!("\\r") => {|_| &b"\r"[..]} |
        tag!("\\t") => {|_| &b"\t"[..]} |
        tag!("\\0") => {|_| &b"\0"[..]}
    )
);

fn is_identifier_start(c: u8) -> bool {
    is_alphabetic(c) || c == '_' as u8
}

fn is_identifier_char(c: u8) -> bool {
    is_identifier_start(c) || is_digit(c)
}
