use parser::{Slice};
use instructions::Instructions;
use memory::MemoryLayout;

use super::errors::Error;

pub fn read_input(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
) -> Result<(), Error> {
    if mem.is_declared(&name) {
        read_into_existing_name(instructions, mem, name, slice)
    }
    else {
        read_into_new_name(instructions, mem, name, slice)
    }
}

fn read_into_existing_name(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
) -> Result<(), Error> {
    if slice.is_some() {
        return Err(Error::IllegalRedeclaration {name: name});
    }

    let (position, size) = mem.get_cell_contents(&name).unwrap();

    instructions.move_right_by(position);
    instructions.read_consecutive(size);
    instructions.move_left_by(position);
    Ok(())
}

fn read_into_new_name(
    instructions: &mut Instructions,
    mem: &mut MemoryLayout,
    name: String,
    slice: Option<Slice>,
) -> Result<(), Error> {
    if slice.is_none() {
        return Err(Error::UndeclaredIdentifier {name: name});
    }
    let slice = slice.unwrap();

    let size = match slice {
        Slice::SingleValue(s) => s,
        Slice::Unspecified => {
            return Err(Error::UnspecifiedInputSizeUnsupported {name: name});
        },
    };

    if size == 0 {
        return Err(Error::DeclaredZeroSize {
            name: name,
        });
    }

    let position = mem.declare(&name, size);
    instructions.move_right_by(position);
    instructions.read_consecutive(size);
    instructions.move_left_by(position);
    Ok(())
}
