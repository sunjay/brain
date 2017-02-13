use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    // ">" - increment the pointer (move it to the "right")
    Right,
    // "<" - decrement the pointer (move it to the "left")
    Left,
    // "+" - increment the byte at the pointer
    Increment,
    // "-" - decrement the byte at the pointer
    Decrement,
    // "." - output the byte at the pointer
    Write,
    // "," - input a byte and store it in the byte at the pointer
    Read,
    // "[" - jump forward past the matching ] if the byte at the pointer is zero
    JumpForwardIfZero,
    // "]" - jump backward to the matching [ unless the byte at the pointer is zero
    JumpBackwardUnlessZero,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Instruction::Right => ">",
            Instruction::Left => "<",
            Instruction::Increment => "+",
            Instruction::Decrement => "-",
            Instruction::Write => ".",
            Instruction::Read => ",",
            Instruction::JumpForwardIfZero => "[",
            Instruction::JumpBackwardUnlessZero => "]",
        })
    }
}
