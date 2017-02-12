use parser::{Expression};
use instructions::Instructions;
use memory::MemoryLayout;

use super::errors::Error;

pub fn output_expr(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    expr: Expression
) -> Result<(), Error> {
    match expr {
        Expression::StringLiteral(text) => {
            let cell = mem.next_available_cell();

            instructions.move_right_by(cell);
            write_string_literal(instructions, text.as_bytes());
            instructions.move_left_by(cell);

            Ok(())
        },
        Expression::Identifier(ident) => {
            let (position, size) = mem.get_cell_contents(&ident).ok_or_else(|| {
                Error::UndeclaredIdentifier {name: ident}
            })?;

            instructions.move_right_by(position);
            instructions.write_consecutive(size);
            instructions.move_left_by(position);

            Ok(())
        },
    }
}

fn write_string_literal(instructions: &mut Instructions, bytes: &[u8]) {
    // Writing string literals are special because you don't necessarily
    // need to store the string literal in any location outside of what is necessary
    // for the write. The memory is to be allocated briefly, then freed.
    // Because of this, we don't split allocation and writing into separate steps.
    // We keep this special routine specifically designed to write string literals

    let mut last_char: u8 = 0;
    for ch in bytes {
        let ch = *ch;
        instructions.increment_relative(last_char, ch);
        instructions.write();

        last_char = ch;
    }

    // always reset this cell because we don't need it anymore
    instructions.increment_relative(last_char, 0);
}
