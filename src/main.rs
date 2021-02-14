use mars_rover_technical_challenge::{
    Instruction, OutOfPlataeuError, ParseInstructionError, ParsePlateauError, ParseRoverError,
    Plateau, Rover,
};
use std::convert::TryFrom;
use std::io;
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
    #[error("Invalid instruction")]
    InvalidInstruction(#[from] ParseInstructionError),
    #[error("Rover is outside plateau")]
    OutsidePlateau(#[from] OutOfPlataeuError),
}

fn interact(input: impl io::BufRead, mut output: impl io::Write) -> Result<(), InteractionError> {
    let mut input_lines = input.lines();
    let plateau: Plateau = input_lines
        .next()
        .ok_or(InteractionError::MissingPlateau)??
        .parse()?;

    while let Some(rover_line) = input_lines.next() {
        let mut rover: Rover = rover_line?.parse()?;

        plateau.confirm_within(rover.x, rover.y)?;

        let instructions_line = input_lines
            .next()
            .ok_or(InteractionError::MissingInstructions)??;
        for instruction_char in instructions_line.chars() {
            let instruction = Instruction::try_from(instruction_char)?;
            rover.follow_instruction(instruction);
            plateau.confirm_within(rover.x, rover.y)?;
        }
        writeln!(output, "{}", rover).map_err(InteractionError::Output)?;
    }
    Ok(())
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    interact(io::stdin().lock(), io::stdout())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rovers_can_follow_instructions() {
        const INPUT: &[u8] = b"5 5\n1 2 N\nLMLMLMLMM\n3 3 E\nMMRMMRMRRM\n";

        let mut output = vec![];
        interact(INPUT, &mut output).unwrap();
        assert_eq!(
            String::from_utf8(output),
            Ok(String::from("1 3 N\n5 1 E\n"))
        );
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_north() {
        assert!(matches!(
            interact(b"10 5\n5 5 N\nM" as &[u8], &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                y: 6,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_east() {
        assert!(matches!(
            interact(b"5 10\n5 5 E\nM" as &[u8], &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                x: 6,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_south() {
        assert!(matches!(
            interact(b"5 5\n5 0 S\nM" as &[u8], &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                y: -1,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_west() {
        assert!(matches!(
            interact(b"5 5\n0 5 W\nM" as &[u8], &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                x: -1,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_starting_outside_the_plateau() {
        assert!(matches!(
            interact(b"5 5\n6 2 W\nM" as &[u8], &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                x: 6,
                ..
            }))
        ));
    }
}
