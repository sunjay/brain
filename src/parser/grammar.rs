use std::fmt;
use std::collections::VecDeque;

use pest::prelude::*;

use super::*;

impl_rdp! {
    grammar! {
        program = _{ soi ~ statement* ~ eoi }

        // conditional is technically an expression too but it can be used as a statement
        // without a semicolon as well
        statement = { declaration | assignment | while_loop | conditional | (expr ~ semi) | comment }

        comment = @{ block_comment | line_comment }
        line_comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
        block_comment = _{ ["/*"] ~ ((!(["*/"]) ~ any) | block_comment)* ~ ["*/"] }

        assignment = { identifier ~ op_assign ~ expr ~ semi}
        declaration = { ["let"] ~ pattern ~ op_declare_type ~ type_def ~ (op_assign ~ expr)? ~ semi}
        op_declare_type = { [":"] }
        op_assign = { ["="] }
        pattern = { identifier }

        type_def = { identifier | array_type }
        array_type = { ["["] ~ type_def ~ semi ~ array_size ~ ["]"] }
        array_size = _{ unspecified | expr }
        unspecified = { ["_"] }

        while_loop = { ["while"] ~ expr ~ block }

        expr = {
            { func_call | field_access | identifier | conditional | string_literal | number }

            // Ordered from lowest precedence to highest precedence
            bool_or = { op_bool_or }
            bool_and = { op_bool_and }
            // NOTE: Order matters! { ["<"] | ["<="] } will never match "<="
            comparison = { op_eq | op_ne | op_ge | op_le | op_gt | op_lt }
        }
        op_bool_or = { ["||"] }
        op_bool_and = { ["&&"] }
        op_eq = { ["=="] }
        op_ne = { ["!="] }
        op_ge = { [">="] }
        op_le = { ["<="] }
        op_gt = { [">"] }
        op_lt = { ["<"] }

        conditional = { ["if"] ~ expr ~ block ~ (op_else_if ~ expr ~ block)* ~ (op_else ~ block)? }
        op_else_if = { ["else if"] }
        op_else = { ["else"] }

        // This allows {} and {statement; statement; statement;} and {statement; expr} and {expr}
        block = _{ block_start ~ statement* ~ expr? ~ block_end }
        block_start = { ["{"] }
        block_end = { ["}"] }

        func_call = { identifier ~ func_args }
        field_access = { identifier ~ op_access ~ identifier ~ func_args? }
        op_access = { ["."] }

        // This allows () and (func_arg, func_arg) and (func_arg) and (func_arg,)
        func_args = _{ func_args_start ~ (func_arg ~ [","])* ~ func_arg? ~ func_args_end }
        func_args_start = { ["("] }
        func_args_end = { [")"] }
        func_arg = _{ expr }

        string_literal = @{ ["\""] ~ literal_char* ~ ["\""] }
        literal_char = { escape_sequence | (!["\""] ~ any) }
        escape_sequence = _{ ["\\\\"] | ["\\\""] | ["\\\'"] | ["\\n"] | ["\\r"] | ["\\t"] | ["\\0"] }

        // Using a hack here to get both the token and the value out of pest
        identifier = { identifier_ }
        identifier_ = @{ !keyword ~ (alpha | ["_"]) ~ (alphanumeric | ["_"])* }
        alpha = _{ ['a'..'z'] | ['A'..'Z'] }
        alphanumeric = _{ alpha | ['0'..'9'] }

        // Using a hack here to get both the token and the value out of pest
        number = { number_ }
        number_ = @{ ["0"] | (nonzero ~ digit*) }
        // Allow "_" in numbers for grouping: 1_000_000 == 1000000
        digit = _{ ["0"] | nonzero | ["_"] }
        nonzero = _{ ['1'..'9'] }

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
        // Top-level method that returns the abstract syntax tree based on the
        // contents of the parser queue
        // Make sure to call program() before this so there is something in the queue
        ast(&self) -> Program {
            (list: _program()) => {
                Program::from(list.into_iter().collect::<Vec<_>>())
            },
        }

        _program(&self) -> VecDeque<Statement> {
            (_: statement, head: _statement(), mut tail: _program()) => {
                tail.push_front(head);

                tail
            },
            (&text: comment, mut tail: _program()) => {
                tail.push_front(Statement::Comment(text.into()));

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
            (token: declaration, pattern: _pattern(), _: op_declare_type, type_def: _type_def(), _: op_assign, _: expr, expr: _expr(), _: semi) => {
                Statement::Declaration {pattern: pattern, type_def: type_def, expr: Some(expr), span: Span::from(token)}
            },
            (token: declaration, pattern: _pattern(), _: op_declare_type, type_def: _type_def(), _: semi) => {
                Statement::Declaration {pattern: pattern, type_def: type_def, expr: None, span: Span::from(token)}
            },
            (token: assignment, ident: _identifier(), _: op_assign, _: expr, expr: _expr(), _: semi) => {
                Statement::Assignment {lhs: ident, expr: expr, span: Span::from(token)}
            },
            (token: while_loop, _: expr, condition: _expr(), body: _block()) => {
                Statement::WhileLoop {condition: condition, body: body, span: Span::from(token)}
            },
            (token: conditional, expr: _conditional()) => {
                Statement::Expression {expr: expr, span: Span::from(token)}
            },
            // This should always be last as it will catch pretty much any cases that weren't caught above
            (token: expr, expr: _expr(), _: semi) => {
                Statement::Expression {expr: expr, span: Span::from(token)}
            },
        }

        _pattern(&self) -> Pattern {
            (token: pattern, ident: _identifier()) => {
                Pattern::Identifier(ident, Span::from(token))
            },
        }

        _type_def(&self) -> TypeDefinition {
            (_: type_def, token: array_type, type_def: _type_def(), _: semi, _: unspecified) => {
                TypeDefinition::Array {type_def: Box::new(type_def), size: None, span: Span::from(token)}
            },
            (_: type_def, token: array_type, type_def: _type_def(), _: semi, _: expr, size: _expr()) => {
                TypeDefinition::Array {type_def: Box::new(type_def), size: Some(size), span: Span::from(token)}
            },
            (token: type_def, ident: _identifier()) => {
                TypeDefinition::Name {name: ident, span: Span::from(token)}
            },
        }

        _expr(&self) -> Expression {
            (token: func_call, method: _identifier(), args: _func_args()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(method, Span::from(token))),
                    args: args,
                    span: Span::from(token),
                }
            },
            (_: field_access, expr: _field_access()) => {
                expr
            },
            (_: conditional, expr: _conditional()) => {
                expr
            },
            (token: bool_or, lhs: _expr(), _: op_bool_or, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span::from(token))),
                    args: vec![lhs, rhs],
                    span: Span::from(token),
                }
            },
            (token: bool_and, lhs: _expr(), _: op_bool_and, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span::from(token))),
                    args: vec![lhs, rhs],
                    span: Span::from(token),
                }
            },
            (token: comparison, lhs: _expr(), op_token, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from(match op_token.rule {
                        Rule::op_eq => "std::cmp::PartialEq::eq",
                        Rule::op_ne => "std::cmp::PartialEq::ne",
                        Rule::op_ge => "std::cmp::PartialOrd::ge",
                        Rule::op_le => "std::cmp::PartialOrd::le",
                        Rule::op_gt => "std::cmp::PartialOrd::gt",
                        Rule::op_lt => "std::cmp::PartialOrd::lt",
                        _ => unreachable!(),
                    }), Span::from(token))),
                    args: vec![lhs, rhs],
                    span: Span::from(token),
                }
            },
            (token: identifier, &ident: identifier_) => {
                Expression::Identifier(ident.into(), Span::from(token))
            },
            (token: string_literal, s: _literal_chars()) => {
                Expression::StringLiteral(s.into_iter().collect(), Span::from(token))
            },
            (token: number, &s: number_) => {
                // If our grammar is correct, we are guarenteed that this will work
                Expression::Number(s.replace("_", "").parse().unwrap(), Span::from(token))
            },
        }

        _field_access(&self) -> Expression {
            (target: _identifier(), token: op_access, field: _identifier(), args: _func_args()) => {
                Expression::Call {
                    method: Box::new(Expression::Access {
                        target: Box::new(Expression::Identifier(target, Span::from(token))),
                        field: Box::new(Expression::Identifier(field, Span::from(token))),
                        span: Span::from(token),
                    }),
                    args: args,
                    span: Span::from(token),
                }
            },
            (target: _identifier(), token: op_access, field: _identifier()) => {
                Expression::Access {
                    target: Box::new(Expression::Identifier(target, Span::from(token))),
                    field: Box::new(Expression::Identifier(field, Span::from(token))),
                    span: Span::from(token),
                }
            },
        }

        _conditional(&self) -> Expression {
            (token: expr, expr: _expr(), block: _block(), _: op_else_if, branches: _branches(), _: op_else, else_block: _block()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: Some(nest_else_ifs(branches, Some(else_block))),
                    span: Span::from(token),
                }
            },
            (token: expr, expr: _expr(), block: _block(), _: op_else_if, branches: _branches()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: Some(nest_else_ifs(branches, None)),
                    span: Span::from(token),
                }
            },
            (token: expr, expr: _expr(), block: _block(), _: op_else, else_block: _block()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: Some(else_block),
                    span: Span::from(token),
                }
            },
            (token: expr, expr: _expr(), block: _block()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: None,
                    span: Span::from(token),
                }
            },
        }

        _branches(&self) -> VecDeque<Expression> {
            (token: expr, expr: _expr(), block: _block(), _: op_else_if, mut tail: _branches()) => {
                tail.push_front(Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: None,
                    span: Span::from(token),
                });

                tail
            },
            (token: expr, expr: _expr(), block: _block()) => {
                let mut queue = VecDeque::new();
                queue.push_front(Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: None,
                    span: Span::from(token),
                });

                queue
            },
        }

        _func_args(&self) -> FuncArgs {
            (_: func_args_start, deque: _expr_deque()) => {
                deque.into_iter().collect()
            },
        }

        _expr_deque(&self) -> VecDeque<Expression> {
            (_: func_args_end) => {
                VecDeque::new()
            },
            (_: expr, head: _expr(), mut tail: _expr_deque()) => {
                tail.push_front(head);

                tail
            },
        }

        _block(&self) -> Block {
            (_: block_start, deque: _block_deque()) => {
                deque.into_iter().collect()
            },
        }

        _block_deque(&self) -> VecDeque<Statement> {
            (_: statement, head: _statement(), mut tail: _block_deque()) => {
                tail.push_front(head);

                tail
            },
            (token: expr, head: _expr(), mut tail: _block_deque()) => {
                tail.push_front(Statement::Expression {expr: head, span: Span::from(token)});

                tail
            },
            (&text: comment, mut tail: _block_deque()) => {
                tail.push_front(Statement::Comment(text.into()));

                tail
            },
            (_: block_end) => {
                VecDeque::new()
            },
        }

        _literal_chars(&self) -> VecDeque<char> {
            (&c: literal_char, mut tail: _literal_chars()) => {
                if c.len() == 2 {
                    debug_assert!(c.chars().next().unwrap() == '\\');
                    tail.push_front(match c.chars().nth(1).unwrap() {
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '0' => '\0',
                        //TODO: Replace this with a proper result when upgrading to pest 1.0
                        _ => panic!("Unknown escape: {}", c)
                    });
                }
                else {
                    debug_assert!(c.len() == 1);
                    tail.push_front(c.chars().next().unwrap());
                }

                tail
            },
            () => {
                VecDeque::new()
            }
        }

        _identifier(&self) -> Identifier {
            (_: identifier, &ident: identifier_) => {
                ident.into()
            },
        }
    }
}

/// Given a series of branch expressions, this will nest them together
/// so that they result in a single nested branch expression
///
/// # Example
/// Given:
/// if foo1 { body1 } else {}
/// if foo2 { body2 } else {}
/// if foo3 { body3 } else {}
///
/// Results in:
/// if foo1 { body1 } else { if foo2 { body2 } else { if foo3 { body3 } else {} } }
fn nest_else_ifs(branches: VecDeque<Expression>, else_block: Option<Block>) -> Block {
    branches.into_iter().rev().fold(else_block, |acc, mut br| {
        let span = match br {
            Expression::Branch {ref mut otherwise, span, ..} => {
                *otherwise = acc;

                span.clone()
            },
            _ => unreachable!(),
        };

        Some(vec![Statement::Expression {
            expr: {
                br
            },
            span: span,
        }])
    }).unwrap()
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just to make things a bit more ergonomic
        use Rule::*;

        write!(f, "{}", match *self {
            eoi => "EOF",
            comment => "comment",
            identifier => "identifier",
            keyword => "keyword",
            number => "number",
            string_literal => "string literal",
            literal_char => "character",
            any => "any character",

            unspecified => "`_`",
            semi => "`;`",

            bool_or => "`or`",
            bool_and => "`and`",
            op_else_if => "`else if`",
            op_else => "`else`",

            op_assign => "`=`",
            op_bool_or => "`||`",
            op_bool_and => "`&&`",
            op_eq => "`==`",
            op_ne => "`!=`",
            op_ge => "`>=`",
            op_le => "`<=`",
            op_gt => "`>`",
            op_lt => "`<`",
            op_access => "`.`",
            op_declare_type => "`:`",

            block_start => "`{`",
            block_end => "`}`",

            func_args_start => "`(`",
            func_args_end => "`)`",

            // There are many rules that will never get matched here because
            // this method is meant to be used for formatting errors
            // We don't want to use the "_" wildcard because we want Rust
            // to tell us when a new rule has to be added here
            statement | assignment | declaration | pattern | type_def | array_type | while_loop | comparison |
            conditional | func_call | field_access | expr | soi | identifier_ | number_ => unreachable!(*self),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use pest::prelude::*;

    use super::*;

    #[test]
    fn string_literal() {
        test_parse(r#""""#, |p| p.string_literal(), vec![
            Token::new(Rule::string_literal, 0, 2),
        ]);

        test_parse(r#""foo""#, |p| p.string_literal(), vec![
            Token::new(Rule::string_literal, 0, 5),
            Token::new(Rule::literal_char, 1, 2),
            Token::new(Rule::literal_char, 2, 3),
            Token::new(Rule::literal_char, 3, 4),
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

        test_parse(r#"1_000_000"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 9),
        ]);

        test_parse(r#"1_000_000_"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 10),
        ]);

        test_parse(r#"1____0_0__0______000____"#, |p| p.number(), vec![
            Token::new(Rule::number, 0, 24),
        ]);

        test_fail(r#"_1_000_000"#, |p| p.number());
    }

    #[test]
    fn field_access() {
        test_parse(r#"foo.bar"#, |p| p.field_access(), vec![
            Token::new(Rule::field_access, 0, 7),
            Token::new(Rule::identifier, 0, 3),
            Token::new(Rule::op_access, 3, 4),
            Token::new(Rule::identifier, 4, 7),
        ]);
    }

    #[test]
    fn numeric_literal() {
        test_method(r#"0"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(0, Span {start: 0, end: 0})
        );

        test_method(r#"100"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(100, Span {start: 0, end: 0})
        );

        test_method(r#"1_000_000"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(1_000_000, Span {start: 0, end: 0})
        );

        test_method(r#"1_000_000_"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(1_000_000_, Span {start: 0, end: 0})
        );

        test_method(r#"1____0_0__0______000____"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(1____0_0__0______000____, Span {start: 0, end: 0})
        );
    }

    #[test]
    fn string_literal_escapes() {
        test_method(r#""foo""#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::StringLiteral("foo".to_owned(), Span {start: 0, end: 0}));

        test_method(r#""\\ \" \' \n \r \t \0""#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::StringLiteral("\\ \" \' \n \r \t \0".to_owned(), Span {start: 0, end: 0}));
    }

    #[test]
    fn functions_field_access() {
        test_method(r#"func(1, "foo", 3)"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Call {
                method: Box::new(Expression::Identifier(Identifier::from("func"), Span {start: 0, end: 0})),
                args: vec![
                    Expression::Number(1, Span {start: 0, end: 0}),
                    Expression::StringLiteral("foo".to_owned(), Span {start: 0, end: 0}),
                    Expression::Number(3, Span {start: 0, end: 0}),
                ],
                span: Span {start: 0, end: 0},
            }
        );

        test_method(r#"thing.prop(1, "foo", 3)"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Call {
                method: Box::new(Expression::Access {
                    target: Box::new(Expression::Identifier(Identifier::from("thing"), Span {start: 0, end: 0})),
                    field: Box::new(Expression::Identifier(Identifier::from("prop"), Span {start: 0, end: 0})),
                    span: Span {start: 0, end: 0},
                }),
                args: vec![
                    Expression::Number(1, Span {start: 0, end: 0}),
                    Expression::StringLiteral("foo".to_owned(), Span {start: 0, end: 0}),
                    Expression::Number(3, Span {start: 0, end: 0}),
                ],
                span: Span {start: 0, end: 0},
            }
        );
    }

    #[test]
    fn empty_program() {
        test_method(r#""#, |p| p.program(), |p| p.ast(),
            Program::from(Vec::new()));

        test_method(r#"
        "#, |p| p.program(), |p| p.ast(),
            Program::from(Vec::new()));

        test_method(r#"

        "#, |p| p.program(), |p| p.ast(),
            Program::from(Vec::new()));
    }

    #[test]
    fn leading_whitespace() {
        test_method(r#"

        foo();
        "#, |p| p.program(), |p| p.ast(),
            Program::from(vec![
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("foo"), Span {start: 0, end: 0})),
                        args: vec![],
                        span: Span {start: 0, end: 0},
                    },
                    span: Span {start: 0, end: 0},
                }
            ])
        );
    }

    #[test]
    fn binary_operators() {
        test_method(r#"
        a || b;
        a && b;
        a == b;
        a != b;
        a >= b;
        a <= b;
        a > b;
        a < b;
        a && b || c;
        a && b || c && d;
        a == b || c && d;
        a == b || c != d;
        a && b || c >= d;
        a <= b || c >= d;
        a < b || c > d;
        a && b && c;
        a && b && c && d;
        a == b && c && d;
        a == b && c != d;
        a && b && c >= d;
        a <= b && c >= d;
        a < b && c > d;
        "#, |p| p.program(), |p| p.ast(),
            Program::from(vec![
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 9, end: 15})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 9, end: 10}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 14, end: 15}),
                        ],
                        span: Span {start: 9, end: 15},
                    },
                    span: Span {start: 9, end: 15},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 25, end: 31})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 25, end: 26}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 30, end: 31}),
                        ],
                        span: Span {start: 25, end: 31},
                    },
                    span: Span {start: 25, end: 31},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"), Span {start: 41, end: 47})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 41, end: 42}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 46, end: 47}),
                        ],
                        span: Span {start: 41, end: 47},
                    },
                    span: Span {start: 41, end: 47},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::ne"), Span {start: 57, end: 63})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 57, end: 58}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 62, end: 63}),
                        ],
                        span: Span {start: 57, end: 63},
                    },
                    span: Span {start: 57, end: 63},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"), Span {start: 73, end: 79})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 73, end: 74}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 78, end: 79}),
                        ],
                        span: Span {start: 73, end: 79},
                    },
                    span: Span {start: 73, end: 79},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::le"), Span {start: 89, end: 95})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 89, end: 90}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 94, end: 95}),
                        ],
                        span: Span {start: 89, end: 95},
                    },
                    span: Span {start: 89, end: 95},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::gt"), Span {start: 105, end: 110})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 105, end: 106}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 109, end: 110}),
                        ],
                        span: Span {start: 105, end: 110},
                    },
                    span: Span {start: 105, end: 110},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::lt"), Span {start: 120, end: 125})),
                        args: vec![
                            Expression::Identifier(Identifier::from("a"), Span {start: 120, end: 121}),
                            Expression::Identifier(Identifier::from("b"), Span {start: 124, end: 125}),
                        ],
                        span: Span {start: 120, end: 125},
                    },
                    span: Span {start: 120, end: 125},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 135, end: 146})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 135, end: 141})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 135, end: 136}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 140, end: 141}),
                                ],
                                span: Span {start: 135, end: 141},
                            },
                            Expression::Identifier(Identifier::from("c"), Span {start: 145, end: 146}),
                        ],
                        span: Span {start: 135, end: 146},
                    },
                    span: Span {start: 135, end: 146},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 156, end: 172})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 156, end: 162})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 156, end: 157}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 161, end: 162}),
                                ],
                                span: Span {start: 156, end: 162},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 166, end: 172})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 166, end: 167}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 171, end: 172}),
                                ],
                                span: Span {start: 166, end: 172},
                            }
                        ],
                        span: Span {start: 156, end: 172},
                    },
                    span: Span {start: 156, end: 172},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 182, end: 198})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"), Span {start: 182, end: 188})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 182, end: 183}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 187, end: 188}),
                                ],
                                span: Span {start: 182, end: 188},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 192, end: 198})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 192, end: 193}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 197, end: 198}),
                                ],
                                span: Span {start: 192, end: 198},
                            },
                        ],
                        span: Span {start: 182, end: 198},
                    },
                    span: Span {start: 182, end: 198},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 208, end: 224})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"), Span {start: 208, end: 214})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 208, end: 209}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 213, end: 214}),
                                ],
                                span: Span {start: 208, end: 214},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::ne"), Span {start: 218, end: 224})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 218, end: 219}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 223, end: 224}),
                                ],
                                span: Span {start: 218, end: 224},
                            },
                        ],
                        span: Span {start: 208, end: 224},
                    },
                    span: Span {start: 208, end: 224},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 234, end: 250})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 234, end: 240})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 234, end: 235}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 239, end: 240}),
                                ],
                                span: Span {start: 234, end: 240},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"), Span {start: 244, end: 250})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 244, end: 245}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 249, end: 250}),
                                ],
                                span: Span {start: 244, end: 250},
                            },
                        ],
                        span: Span {start: 234, end: 250},
                    },
                    span: Span {start: 234, end: 250},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 260, end: 276})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::le"), Span {start: 260, end: 266})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 260, end: 261}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 265, end: 266}),
                                ],
                                span: Span {start: 260, end: 266},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"), Span {start: 270, end: 276})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 270, end: 271}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 275, end: 276}),
                                ],
                                span: Span {start: 270, end: 276},
                            },
                        ],
                        span: Span {start: 260, end: 276},
                    },
                    span: Span {start: 260, end: 276},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"), Span {start: 286, end: 300})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::lt"), Span {start: 286, end: 291})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 286, end: 287}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 290, end: 291}),
                                ],
                                span: Span {start: 286, end: 291},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::gt"), Span {start: 295, end: 300})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 295, end: 296}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 299, end: 300}),
                                ],
                                span: Span {start: 295, end: 300},
                            },
                        ],
                        span: Span {start: 286, end: 300},
                    },
                    span: Span {start: 286, end: 300},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 310, end: 321})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 310, end: 316})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 310, end: 311}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 315, end: 316}),
                                ],
                                span: Span {start: 310, end: 316},
                            },
                            Expression::Identifier(Identifier::from("c"), Span {start: 320, end: 321}),
                        ],
                        span: Span {start: 310, end: 321},
                    },
                    span: Span {start: 310, end: 321},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 331, end: 347})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 331, end: 342})),
                                args: vec![
                                    Expression::Call {
                                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 331, end: 337})),
                                        args: vec![
                                            Expression::Identifier(Identifier::from("a"), Span {start: 331, end: 332}),
                                            Expression::Identifier(Identifier::from("b"), Span {start: 336, end: 337}),
                                        ],
                                        span: Span {start: 331, end: 337},
                                    },
                                    Expression::Identifier(Identifier::from("c"), Span {start: 341, end: 342}),
                                ],
                                span: Span {start: 331, end: 342},
                            },
                            Expression::Identifier(Identifier::from("d"), Span {start: 346, end: 347}),
                        ],
                        span: Span {start: 331, end: 347},
                    },
                    span: Span {start: 331, end: 347},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 357, end: 373})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 357, end: 368})),
                                args: vec![
                                    Expression::Call {
                                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"), Span {start: 357, end: 363})),
                                        args: vec![
                                            Expression::Identifier(Identifier::from("a"), Span {start: 357, end: 358}),
                                            Expression::Identifier(Identifier::from("b"), Span {start: 362, end: 363}),
                                        ],
                                        span: Span {start: 357, end: 363},
                                    },
                                    Expression::Identifier(Identifier::from("c"), Span {start: 367, end: 368}),
                                ],
                                span: Span {start: 357, end: 368},
                            },
                            Expression::Identifier(Identifier::from("d"), Span {start: 372, end: 373}),
                        ],
                        span: Span {start: 357, end: 373},
                    },
                    span: Span {start: 357, end: 373},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 383, end: 399})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"), Span {start: 383, end: 389})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 383, end: 384}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 388, end: 389}),
                                ],
                                span: Span {start: 383, end: 389},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::ne"), Span {start: 393, end: 399})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 393, end: 394}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 398, end: 399}),
                                ],
                                span: Span {start: 393, end: 399},
                            },
                        ],
                        span: Span {start: 383, end: 399},
                    },
                    span: Span {start: 383, end: 399},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 409, end: 425})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 409, end: 415})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 409, end: 410}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 414, end: 415}),
                                ],
                                span: Span {start: 409, end: 415},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"), Span {start: 419, end: 425})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 419, end: 420}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 424, end: 425}),
                                ],
                                span: Span {start: 419, end: 425},
                            },
                        ],
                        span: Span {start: 409, end: 425},
                    },
                    span: Span {start: 409, end: 425},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 435, end: 451})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::le"), Span {start: 435, end: 441})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 435, end: 436}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 440, end: 441}),
                                ],
                                span: Span {start: 435, end: 441},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"), Span {start: 445, end: 451})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 445, end: 446}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 450, end: 451}),
                                ],
                                span: Span {start: 445, end: 451},
                            },
                        ],
                        span: Span {start: 435, end: 451},
                    },
                    span: Span {start: 435, end: 451},
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"), Span {start: 461, end: 475})),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::lt"), Span {start: 461, end: 466})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("a"), Span {start: 461, end: 462}),
                                    Expression::Identifier(Identifier::from("b"), Span {start: 465, end: 466}),
                                ],
                                span: Span {start: 461, end: 466},
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::gt"), Span {start: 470, end: 475})),
                                args: vec![
                                    Expression::Identifier(Identifier::from("c"), Span {start: 470, end: 471}),
                                    Expression::Identifier(Identifier::from("d"), Span {start: 474, end: 475}),
                                ],
                                span: Span {start: 470, end: 475},
                            },
                        ],
                        span: Span {start: 461, end: 475},
                    },
                    span: Span {start: 461, end: 475},
                },
            ])
        );
    }

    #[test]
    fn conditionals() {
        // Basic if
        test_method(r#"
        if foo {
            a();
        }
        "#.trim(), |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::Branch {
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"), Span {start: 3, end: 6})),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"), Span {start: 21, end: 24})),
                                args: vec![],
                                span: Span {start: 21, end: 24},
                            },
                            span: Span {start: 21, end: 24},
                        },
                    ],
                    otherwise: None,
                    span: Span {start: 3, end: 6},
                },
                span: Span {start: 0, end: 35},
            }
        );

        // Basic if else
        test_method(r#"
        if foo {
            a();
        }
        else {
            b();
        }
        "#.trim(), |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::Branch {
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"), Span {start: 0, end: 0})),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"), Span {start: 0, end: 0})),
                                args: vec![],
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("b"), Span {start: 0, end: 0})),
                                args: vec![],
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ]),
                    span: Span {start: 0, end: 0},
                },
                span: Span {start: 0, end: 0},
            }
        );

        // Basic if else-if else
        test_method(r#"
        if foo {
            a();
        }
        else if foo2 {
            c();
        }
        else if foo3 {
            d();
        }
        else {
            b();
        }
        "#.trim(), |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::Branch {
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"), Span {start: 0, end: 0})),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"), Span {start: 0, end: 0})),
                                args: vec![],
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Branch {
                                condition: Box::new(Expression::Identifier(Identifier::from("foo2"), Span {start: 0, end: 0})),
                                body: vec![
                                    Statement::Expression {
                                        expr: Expression::Call {
                                            method: Box::new(Expression::Identifier(Identifier::from("c"), Span {start: 0, end: 0})),
                                            args: vec![],
                                            span: Span {start: 0, end: 0},
                                        },
                                        span: Span {start: 0, end: 0},
                                    },
                                ],
                                otherwise: Some(vec![
                                    Statement::Expression {
                                        expr: Expression::Branch {
                                            condition: Box::new(Expression::Identifier(Identifier::from("foo3"), Span {start: 0, end: 0})),
                                            body: vec![
                                                Statement::Expression {
                                                    expr: Expression::Call {
                                                        method: Box::new(Expression::Identifier(Identifier::from("d"), Span {start: 0, end: 0})),
                                                        args: vec![],
                                                        span: Span {start: 0, end: 0},
                                                    },
                                                    span: Span {start: 0, end: 0},
                                                },
                                            ],
                                            otherwise: Some(vec![
                                                Statement::Expression {
                                                    expr: Expression::Call {
                                                        method: Box::new(Expression::Identifier(Identifier::from("b"), Span {start: 0, end: 0})),
                                                        args: vec![],
                                                        span: Span {start: 0, end: 0},
                                                    },
                                                    span: Span {start: 0, end: 0},
                                                },
                                            ]),
                                            span: Span {start: 0, end: 0},
                                        },
                                        span: Span {start: 0, end: 0},
                                    },
                                ]),
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ]),
                    span: Span {start: 0, end: 0},
                },
                span: Span {start: 0, end: 0},
            }
        );

        // Basic if else-if (no else)
        test_method(r#"
        if foo {
            a();
        }
        else if foo2 {
            c();
        }
        else if foo3 {
            d();
        }
        "#.trim(), |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::Branch {
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"), Span {start: 0, end: 0})),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"), Span {start: 0, end: 0})),
                                args: vec![],
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Branch {
                                condition: Box::new(Expression::Identifier(Identifier::from("foo2"), Span {start: 0, end: 0})),
                                body: vec![
                                    Statement::Expression {
                                        expr: Expression::Call {
                                            method: Box::new(Expression::Identifier(Identifier::from("c"), Span {start: 0, end: 0})),
                                            args: vec![],
                                            span: Span {start: 0, end: 0},
                                        },
                                        span: Span {start: 0, end: 0},
                                    },
                                ],
                                otherwise: Some(vec![
                                    Statement::Expression {
                                        expr: Expression::Branch {
                                            condition: Box::new(Expression::Identifier(Identifier::from("foo3"), Span {start: 0, end: 0})),
                                            body: vec![
                                                Statement::Expression {
                                                    expr: Expression::Call {
                                                        method: Box::new(Expression::Identifier(Identifier::from("d"), Span {start: 0, end: 0})),
                                                        args: vec![],
                                                        span: Span {start: 0, end: 0},
                                                    },
                                                    span: Span {start: 0, end: 0},
                                                },
                                            ],
                                            otherwise: None,
                                            span: Span {start: 0, end: 0},
                                        },
                                        span: Span {start: 0, end: 0},
                                    },
                                ]),
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ]),
                    span: Span {start: 0, end: 0},
                },
                span: Span {start: 0, end: 0},
            }
        );

        // Declaration using if expression
        test_method(r#"
        let a: u8 = if foo {
            1
        }
        else if bar7 {
            2
        }
        else {
            3
        };
        "#.trim(), |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Declaration {
                pattern: Pattern::Identifier(Identifier::from("a"), Span {start: 0, end: 0}),
                type_def: TypeDefinition::Name {
                    name: Identifier::from("u8"),
                    span: Span {start: 0, end: 0},
                },
                expr: Some(Expression::Branch {
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"), Span {start: 0, end: 0})),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Number(1, Span {start: 0, end: 0}),
                            span: Span {start: 0, end: 0},
                        },
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Branch {
                                condition: Box::new(Expression::Identifier(Identifier::from("bar7"), Span {start: 0, end: 0})),
                                body: vec![
                                    Statement::Expression {
                                        expr: Expression::Number(2, Span {start: 0, end: 0}),
                                        span: Span {start: 0, end: 0},
                                    },
                                ],
                                otherwise: Some(vec![
                                    Statement::Expression {
                                        expr: Expression::Number(3, Span {start: 0, end: 0}),
                                        span: Span {start: 0, end: 0},
                                    },
                                ]),
                                span: Span {start: 0, end: 0},
                            },
                            span: Span {start: 0, end: 0},
                        },
                    ]),
                    span: Span {start: 0, end: 0},
                }),
                span: Span {start: 0, end: 0},
            }
        );
    }

    fn test_parse<F>(input: &'static str, parse: F, tokens: Vec<Token<Rule>>)
        where F: FnOnce(&mut Rdp<StringInput>) -> bool {

        let mut parser = parser_from(input);
        assert!(parse(&mut parser), "Parsing failed");
        assert!(parser.end(), "Parser did not reach eoi");

        assert_eq!(parser.queue(), &tokens);
    }

    fn test_method<T: Debug + PartialEq, F, P>(input: &'static str, parse: P, method: F, expected: T)
        where P: FnOnce(&mut Rdp<StringInput>) -> bool,
              F: FnOnce(&Rdp<StringInput>) -> T {

        let mut parser = parser_from(input);
        assert!(parse(&mut parser), "Parsing failed");
        assert!(parser.end(), "Parser did not reach eoi");

        assert_eq!(method(&parser), expected);
    }

    fn test_fail<F>(input: &'static str, parse: F)
        where F: FnOnce(&mut Rdp<StringInput>) -> bool {

        let mut parser = parser_from(input);
        assert!(!parse(&mut parser), "Parsing passed when expected it to fail");
        assert!(!parser.end(), "Parser reached end when expected it to fail");

        assert!(parser.queue().is_empty(), "Queue was not empty despite expecting to fail");
    }

    fn parser_from(s: &'static str) -> Rdp<StringInput> {
        Rdp::new(StringInput::new(s))
    }
}
