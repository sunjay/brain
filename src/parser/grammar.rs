use std::fmt;
use std::collections::VecDeque;

use pest::prelude::*;

use super::*;

impl_rdp! {
    grammar! {
        module = _{ soi ~ statement* ~ eoi }

        // conditional is technically an expression too but it can be used as a statement
        // without a semicolon as well
        statement = { declaration | assignment | while_loop | conditional | (expr ~ semi) | comment }

        comment = @{ block_comment | line_comment }
        line_comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
        block_comment = _{ ["/*"] ~ ((!(["*/"]) ~ any) | block_comment)* ~ ["*/"] }

        assignment = { identifier ~ op_assign ~ expr ~ semi}
        declaration = { ["let"] ~ ["mut"]? ~ pattern ~ op_declare_type ~ type_def ~ (op_assign ~ expr)? ~ semi}
        op_declare_type = { [":"] }
        op_assign = { ["="] }
        pattern = { identifier }

        type_def = _{ identifier | array_type }
        array_type = { ["["] ~ type_def ~ semi ~ array_size ~ ["]"] }
        array_size = _{ unspecified | expr }
        unspecified = { ["_"] }

        while_loop = { ["while"] ~ expr ~ block }

        expr = {
            { bool_not | func_call | field_access | string_literal | bool_literal | identifier | conditional | number }

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

        bool_not = _{ op_bool_not ~ expr }
        op_bool_not = { ["!"] }

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

        string_literal = @{ ["b\""] ~ literal_char* ~ ["\""] }
        literal_char = { escape_sequence | (!["\""] ~ any) }
        escape_sequence = _{ ["\\\\"] | ["\\\""] | ["\\\'"] | ["\\n"] | ["\\r"] | ["\\t"] | ["\\0"] }

        bool_literal = @{ ["true"] | ["false"] }

        identifier = @{ !keyword ~ (alpha | ["_"]) ~ (alphanumeric | ["_"])* }
        alpha = _{ ['a'..'z'] | ['A'..'Z'] }
        alphanumeric = _{ alpha | ['0'..'9'] }

        number = @{ ["0"] | (nonzero ~ digit*) }
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
        // Top-level method that returns the abstract syntax tree based on the contents of the
        // parser queue
        // Make sure to call module() before this so there is something in the queue
        module_ast(&self) -> Module {
            (statements: _module()) => {
                Module {
                    body: statements.into_iter().collect::<Block>(),
                }
            },
        }

        _module(&self) -> VecDeque<Statement> {
            (_: statement, head: _statement(), mut tail: _module()) => {
                tail.push_front(head);

                tail
            },
            (&text: comment, mut tail: _module()) => {
                tail.push_front(Statement::Comment(text.into()));

                tail
            },
            () => {
                let mut tail = VecDeque::new();
                // We do this so the last statement in a block always represents its return type
                tail.push_front(Statement::Expression {expr: Expression::UnitLiteral});
                tail
            },
        }

        _statement(&self) -> Statement {
            (&text: comment) => {
                Statement::Comment(text.into())
            },
            (_: declaration, pattern: _pattern(), _: op_declare_type, type_def: _type_def(), _: op_assign, _: expr, expr: _expr(), _: semi) => {
                Statement::Declaration {pattern: pattern, type_def: type_def, expr: Some(expr)}
            },
            (_: declaration, pattern: _pattern(), _: op_declare_type, type_def: _type_def(), _: semi) => {
                Statement::Declaration {pattern: pattern, type_def: type_def, expr: None}
            },
            (_: assignment, ident: _identifier(), _: op_assign, _: expr, expr: _expr(), _: semi) => {
                Statement::Assignment {lhs: ident, expr: expr}
            },
            (_: while_loop, _: expr, condition: _expr(), body: _block()) => {
                Statement::WhileLoop {condition: condition, body: body}
            },
            (_: conditional, expr: _conditional()) => {
                Statement::Expression {expr: expr}
            },
            // This should always be lmodule_ast as it will catch pretty much any cases that weren't caught above
            (_: expr, expr: _expr(), _: semi) => {
                Statement::Expression {expr: expr}
            },
        }

        _pattern(&self) -> Pattern {
            (_: pattern, ident: _identifier()) => {
                Pattern::Identifier(ident)
            },
        }

        _type_def(&self) -> TypeDefinition {
            (_: array_type, type_def: _type_def(), _: semi, _: unspecified) => {
                TypeDefinition::Array {type_def: Box::new(type_def), size: None}
            },
            (_: array_type, type_def: _type_def(), _: semi, _: expr, size: _expr()) => {
                TypeDefinition::Array {type_def: Box::new(type_def), size: Some(size)}
            },
            (ident: _identifier()) => {
                TypeDefinition::Name {name: ident}
            },
        }

        _expr(&self) -> Expression {
            (_: op_bool_not, _:expr, expr: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from("std::ops::Not"))),
                    args: vec![expr],
                }
            },
            (_: func_call, method: _identifier(), args: _call_args()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(method)),
                    args: args,
                }
            },
            (_: field_access, expr: _field_access()) => {
                expr
            },
            (_: conditional, expr: _conditional()) => {
                expr
            },
            (_: bool_or, lhs: _expr(), _: op_bool_or, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                    args: vec![lhs, rhs],
                }
            },
            (_: bool_and, lhs: _expr(), _: op_bool_and, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                    args: vec![lhs, rhs],
                }
            },
            (_: comparison, lhs: _expr(), op_token, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(Identifier::from(match op_token.rule {
                        Rule::op_eq => "std::cmp::PartialEq::eq",
                        Rule::op_ne => "std::cmp::PartialEq::ne",
                        Rule::op_ge => "std::cmp::PartialOrd::ge",
                        Rule::op_le => "std::cmp::PartialOrd::le",
                        Rule::op_gt => "std::cmp::PartialOrd::gt",
                        Rule::op_lt => "std::cmp::PartialOrd::lt",
                        _ => unreachable!(),
                    }))),
                    args: vec![lhs, rhs],
                }
            },
            (&ident: bool_literal) => {
                Expression::Identifier(ident.into())
            },
            (&ident: identifier) => {
                Expression::Identifier(ident.into())
            },
            (_: string_literal, s: _literal_chars()) => {
                Expression::ByteLiteral(s.into_iter().collect())
            },
            (&s: number) => {
                // If our grammar is correct, we are guarenteed that this will work
                Expression::Number(s.replace("_", "").parse().unwrap())
            },
        }

        _field_access(&self) -> Expression {
            (target: _identifier(), _: op_access, field: _identifier(), args: _call_args()) => {
                Expression::Call {
                    method: Box::new(Expression::Access {
                        target: Box::new(Expression::Identifier(target)),
                        field: field,
                    }),
                    args: args,
                }
            },
            (target: _identifier(), _: op_access, field: _identifier()) => {
                Expression::Access {
                    target: Box::new(Expression::Identifier(target)),
                    field: field,
                }
            },
        }

        _conditional(&self) -> Expression {
            (_: expr, expr: _expr(), block: _block(), _: op_else_if, branches: _branches(), _: op_else, else_block: _block()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: Some(nest_else_ifs(branches, Some(else_block))),
                }
            },
            (_: expr, expr: _expr(), block: _block(), _: op_else_if, branches: _branches()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: Some(nest_else_ifs(branches, None)),
                }
            },
            (_: expr, expr: _expr(), block: _block(), _: op_else, else_block: _block()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: Some(else_block),
                }
            },
            (_: expr, expr: _expr(), block: _block()) => {
                Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: None,
                }
            },
        }

        _branches(&self) -> VecDeque<Expression> {
            (_: expr, expr: _expr(), block: _block(), _: op_else_if, mut tail: _branches()) => {
                tail.push_front(Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: None,
                });

                tail
            },
            (_: expr, expr: _expr(), block: _block()) => {
                let mut queue = VecDeque::new();
                queue.push_front(Expression::Branch {
                    condition: Box::new(expr),
                    body: block,
                    otherwise: None,
                });

                queue
            },
        }

        _call_args(&self) -> CallArgs {
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
            (&text: comment, mut tail: _block_deque()) => {
                tail.push_front(Statement::Comment(text.into()));

                tail
            },
            (_: expr, head: _expr(), _: block_end) => {
                let mut tail = VecDeque::new();
                tail.push_front(Statement::Expression {expr: head});

                tail
            },
            (_: block_end) => {
                let mut tail = VecDeque::new();
                // We do this so the last statement in a block always represents its return type
                tail.push_front(Statement::Expression {expr: Expression::UnitLiteral});
                tail
            },
        }

        _literal_chars(&self) -> VecDeque<u8> {
            (&c: literal_char, mut tail: _literal_chars()) => {
                if c.len() == 2 {
                    debug_assert!(c.bytes().next().unwrap() == b'\\');
                    tail.push_front(match c.bytes().nth(1).unwrap() {
                        b'\\' => b'\\',
                        b'"' => b'"',
                        b'\'' => b'\'',
                        b'n' => b'\n',
                        b'r' => b'\r',
                        b't' => b'\t',
                        b'0' => b'\0',
                        //TODO: Replace this with a proper result when upgrading to pest 1.0
                        _ => panic!("Unknown escape: {}", c)
                    });
                }
                else {
                    debug_assert!(c.len() == 1);
                    tail.push_front(c.bytes().next().unwrap());
                }

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
        Some(vec![Statement::Expression {
            expr: {
                match br {
                    Expression::Branch {ref mut otherwise, ..} => {
                        *otherwise = acc;
                    },
                    _ => unreachable!(),
                };
                br
            },
        }])
    }).unwrap()
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just to make things a bit more ergonomic
        use self::Rule::*;

        write!(f, "{}", match *self {
            eoi => "EOF",
            comment => "comment",
            identifier => "identifier",
            keyword => "keyword",
            number => "number",
            string_literal => "string literal",
            bool_literal => "boolean literal",
            literal_char => "character",
            any => "any character",

            unspecified => "`_`",
            semi => "`;`",

            bool_or => "`or`",
            bool_and => "`and`",
            conditional => "`if`",
            op_else_if => "`else if`",
            op_else => "`else`",

            op_assign => "`=`",
            op_bool_or => "`||`",
            op_bool_and => "`&&`",
            op_bool_not => "`!`",
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
            statement | assignment | declaration | pattern | array_type | while_loop | comparison |
            func_call | field_access | expr | soi => unreachable!("{:?}", *self),
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
        test_parse(r#"b"""#, |p| p.string_literal(), vec![
            Token::new(Rule::string_literal, 0, 3),
        ]);

        test_parse(r#"b"foo""#, |p| p.string_literal(), vec![
            Token::new(Rule::string_literal, 0, 6),
            Token::new(Rule::literal_char, 2, 3),
            Token::new(Rule::literal_char, 3, 4),
            Token::new(Rule::literal_char, 4, 5),
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
            Expression::Number(0)
        );

        test_method(r#"100"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(100)
        );

        test_method(r#"1_000_000"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(1_000_000)
        );

        test_method(r#"1_000_000_"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(1_000_000_)
        );

        test_method(r#"1____0_0__0______000____"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Number(1____0_0__0______000____)
        );
    }

    #[test]
    fn string_literal_escapes() {
        test_method(r#"b"foo""#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::ByteLiteral(b"foo".to_vec()));

        test_method(r#"b"\\ \" \' \n \r \t \0""#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::ByteLiteral(b"\\ \" \' \n \r \t \0".to_vec()));
    }

    #[test]
    fn functions_field_access() {
        test_method(r#"func(1, b"foo", 3)"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Call {
                method: Box::new(Expression::Identifier(Identifier::from("func"))),
                args: vec![
                    Expression::Number(1),
                    Expression::ByteLiteral(b"foo".to_vec()),
                    Expression::Number(3),
                ],
            }
        );

        test_method(r#"thing.prop(1, b"foo", 3)"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Call {
                method: Box::new(Expression::Access {
                    target: Box::new(Expression::Identifier(Identifier::from("thing"))),
                    field: Identifier::from("prop"),
                }),
                args: vec![
                    Expression::Number(1),
                    Expression::ByteLiteral(b"foo".to_vec()),
                    Expression::Number(3),
                ],
            }
        );
    }

    #[test]
    fn empty_program() {
        test_method(r#""#, |p| p.module(), |p| p.module_ast(),
            Module::empty());

        test_method(r#"
        "#, |p| p.module(), |p| p.module_ast(),
            Module::empty());

        test_method(r#"

        "#, |p| p.module(), |p| p.module_ast(),
            Module::empty());
    }

    #[test]
    fn leading_whitespace() {
        test_method(r#"

        foo();
        "#, |p| p.module(), |p| p.module_ast(),
            Module::from(vec![
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("foo"))),
                        args: vec![],
                    },
                },
                Statement::Expression {expr: Expression::UnitLiteral},
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
        "#, |p| p.module(), |p| p.module_ast(),
            Module::from(vec![
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::ne"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::le"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::gt"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::lt"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Identifier(Identifier::from("c")),
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::ne"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::le"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator||"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::lt"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::gt"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Identifier(Identifier::from("c")),
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![
                                    Expression::Call {
                                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                                    },
                                    Expression::Identifier(Identifier::from("c")),
                                ],
                            },
                            Expression::Identifier(Identifier::from("d")),
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![
                                    Expression::Call {
                                        method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"))),
                                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                                    },
                                    Expression::Identifier(Identifier::from("c")),
                                ],
                            },
                            Expression::Identifier(Identifier::from("d")),
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::ne"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::le"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::ge"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("operator&&"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::lt"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialOrd::gt"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {expr: Expression::UnitLiteral},
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
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"))),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"))),
                                args: vec![],
                            },
                        },
                        Statement::Expression {expr: Expression::UnitLiteral},
                    ],
                    otherwise: None,
                },
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
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"))),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"))),
                                args: vec![],
                            },
                        },
                        Statement::Expression {expr: Expression::UnitLiteral},
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("b"))),
                                args: vec![],
                            }
                        },
                        Statement::Expression {expr: Expression::UnitLiteral},
                    ]),
                },
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
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"))),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"))),
                                args: vec![],
                            },
                        },
                        Statement::Expression {expr: Expression::UnitLiteral},
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Branch {
                                condition: Box::new(Expression::Identifier(Identifier::from("foo2"))),
                                body: vec![
                                    Statement::Expression {
                                        expr: Expression::Call {
                                            method: Box::new(Expression::Identifier(Identifier::from("c"))),
                                            args: vec![],
                                        },
                                    },
                                    Statement::Expression {expr: Expression::UnitLiteral},
                                ],
                                otherwise: Some(vec![
                                    Statement::Expression {
                                        expr: Expression::Branch {
                                            condition: Box::new(Expression::Identifier(Identifier::from("foo3"))),
                                            body: vec![
                                                Statement::Expression {
                                                    expr: Expression::Call {
                                                        method: Box::new(Expression::Identifier(Identifier::from("d"))),
                                                        args: vec![],
                                                    },
                                                },
                                                Statement::Expression {expr: Expression::UnitLiteral},
                                            ],
                                            otherwise: Some(vec![
                                                Statement::Expression {
                                                    expr: Expression::Call {
                                                        method: Box::new(Expression::Identifier(Identifier::from("b"))),
                                                        args: vec![],
                                                    }
                                                },
                                                Statement::Expression {expr: Expression::UnitLiteral},
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        },
                    ]),
                },
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
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"))),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("a"))),
                                args: vec![],
                            },
                        },
                        Statement::Expression {expr: Expression::UnitLiteral},
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Branch {
                                condition: Box::new(Expression::Identifier(Identifier::from("foo2"))),
                                body: vec![
                                    Statement::Expression {
                                        expr: Expression::Call {
                                            method: Box::new(Expression::Identifier(Identifier::from("c"))),
                                            args: vec![],
                                        },
                                    },
                                    Statement::Expression {expr: Expression::UnitLiteral},
                                ],
                                otherwise: Some(vec![
                                    Statement::Expression {
                                        expr: Expression::Branch {
                                            condition: Box::new(Expression::Identifier(Identifier::from("foo3"))),
                                            body: vec![
                                                Statement::Expression {
                                                    expr: Expression::Call {
                                                        method: Box::new(Expression::Identifier(Identifier::from("d"))),
                                                        args: vec![],
                                                    },
                                                },
                                                Statement::Expression {expr: Expression::UnitLiteral},
                                            ],
                                            otherwise: None,
                                        },
                                    },
                                ]),
                            },
                        },
                    ]),
                },
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
                pattern: Pattern::Identifier(Identifier::from("a")),
                type_def: TypeDefinition::Name {
                    name: Identifier::from("u8"),
                },
                expr: Some(Expression::Branch {
                    condition: Box::new(Expression::Identifier(Identifier::from("foo"))),
                    body: vec![
                        Statement::Expression {
                            expr: Expression::Number(1),
                        },
                    ],
                    otherwise: Some(vec![
                        Statement::Expression {
                            expr: Expression::Branch {
                                condition: Box::new(Expression::Identifier(Identifier::from("bar7"))),
                                body: vec![
                                    Statement::Expression {
                                        expr: Expression::Number(2)
                                    },
                                ],
                                otherwise: Some(vec![
                                    Statement::Expression {
                                        expr: Expression::Number(3)
                                    },
                                ]),
                            },
                        },
                    ]),
                }),
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
