type CoordinateUnit = i16;

mod instruction;
pub mod plateau;
pub mod rover;
pub use instruction::{Instruction, ParseInstructionError};
pub use rover::Rover;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_vector(&self) -> (CoordinateUnit, CoordinateUnit) {
        use Direction::*;
        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }

    fn rotate_left(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn rotate_right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rover_a_can_follow_instuctions() {
        let mut rover = Rover::new(1, 2, Direction::North);
        rover.follow_instruction(Instruction::Left);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Left);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Left);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Left);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Move);

        assert_eq!(rover, Rover::new(1, 3, Direction::North));
    }

    #[test]
    fn rover_b_can_follow_instructions() {
        let mut rover = Rover::new(3, 3, Direction::East);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Right);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Right);
        rover.follow_instruction(Instruction::Move);
        rover.follow_instruction(Instruction::Right);
        rover.follow_instruction(Instruction::Right);
        rover.follow_instruction(Instruction::Move);

        assert_eq!(rover, Rover::new(5, 1, Direction::East));
    }
}
