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
        array_size = _{ unspecified | expr }
        unspecified = { ["_"] }

        while_loop = { ["while"] ~ expr ~ block }
        for_loop = { ["for"] ~ pattern ~ ["in"] ~ expr ~ block }

        expr = _{
            { constant | field_access | func_call | group | block | conditional | bool_not | string_literal | number | negation }

            // Ordered from lowest precedence to highest precedence
            bool_or = {< ["||"] }
            bool_and = {< ["&&"] }
            // NOTE: Order matters! { ["<"] | ["<="] } will never match "<="
            comparison = { ["=="] | ["!="] | [">="] | ["<="] | [">"] | ["<"] }
            concatenation = {< ["++"] }
            range = { [".."] }
            term = { ["+"] | ["-"] }
            factor = { ["/"] | ["*"] | ["%"] }
            pow = { ["**"] }
        }

        conditional = { ["if"] ~ expr ~ block ~ (["else"] ~ conditional)? ~ (["else"] ~ block)? }

        // This allows {} and {statement; statement; statement;} and {statement; expr} and {expr}
        block = { ["{"] ~ statement* ~ expr? ~ ["}"] }

        group = { ["("] ~ expr ~ [")"] }

        bool_not = { ["!"] ~ expr }
        negation = { ["-"] ~ expr }

        func_call = { (group | identifier) ~ func_args }
        field_access = { (identifier | group | block) ~ field_access_rest }
        field_access_rest = _{ op_access ~ identifier ~ func_args? ~ field_access_rest* }
        op_access = { ["."] }

        // This allows () and (func_arg, func_arg) and (func_arg) and (func_arg,)
        func_args = { ["("] ~ (func_arg ~ [","])* ~ func_arg? ~ [")"] }
        func_arg = _{ expr }

        string_literal = @{ ["\""] ~ literal_char* ~ ["\""] }
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
            (_: array_type, type_def: _type_def(), _: semi, size: _optional_expr()) => {
                TypeDefinition::Array {type_def: Box::new(type_def), size: size}
            },
        }

        _optional_expr(&self) -> Option<Expression> {
            (e: _expr()) => Some(e),
            () => None,
        }

        _expr(&self) -> Expression {
            (_: func_call, method: _expr(), args: _func_args()) => {
                Expression::Call {method: Box::new(method), args: args}
            },
            (_: field_access, target: _expr(), field: _field_access_rest()) => {
                let mut exprs = vec![target];
                exprs.extend(field);

                println!("{:?}", exprs);
                unimplemented!();

                //Expression::Access {target: Box::new(target), field: Box::new(field)}
            },
            (&ident: identifier) => {
                Expression::Identifier(ident.into())
            },
            (&s: string_literal) => {
                Expression::StringLiteral(s.into())
            },
            (&s: number) => {
                // If our grammar is correct, we are guarenteed that this will work
                Expression::Number(s.parse().unwrap())
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

        _field_access_rest(&self) -> VecDeque<Expression> {
            (_: op_access, ident: _identifier(), args: _func_args(), mut rest: _field_access_rest()) => {
                rest.push_front(Expression::Call {
                    method: Box::new(Expression::Identifier(ident)),
                    args: args,
                });

                rest
            },
            (_: op_access, ident: _identifier(), mut rest: _field_access_rest()) => {
                rest.push_front(Expression::Identifier(ident));

                rest
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
            (head: _expr(), mut tail: _block_deque()) => {
                tail.push_front(Statement::Expression {expr: head});

                tail
            },
            () => {
                VecDeque::new()
            },
        }

        _identifier(&self) -> Identifier {
            (&ident: identifier) => {
                ident.into()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pest::prelude::*;

    #[test]
    fn string_literal() {
        test_parse(r#""foo""#, |p| p.string_literal(), vec![
            Token::new(Rule::string_literal, 0, 5),
        ]);
    }

    #[test]
    fn number() {
        test_parse(r#"0"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 1),
        ]);

        test_parse(r#"100"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 3),
        ]);

        test_parse(r#"-100"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 4),
        ]);

        test_parse(r#"+100"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 4),
        ]);

        test_parse(r#"1_000_000"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 9),
        ]);
    }

    fn test_parse<F>(input: &'static str, parse: F, tokens: Vec<Token<Rule>>)
        where F: FnOnce(&mut Rdp<StringInput>) -> bool {

        let mut parser = parser_from(input);
        assert!(parse(&mut parser), "Parsing failed");
        assert!(parser.end(), "Parser did not reach eoi");

        assert_eq!(parser.queue(), &tokens);
    }

    fn parser_from(s: &'static str) -> Rdp<StringInput> {
        Rdp::new(StringInput::new(s))
    }
}
