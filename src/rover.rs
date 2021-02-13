use crate::CoordinateUnit;
use crate::Direction;
use crate::Instruction;
use std::fmt::Display;
use std::num::ParseIntError;

use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseRoverError {
    #[error("missing x")]
    MissingX,
    #[error("missing y")]
    MissingY,
    #[error("missing direction")]
    MissingDirection,
    #[error("couldn't parse x")]
    UnparsableX(ParseIntError),
    #[error("couldn't parse y")]
    UnparsableY(ParseIntError),
    #[error("couldn't parse direction")]
    UnparsableDirection,
}

impl FromStr for Rover {
    type Err = ParseRoverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut details = s.split_whitespace();
        let x = details
            .next()
            .ok_or(ParseRoverError::MissingX)?
            .parse()
            .map_err(ParseRoverError::UnparsableX)?;
        let y = details
            .next()
            .ok_or(ParseRoverError::MissingY)?
            .parse()
            .map_err(ParseRoverError::UnparsableY)?;
        let direction = match details.next().ok_or(ParseRoverError::MissingDirection)? {
            "N" => Direction::North,
            "E" => Direction::East,
            "S" => Direction::South,
            "W" => Direction::West,
            _ => return Err(ParseRoverError::UnparsableDirection),
        };

        Ok(Rover { x, y, direction })
    }
}

impl Display for Rover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self.direction {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
            Direction::West => 'W',
        };
        write!(f, "{} {} {}", self.x, self.y, direction)
    }
}

#[derive(Debug, PartialEq)]
pub struct Rover {
    x: CoordinateUnit,
    y: CoordinateUnit,
    direction: Direction,
}

impl Rover {
    pub fn new(x: CoordinateUnit, y: CoordinateUnit, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    pub fn follow_instruction(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Left => self.direction = self.direction.rotate_left(),
            Right => self.direction = self.direction.rotate_right(),
            Move => {
                let vector = self.direction.to_vector();
                self.x += vector.0;
                self.y += vector.1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rover::Rover;
    use crate::Direction;

    #[test]
    fn can_be_parsed() {
        assert_eq!("1 2 N".parse(), Ok(Rover::new(1, 2, Direction::North)));
    }
}
