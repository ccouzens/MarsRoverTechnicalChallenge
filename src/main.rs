use mars_rover_technical_challenge::plateau::{ParsePlateauError, Plateau};
use mars_rover_technical_challenge::rover::ParseRoverError;
use mars_rover_technical_challenge::{Command, Rover};
use std::io::{self};
use std::io::{BufRead, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InteractionError {
    #[error("Failed to read input")]
    Input(#[from] io::Error),
    #[error("Failed to write output")]
    Output(io::Error),
    #[error("Expected plateau's dimensions")]
    MissingPlateau,
    #[error("Failed to parse plateau's dimensions")]
    PlateauInvalid(#[from] ParsePlateauError),
    #[error("Failed to parse rover's details")]
    RoverInvalid(#[from] ParseRoverError),
    #[error("Expected instructions for rover")]
    MissingInstructions,
    #[error("Invalid Instruction {0}")]
    InvalidInstruction(char),
}

fn interact<R: BufRead, W: Write>(input: R, output: &mut W) -> Result<(), InteractionError> {
    let mut lines = input.lines();
    let plateau: Plateau = lines
        .next()
        .ok_or(InteractionError::MissingPlateau)??
        .parse()?;
    while let Some(rover_line) = lines.next() {
        let mut rover: Rover = rover_line?.parse()?;
        let instructions = lines
            .next()
            .ok_or(InteractionError::MissingInstructions)??;
        for instruction in instructions.chars() {
            let command = match instruction {
                'L' => Command::Left,
                'R' => Command::Right,
                'M' => Command::Move,
                o => return Err(InteractionError::InvalidInstruction(o)),
            };
            rover.follow_command(command);
        }
        writeln!(output, "{}", rover).map_err(InteractionError::Output)?;
    }
    Ok(())
}

fn main() -> Result<(), InteractionError> {
    interact(&mut io::stdin().lock(), &mut io::stdout().lock())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rovers_can_follow_commands() {
        const INPUT: &[u8] = b"5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM
";

        let mut output = vec![];
        interact(INPUT, &mut output).unwrap();
        assert_eq!(
            String::from_utf8(output),
            Ok(String::from(
                "1 3 N
5 1 E
"
            ))
        );
    }
}
