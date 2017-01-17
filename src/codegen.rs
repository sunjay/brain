use parser::{Statement, Expression};
use instruction::Instruction;
use instruction::Instruction::*;

/// Expands the given statement into instructions
pub fn expand(instructions: &mut Vec<Instruction>, stmt: Statement) {
    match stmt {
        Statement::Comment(_) => (),
        Statement::Assignment(lhs, rhs) => {
            match lhs {
                None => output_expr(instructions, rhs),
                Some(_) => unimplemented!(),
            }
        }
    }
}

fn output_expr(instructions: &mut Vec<Instruction>, expr: Expression) {
    match expr {
        Expression::Text(text) => {
            println!("{}", text);
            for ch in text.as_bytes() {
                instructions.push(Right);
                for _ in 0..*ch {
                    instructions.push(Increment);
                }
                instructions.push(Write);
            }
        }
    }
}
