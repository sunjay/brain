use parser::{Statement, Expression};
use instructions::Instructions;

/// Expands the given statement into instructions
pub fn expand(instructions: &mut Instructions, stmt: Statement) {
    match stmt {
        Statement::Comment(_) => (),
        Statement::Output(expr) => output_expr(instructions, expr),
        declaration => println!("{:?}", declaration), //TODO
    }
}

fn output_expr(instructions: &mut Instructions, expr: Expression) {
    match expr {
        Expression::StringLiteral(text) => {
            for ch in text.as_bytes() {
                instructions.right(1);
                instructions.increment(*ch as usize);
                instructions.write(1);
            }
        },
        Expression::Identifier(ident) => println!("{:?}", ident), //TODO
    }
}
