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

fn interact<R: io::BufRead, W: io::Write>(
    input: R,
    output: &mut W,
) -> Result<(), InteractionError> {
    let mut lines = input.lines();
    let plateau: Plateau = lines
        .next()
        .ok_or(InteractionError::MissingPlateau)??
        .parse()?;
    while let Some(rover_line) = lines.next() {
        let mut rover: Rover = rover_line?.parse()?;

        plateau.confirm_within(rover.x, rover.y)?;

        let instructions = lines
            .next()
            .ok_or(InteractionError::MissingInstructions)??;
        for instruction in instructions.chars().map(Instruction::try_from) {
            rover.follow_instruction(instruction?);
            plateau.confirm_within(rover.x, rover.y)?;
        }
        writeln!(output, "{}", rover).map_err(InteractionError::Output)?;
    }
    Ok(())
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    interact(&mut io::stdin().lock(), &mut io::stdout().lock())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rovers_can_follow_instructions() {
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

    #[test]
    fn rover_crashes_when_leaving_the_plateau_north() {
        const INPUT: &[u8] = b"10 5
5 5 N
M
";

        assert!(matches!(
            interact(INPUT, &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                y: 6,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_east() {
        const INPUT: &[u8] = b"5 10
5 5 E
M
";

        assert!(matches!(
            interact(INPUT, &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                x: 6,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_south() {
        const INPUT: &[u8] = b"5 5
5 0 S
M
";

        assert!(matches!(
            interact(INPUT, &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                y: -1,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_leaving_the_plateau_west() {
        const INPUT: &[u8] = b"5 5
0 5 W
M
";

        assert!(matches!(
            interact(INPUT, &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                x: -1,
                ..
            }))
        ));
    }

    #[test]
    fn rover_crashes_when_starting_outside_the_plateau() {
        const INPUT: &[u8] = b"5 5
6 2 W
M
";

        assert!(matches!(
            interact(INPUT, &mut vec![]),
            Err(InteractionError::OutsidePlateau(OutOfPlataeuError {
                x: 6,
                ..
            }))
        ));
    }
}
