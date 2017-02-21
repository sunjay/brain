use std::collections::VecDeque;

use pest::prelude::*;

use super::*;

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

        type_def = _{ identifier | array_type }
        array_type = { ["["] ~ type_def ~ semi ~ array_size ~ ["]"] }
        array_size = _{ unspecified | number }
        unspecified = { ["_"] }

        while_loop = { ["while"] ~ expr ~ block }
        for_loop = { ["for"] ~ pattern ~ ["in"] ~ expr ~ block }

        expr = _{
            { block | group | constant | func_call | conditional | bool_not | string_literal | range | number }

            // Ordered from lowest precedence to highest precedence
            bool_or = {< ["||"] }
            bool_and = {< ["&&"] }
            // NOTE: Order matters! { ["<"] | ["<="] } will never match "<="
            comparison = { ["=="] | ["!="] | [">="] | ["<="] | [">"] | ["<"] }
            concatenation = {< ["++"] }
            term = { ["+"] | ["-"] }
            factor = { ["/"] | ["*"] | ["%"] }
            pow = { ["**"] }

            field_access = {< ["."] }
        }

        conditional = { ["if"] ~ expr ~ block ~ (["else"] ~ conditional)? ~ (["else"] ~ block)? }

        // This allows {} and {statement; statement; statement;} and {statement; expr} and {expr}
        block = { ["{"] ~ statement* ~ expr? ~ ["}"] }

        group = { ["("] ~ expr ~ [")"] }
        bool_not = { ["!"] ~ expr }
        range = { number ~ ([","] ~ number)? ~ [".."] ~ number }

        func_call = { expr ~ func_args }

        // This allows () and (func_arg, func_arg) and (func_arg) and (func_arg,)
        func_args = _{ ["("] ~ (func_arg ~ [","])* ~ func_arg? ~ [")"] }
        func_arg = _{ expr }

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
        semi = { [";"] }
    }

    process! {
        parse_program(&self) -> Program {
            (list: _program()) => {
                Program::new(list.into_iter().collect())
            },
        }

        _program(&self) -> VecDeque<Statement> {
            (head: _statement(), mut tail: _program()) => {
                tail.push_front(head);

                tail
            },
            () => {
                VecDeque::new()
            },
        }

        _statement(&self) -> Statement {
            (&text: comment) => {
                Statement::Comment(text.into())
            },
            (_: declaration, pattern: _pattern(), type_def: _type_def(), expr: _optional_expr(), _: semi) => {
                Statement::Declaration {pattern: pattern, type_def: type_def, expr: expr}
            },
            (_: assignment, ident: _identifier(), expr: _expr(), _: semi) => {
                Statement::Assignment {lhs: ident, expr: expr}
            },
            (_: while_loop, condition: _expr(), body: _block()) => {
                Statement::WhileLoop {condition: condition, body: body}
            },
            // This should always be last as it will catch pretty much any cases that weren't caught above
            (expr: _expr(), _: semi) => {
                Statement::Expression {expr: expr}
            },
        }

        _pattern(&self) -> Pattern {
            (_: pattern, ident: _identifier()) => {
                Pattern::Identifier(ident)
            },
        }

        _type_def(&self) -> TypeDefinition {
            (ident: _identifier()) => {
                TypeDefinition::Name {name: ident}
            },
            (_: array_type, type_def: _type_def(), _: semi, size: _array_size()) => {
                TypeDefinition::Array {type_def: Box::new(type_def), size: size}
            },
        }

        _array_size(&self) -> Option<Number> {
            (_: unspecified) => {
                None
            },
            (n: _number()) => {
                Some(n)
            }
        }

        _optional_expr(&self) -> Option<Expression> {
            (e: _expr()) => Some(e),
            () => None,
        }

        _expr(&self) -> Expression {
            (_: func_call, method: _expr(), args: _func_args()) => {
                Expression::Call {method: Box::new(method), args: args}
            },
            (_: field_access, target: _expr(), field: _expr()) => {
                Expression::Access {target: Box::new(target), field: Box::new(field)}
            },
            (&s: string_literal) => {
                Expression::StringLiteral(s.into())
            },
            (n: _number()) => {
                Expression::Number(n)
            },
        }

        _func_args(&self) -> FuncArgs {
            (deque: _expr_deque()) => {
                deque.into_iter().collect()
            },
        }

        _expr_deque(&self) -> VecDeque<Expression> {
            (head: _expr(), mut tail: _expr_deque()) => {
                tail.push_front(head);

                tail
            },
            () => {
                VecDeque::new()
            },
        }

        _block(&self) -> Block {
            (_: block, deque: _block_deque()) => {
                deque.into_iter().collect()
            },
        }

        _block_deque(&self) -> VecDeque<Statement> {
            (head: _statement(), mut tail: _block_deque()) => {
                tail.push_front(head);

                tail
            },
            () => {
                VecDeque::new()
            },
        }

        _identifier(&self) -> Identifier {
            (&ident: identifier) => {
                ident.into()
            }
        }

        _number(&self) -> Number {
            (&s: number) => {
                // If our grammar is correct, we are guarenteed that this will work
                s.parse().unwrap()
            }
        }
    }
}
