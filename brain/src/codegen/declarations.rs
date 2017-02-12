use parser::{Slice, Expression};
use instructions::Instructions;
use memory::MemoryLayout;

use super::errors::Error;
use super::cells::copy_cells;

pub fn declare(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
    expr: Expression
) -> Result<(), Error> {
    if mem.is_declared(&name) {
        assign_previously_declarated(instructions, mem, name, slice, expr)
    }
    else {
        declare_undeclared(instructions, mem, name, slice, expr)
    }
}

/// Assigns a new value to a previously declared identifier
fn assign_previously_declarated(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
    expr: Expression
) -> Result<(), Error> {
    if slice.is_some() {
        return Err(Error::IllegalRedeclaration {name: name});
    }

    let (position, size) = mem.get_cell_contents(&name).unwrap();

    match expr {
        Expression::StringLiteral(value) => {
            if value.len() != size {
                return Err(Error::IncorrectSizedExpression {
                    name: name,
                    expected: size,
                    actual: value.len(),
                });
            }

            // Zero the cells first, then write the new value
            instructions.move_right_by(position);
            instructions.zero_cells(size);
            instructions.store_bytes(value.as_bytes());
            instructions.move_left_by(position);
            Ok(())
        },
        Expression::Identifier(value_name) => {
            if name == value_name {
                return Err(Error::SelfAssignment {
                    name: name,
                });
            }

            let (source_position, source_size) = mem.get_cell_contents(&value_name).ok_or_else(|| Error::UndeclaredIdentifier {name: value_name})?;

            if size != source_size {
                return Err(Error::IncorrectSizedExpression {
                    name: name,
                    expected: size,
                    actual: source_size,
                });
            }

            // Zero the cells first, then write the new value
            instructions.move_right_by(position);
            instructions.zero_cells(size);
            instructions.move_left_by(position);

            copy_cells(instructions, mem, source_position, position, size);
            Ok(())
        },
    }
}

/// Declares a new identifier that was previously undeclared
fn declare_undeclared(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
    expr: Expression
) -> Result<(), Error> {
    if slice.is_none() {
        return Err(Error::UndeclaredIdentifier {name: name});
    }
    let slice = slice.unwrap();

    match expr {
        Expression::StringLiteral(value) => {
            let size = match slice {
                Slice::SingleValue(s) => s,
                Slice::Unspecified => value.len(),
            };

            if size == 0 {
                return Err(Error::DeclaredZeroSize {
                    name: name,
                });
            }
            else if size != value.len() {
                return Err(Error::DeclaredIncorrectSize {
                    name: name,
                    expected: value.len(),
                    actual: size,
                });
            }

            let position = mem.declare(&name, size);
            instructions.move_right_by(position);
            instructions.store_bytes(value.as_bytes());
            instructions.move_left_by(position);
            Ok(())
        },
        Expression::Identifier(value_name) => {
            let (source_position, source_size) = mem.get_cell_contents(&value_name).ok_or_else(|| Error::UndeclaredIdentifier {name: value_name})?;

            let size = match slice {
                Slice::SingleValue(s) => s,
                Slice::Unspecified => source_size,
            };

            if size != source_size {
                return Err(Error::DeclaredIncorrectSize {
                    name: name,
                    expected: source_size,
                    actual: size,
                });
            }

            let position = mem.declare(&name, size);
            copy_cells(instructions, mem, source_position, position, size);
            Ok(())
        }
    }
}
