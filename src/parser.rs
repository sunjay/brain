use std::str::{self, FromStr};

use pest::prelude::*;

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
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parser = Rdp::new(StringInput::new(input));

        println!("{:?}", (parser.program(), parser.end()));
        println!("{:#?}", parser.queue());
        println!("{:?}", parser.stack());
        let (expected, pos) = parser.expected();
        let (line, col) = parser.input().line_col(pos);
        println!("Expected: {:?} at line {} col {}", expected, line, col);
        unimplemented!();
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

        statement = _{ declaration | assignment | while_loop | for_loop | conditional | (expr ~ semi) | comment }

        comment = @{ block_comment | line_comment }
        line_comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
        block_comment = _{ ["/*"] ~ ((!(["*/"]) ~ any) | block_comment)* ~ ["*/"] }

        assignment = { identifier ~ ["="] ~ expr ~ semi}
        declaration = { ["let"] ~ pattern ~ [":"] ~ type_def ~ (["="] ~ expr)? ~ semi}
        pattern = { identifier }

        type_def = { identifier | array_type }
        array_type = { ["["] ~ identifier ~ semi ~ array_size ~ ["]"] }
        array_size = { ["_"] | number }

        while_loop = { ["while"] ~ expr ~ block }
        for_loop = { ["for"] ~ pattern ~ ["in"] ~ expr ~ block }

        expr = {
            { block | group | constant | func_call | call_chain | conditional | string_literal | range | number }

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

        conditional = { ["if"] ~ expr ~ block ~ (["else"] ~ conditional)? ~ (["else"] ~ block)? }

        // This allows {} and {statement; statement; statement;} and {statement; expr} and {expr}
        block = { ["{"] ~ statement* ~ expr? ~ ["}"] }
        group = { ["("] ~ expr ~ [")"] }
        range = { number ~ (comma ~ number)? ~ [".."] ~ number }

        func_call = { identifier ~ func_args }

        call_chain = { identifier ~ prop_get_call* }
        prop_get_call = { ["."] ~ identifier ~ func_args? }

        // This allows () and (func_arg, func_arg) and (func_arg) and (func_arg,)
        func_args = { ["("] ~ (func_arg ~ comma)* ~ func_arg? ~ [")"] }
        func_arg = { expr }

        string_literal = { ["\""] ~ literal_char* ~ ["\""] }
        literal_char = _{ escape_sequence | (!["\""] ~ any) }
        escape_sequence = _{ ["\\\\"] | ["\\\""] | ["\\\'"] | ["\\n"] | ["\\r"] | ["\\t"] | ["\\0"] | ["\\f"] | ["\\v"] | ["\\e"] }

        identifier = @{ !keyword ~ (alpha | ["_"]) ~ (alphanumeric | ["_"])* }
        alpha = _{ ['a'..'z'] | ['A'..'Z'] }
        alphanumeric = _{ alpha | ['0'..'9'] }

        number = @{ (["-"] | ["+"])? ~ positive_integer }
        positive_integer = _{ ["0"] | (nonzero ~ digit*) }
        // Allow "_" in numbers for grouping: 1_000_000 == 1000000
        digit = _{ ["0"] | nonzero | ["_"] }
        nonzero = _{ ['1'..'9'] }

        constant = @{ ["true"] | ["false"] }

        whitespace = _{ [" "] | ["\t"] | ["\u{000C}"] | ["\r"] | ["\n"] }
        // NOTE: When changing this code, make sure you don't have a subset of a word before
        // another word. For example: { ["type"] | ["typeof"] } will never match "typeof"
        keyword = @{
            ["abstract"] | ["as"] | ["become"] | ["break"] | ["byte"] | ["class"] | ["clear"] |
            ["const"] | ["continue"] | ["do"] | ["else"] | ["enum"] | ["eval"] | ["export"] |
            ["extern"] | ["false"] | ["final"] | ["fn"] | ["for"] | ["if"] | ["impl"] | ["import"] |
            ["in"] | ["let"] | ["loop"] | ["match"] | ["mod"] | ["move"] | ["mut"] | ["of"] |
            ["out"] | ["pub"] | ["raw"] | ["ref"] | ["return"] | ["self"] | ["static"] |
            ["struct"] | ["super"] | ["trait"] | ["true"] | ["typeof"] | ["type"] | ["unsafe"] |
            ["use"] | ["where"] | ["while"] | ["yield"]
        }

        // These are separate rules because we can use the generated rules and tokens to provide
        // better error messages
        comma = { [","] }
        semi = { [";"] }
    }
}
