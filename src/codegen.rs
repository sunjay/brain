use parser::{Statement, Slice, Expression};
use instructions::Instructions;
use memory::MemoryLayout;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    // Illegal redeclaration of a name
    IllegalRedeclaration(String),
    // Name used before it was declared
    UndeclaredIdentifier(String),
    // Declaration contained a size, but it was invalid
    DeclaredIncorrectSize {
        name: String,
        expected: usize,
        actual: usize,
    }
}

/// Expands the given statement into instructions
pub fn expand(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    stmt: Statement
) -> Result<(), Error> {
    match stmt {
        Statement::Comment(_) => Ok(()),
        Statement::Output(expr) => output_expr(instructions, mem, expr),
        Statement::Declaration {name, slice, expr} => declare(instructions, mem, name, slice, expr),
    }
}

fn output_expr(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    expr: Expression
) -> Result<(), Error> {
    match expr {
        Expression::StringLiteral(text) => {
            let cell = mem.next_available_cell();
            let start_cell = mem.current_cell();

            instructions.move_relative(start_cell, cell);
            write_string_literal(instructions, text.as_bytes());
            instructions.move_relative(cell, start_cell);

            Ok(())
        },
        Expression::Identifier(ident) => {
            let (position, size) = mem.get_cell_contents(&ident).ok_or_else(|| {
                Error::UndeclaredIdentifier(ident)
            })?;

            instructions.move_relative(mem.current_cell(), position);
            instructions.write_consecutive(size);
            // This way we end up one cell after the last written one
            instructions.move_right();
            mem.set_current_cell(position + size);

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

fn declare(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
    expr: Expression
) -> Result<(), Error> {
    if mem.is_declared(&name) {
        if slice.is_some() {
            return Err(Error::IllegalRedeclaration(name));
        }

        unimplemented!();
    }

    // Name is not declared
    else {
        let (position, value): (Option<usize>, Vec<u8>) = match expr {
            Expression::StringLiteral(s) => (None, s.into_bytes()),
            Expression::Identifier(value_name) => unimplemented!(),
        };

        if slice.is_none() && position.is_none() {
            return Err(Error::UndeclaredIdentifier(name));
        }

        let slice = slice.unwrap();
        let size = match slice {
            Slice::SingleValue(s) => s,
            Slice::Unspecified => value.len(),
        };

        if size != value.len() {
            return Err(Error::DeclaredIncorrectSize {
                name: name,
                expected: value.len(),
                actual: size,
            });
        }

        if position.is_none() {
            let position = mem.declare(&name, size);
            instructions.move_relative(mem.current_cell(), position);
            instructions.store_bytes(value.as_slice());
            mem.set_current_cell(position + value.len());
            Ok(())
        }
        else {
            unimplemented!();
        }
    }
}
