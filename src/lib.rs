type CoordinateUnit = i16;

mod direction;
mod instruction;
mod plateau;
mod rover;
pub use direction::{Direction, ParseDirectionError};
pub use instruction::{Instruction, ParseInstructionError};
pub use plateau::{OutOfPlataeuError, ParsePlateauError, Plateau};
pub use rover::{ParseRoverError, Rover};

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
