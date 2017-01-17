// As specified here: http://www.muppetlabs.com/~breadbox/bf/

use std::convert::Into;

use instruction::Instruction;
use parser::Program;
use codegen;

#[derive(Debug, PartialEq, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn from_program(program: Program) -> Result<Self, ()> {
        let mut instrs = Vec::new();

        for stmt in program {
            codegen::expand(&mut instrs, stmt);
        }

        Ok(Instructions(instrs))
    }
}

impl Into<String> for Instructions {
    fn into(self) -> String {
        self.into_iter().fold(String::new(), |acc, instr| acc + &instr.to_string())
    }
}

impl IntoIterator for Instructions {
    type Item = Instruction;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
