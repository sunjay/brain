use std::iter;
use std::collections::VecDeque;

use pest::prelude::*;

use super::*;

impl_rdp! {
    grammar! {
        program = _{ statement* ~ eoi }

        // conditional is technically an expression too but it can be used as a statement
        // without a semicolon as well
        statement = { declaration | assignment | while_loop | conditional | (expr ~ semi) | comment }

        comment = @{ block_comment | line_comment }
        line_comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
        block_comment = _{ ["/*"] ~ ((!(["*/"]) ~ any) | block_comment)* ~ ["*/"] }

        assignment = { identifier ~ op_assign ~ expr ~ semi}
        declaration = { ["let"] ~ pattern ~ [":"] ~ type_def ~ (op_assign ~ expr)? ~ semi}
        op_assign = { ["="] }
        pattern = { identifier }

        type_def = _{ identifier | array_type }
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
        parse_program(&self) -> Program {
            (list: _program()) => {
                Program::new(list.into_iter().collect())
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
            (_: declaration, pattern: _pattern(), type_def: _type_def(), _: op_assign, _: expr, expr: _expr(), _: semi) => {
                Statement::Declaration {pattern: pattern, type_def: type_def, expr: Some(expr)}
            },
            (_: declaration, pattern: _pattern(), type_def: _type_def(), _: semi) => {
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
            // This should always be last as it will catch pretty much any cases that weren't caught above
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
            (_: func_call, method: _identifier(), args: _func_args()) => {
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
                    method: Box::new(Expression::Identifier("std::ops::Or::or".to_owned())),
                    args: vec![lhs, rhs],
                }
            },
            (_: bool_and, lhs: _expr(), _: op_bool_and, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier("std::ops::And::and".to_owned())),
                    args: vec![lhs, rhs],
                }
            },
            (_: comparison, lhs: _expr(), op_token, rhs: _expr()) => {
                Expression::Call {
                    method: Box::new(Expression::Identifier(match op_token.rule {
                        Rule::op_eq => "std::cmp::PartialEq::eq",
                        Rule::op_ne => "std::cmp::PartialEq::ne",
                        Rule::op_ge => "std::cmp::PartialOrd::ge",
                        Rule::op_le => "std::cmp::PartialOrd::le",
                        Rule::op_gt => "std::cmp::PartialOrd::gt",
                        Rule::op_lt => "std::cmp::PartialOrd::lt",
                        _ => unreachable!(),
                    }.to_owned())),
                    args: vec![lhs, rhs],
                }
            },
            (&ident: identifier) => {
                Expression::Identifier(ident.into())
            },
            (_: string_literal, s: _literal_chars()) => {
                Expression::StringLiteral(s.into_iter().collect())
            },
            (&s: number) => {
                // If our grammar is correct, we are guarenteed that this will work
                Expression::Number(s.parse().unwrap())
            },
        }

        _field_access(&self) -> Expression {
            (target: _identifier(), _: op_access, field: _identifier(), args: _func_args()) => {
                Expression::Call {
                    method: Box::new(Expression::Access {
                        target: Box::new(Expression::Identifier(target)),
                        field: Box::new(Expression::Identifier(field)),
                    }),
                    args: args,
                }
            },
            (target: _identifier(), _: op_access, field: _identifier()) => {
                Expression::Access {
                    target: Box::new(Expression::Identifier(target)),
                    field: Box::new(Expression::Identifier(field)),
                }
            },
        }

        _conditional(&self) -> Expression {
            (_: expr, expr: _expr(), block: _block(), _: op_else_if, branches: _branches(), _: op_else, else_block: _block()) => {
                Expression::ConditionGroup {
                    branches: iter::once((expr, block)).chain(branches).collect(),
                    default: Some(else_block),
                }
            },
            (_: expr, expr: _expr(), block: _block(), _: op_else_if, branches: _branches()) => {
                Expression::ConditionGroup {
                    branches: iter::once((expr, block)).chain(branches).collect(),
                    default: None,
                }
            },
            (_: expr, expr: _expr(), block: _block(), _: op_else, else_block: _block()) => {
                Expression::ConditionGroup {
                    branches: vec![(expr, block)],
                    default: Some(else_block),
                }
            },
            (_: expr, expr: _expr(), block: _block()) => {
                Expression::ConditionGroup {
                    branches: vec![(expr, block)],
                    default: None,
                }
            },
        }

        _branches(&self) -> VecDeque<(Expression, Block)> {
            (_: expr, expr: _expr(), block: _block(), _: op_else_if, mut tail: _branches()) => {
                tail.push_front((expr, block));

                tail
            },
            (_: expr, expr: _expr(), block: _block()) => {
                let mut queue = VecDeque::new();
                queue.push_front((expr, block));

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
            (_: expr, head: _expr(), mut tail: _block_deque()) => {
                tail.push_front(Statement::Expression {expr: head});

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
            (&ident: identifier) => {
                ident.into()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use pest::prelude::*;

    use super::*;
    // ast, etc.
    use super::super::*;

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
    fn string_literal_escapes() {
        test_method(r#""foo""#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::StringLiteral("foo".to_owned()));

        test_method(r#""\\ \" \' \n \r \t \0""#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::StringLiteral("\\ \" \' \n \r \t \0".to_owned()));
    }

    #[test]
    fn functions_field_access() {
        test_method(r#"func(1, "foo", 3)"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Call {
                method: Box::new(Expression::Identifier("func".to_owned())),
                args: vec![
                    Expression::Number(1),
                    Expression::StringLiteral("foo".to_owned()),
                    Expression::Number(3),
                ],
            }
        );

        test_method(r#"thing.prop(1, "foo", 3)"#, |p| p.expr(), |p| {p.inc_queue_index(); p._expr()},
            Expression::Call {
                method: Box::new(Expression::Access {
                    target: Box::new(Expression::Identifier("thing".to_owned())),
                    field: Box::new(Expression::Identifier("prop".to_owned())),
                }),
                args: vec![
                    Expression::Number(1),
                    Expression::StringLiteral("foo".to_owned()),
                    Expression::Number(3),
                ],
            }
        );
    }

    #[test]
    fn binary_operators() {
        // Basic if
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
        "#, |p| p.program(), |p| p.parse_program(),
            Program::new(vec![
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
                        args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Identifier(Identifier::from("c")),
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::cmp::PartialEq::eq"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                                args: vec![Expression::Identifier(Identifier::from("c")), Expression::Identifier(Identifier::from("d"))],
                            },
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::Or::or"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                                args: vec![Expression::Identifier(Identifier::from("a")), Expression::Identifier(Identifier::from("b"))],
                            },
                            Expression::Identifier(Identifier::from("c")),
                        ],
                    },
                },
                Statement::Expression {
                    expr: Expression::Call {
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                                args: vec![
                                    Expression::Call {
                                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
                        args: vec![
                            Expression::Call {
                                method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
                        method: Box::new(Expression::Identifier(Identifier::from("std::ops::And::and"))),
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
        "#, |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::ConditionGroup {
                    branches: vec![
                        (Expression::Identifier("foo".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("a".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                    ],
                    default: None,
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
        "#, |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::ConditionGroup {
                    branches: vec![
                        (Expression::Identifier("foo".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("a".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                    ],
                    default: Some(vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier("b".to_owned())),
                                args: vec![],
                            }
                        },
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
        "#, |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::ConditionGroup {
                    branches: vec![
                        (Expression::Identifier("foo".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("a".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                        (Expression::Identifier("foo2".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("c".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                        (Expression::Identifier("foo3".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("d".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                    ],
                    default: Some(vec![
                        Statement::Expression {
                            expr: Expression::Call {
                                method: Box::new(Expression::Identifier("b".to_owned())),
                                args: vec![],
                            }
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
        "#, |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Expression {
                expr: Expression::ConditionGroup {
                    branches: vec![
                        (Expression::Identifier("foo".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("a".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                        (Expression::Identifier("foo2".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("c".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                        (Expression::Identifier("foo3".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Call {
                                    method: Box::new(Expression::Identifier("d".to_owned())),
                                    args: vec![],
                                },
                            },
                        ]),
                    ],
                    default: None,
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
        "#, |p| p.statement(), |p| {p.inc_queue_index(); p._statement()},
            Statement::Declaration {
                pattern: Pattern::Identifier("a".to_owned()),
                type_def: TypeDefinition::Name {
                    name: "u8".to_owned(),
                },
                expr: Some(Expression::ConditionGroup {
                    branches: vec![
                        (Expression::Identifier("foo".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Number(1),
                            },
                        ]),
                        (Expression::Identifier("bar7".to_owned()), vec![
                            Statement::Expression {
                                expr: Expression::Number(2),
                            },
                        ]),
                    ],
                    default: Some(vec![
                        Statement::Expression {
                            expr: Expression::Number(3),
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
        Rdp::new(StringInput::new(s.trim()))
    }
}
