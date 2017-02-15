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
const IF_KEYWORD: &'static str = "if";

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
    },
    IfCondition {
        condition: Expression,
        body: Vec<Statement>,
    },
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

impl_rdp! {
    grammar! {
        program = _{ statement* ~ eoi }

        statement = _{ comment | declaration | while_loop | (expr ~ ";") }

        comment = _{ line_comment | block_comment }
        line_comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
        block_comment = _{ ["/*"] ~ ((!(["*/"]) ~ any) | block_comment)* ~ ["*/"] }

        declaration = _{ ["let"] ~ identifier ~ [":"] ~ type ~ (["="] ~ expr)? ~ [";"] }

        type = _{ identifier | array_type }
        array_type = _{ ["["] ~ identifier ~ [";"] ~ array_size ~ ["]"] }
        array_size = _{ ["_"] | positive_integer }

        while_loop = _{ ["while"] ~ expr ~ block }

        expr = _{
            { conditional | string_literal | number | constant | range | block | group | func_call | method_calls }

            // Ordered from lowest precedence to highest precedence
            bool_or = {< ["||"] }
            bool_and = {< ["&&"] }
            // NOTE: Order matters! { ["<"] | ["<="] } will never match "<="
            comparison = { ["=="] | ["!="] | [">="] | ["<="] | [">"] | ["<"] }
            concatenation = {< ["++"] }
            term = { ["+"] | ["-"] }
            factor = { ["/"] | ["*"] | ["%"] }
            pow = { ["**"] }
        }

        conditional = _{ ["if"] ~ expr ~ block ~ (["else"] ~ conditional)? ~ (["else"] ~ block)? }

        // This allows {} and {expr; expr} and {expr; expr;} and {expr}
        block = _{ ["{"] ~ (expr ~ [";"])* ~ expr? ~ ["}"] }
        group = _{ ["("] ~ expr ~ [")"] }
        range = _{ number ~ ([","] ~ number)? ~ [".."] ~ number }

        func_call = _{ identifier ~ func_args }

        method_calls = _{ identifier ~ method_call* }
        method_call = _{ ["."] ~ identifier ~ func_args }

        // This allows () and (func_arg, func_arg) and (func_arg) and (func_arg,)
        func_args = _{ ["("] ~ (func_arg ~ [","])* ~ func_arg? ~ [")"] }
        func_arg = _{ expr }

        string_literal = _{ ["\""] ~ literal_char* ~ ["\""] }
        literal_char = _{ escape_sequence | any+ }
        escape_sequence = _{ ["\\\\"] | ["\\\""] | ["\\\'"] | ["\\n"] | ["\\r"] | "\\t" | ["\\0"] | ["\\f"] | ["\\v"] | ["\\e"] }

        identifier = @{ !keyword ~ (alpha | ["_"]) ~ (alphanumeric | ["_"])* }
        alpha = _{ ['a'..'z'] }
        alphanumeric = _{ alpha | ['0'..'9'] }

        number = @{ (["-"] | ["+"])? ~ positive_integer }
        positive_integer = _{ ["0"] | (nonzero ~ digit*) }
        // Allow "_" in numbers for grouping: 1_000_000 == 1000000
        digit = _{ ["0"] | nonzero | ["_"] }
        nonzero = _{ ['1'..'9'] }

        constant = @{ ["true"] | ["false"] }

        whitespace = { [" "] | ["\t"] | ["\u{000C}"] | ["\r"] | ["\n"] }
        // NOTE: When changing this code, make sure you don't have a subset of a word before
        // another word. For example: { ["type"] | ["typeof"] } will never match "typeof"
        keyword = @{
            ["abstract"] | ["as"] | ["become"] | ["break"] | ["byte"] | ["class"] | ["clear"] |
            ["const"] | ["continue"] | ["do"] | ["else"] | ["enum"] | ["eval"] | ["export"] |
            ["extern"] | ["false"] | ["final"] | ["fn"] | ["for"] | ["if"] | ["impl"] | ["import"] |
            ["in"] | ["let"] | ["loop"] | ["match"] | ["mod"] | ["move"] | ["mut"] | ["of"] |
            ["out"] | ["pub"] | ["raw"] | ["read"] | ["ref"] | ["return"] | ["self"] | ["static"] |
            ["struct"] | ["super"] | ["trait"] | ["true"] | ["typeof"] | ["type"] | ["unsafe"] |
            ["use"] | ["where"] | ["while"] | ["yield"]
        }
    }

    process! {
        program(&self) -> Program {
        }
    }
}
