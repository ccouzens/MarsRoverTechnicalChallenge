use crate::CoordinateUnit;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[error("Unrecognized direction")]
pub struct ParseDirectionError;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn to_vector(&self) -> (CoordinateUnit, CoordinateUnit) {
        use Direction::*;
        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }

    pub fn rotate_left(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn rotate_right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Direction::North),
            "E" => Ok(Direction::East),
            "S" => Ok(Direction::South),
            "W" => Ok(Direction::West),
            _ => Err(ParseDirectionError),
        }
    }
}

impl From<&Direction> for char {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
            Direction::West => 'W',
        }
    }
}
