use std::convert::TryFrom;
use thiserror::Error;

pub enum Instruction {
    Left,
    Right,
    Move,
}

#[derive(Error, Debug, PartialEq)]
#[error("Unrecognized instruction {0}")]
pub struct ParseInstructionError(char);

impl TryFrom<char> for Instruction {
    type Error = ParseInstructionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            'M' => Ok(Instruction::Move),
            o => Err(ParseInstructionError(o)),
        }
    }
}
