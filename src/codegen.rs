use parser::{Statement, Slice, Expression};
use instructions::Instructions;
use memory::MemoryLayout;

/// Expands the given statement into instructions
pub fn expand(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    stmt: Statement
) {
    match stmt {
        Statement::Comment(_) => (),
        Statement::Output(expr) => output_expr(instructions, mem, expr),
        Statement::Declaration {name, slice, expr} => declare(instructions, mem, name, slice, expr),
    }
}

fn output_expr(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    expr: Expression
) {
    match expr {
        Expression::StringLiteral(text) => {
            let cell = mem.next_available_cell();
            instructions.move_relative(mem.current_cell(), cell);

            let mut last_char: u8 = 0;
            for ch in text.as_bytes() {
                let ch = *ch;
                instructions.increment_relative(last_char, ch);
                instructions.write();

                last_char = ch;
            }

            // always reset this cell because we don't need it anymore
            instructions.increment_relative(last_char, 0);
        },
        Expression::Identifier(ident) => println!("{:?}", ident), //TODO
    }
}

fn declare(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
    expr: Expression
) {
}
