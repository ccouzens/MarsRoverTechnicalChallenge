type CoordinateUnit = i16;

pub mod plateau;
pub mod rover;
pub use rover::Rover;

pub enum Command {
    Left,
    Right,
    Move,
}

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
    fn rover_a_can_follow_commands() {
        let mut rover = Rover::new(1, 2, Direction::North);
        rover.follow_command(Command::Left);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Left);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Left);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Left);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Move);

        assert_eq!(rover, Rover::new(1, 3, Direction::North));
    }

    #[test]
    fn rover_b_can_follow_commands() {
        let mut rover = Rover::new(3, 3, Direction::East);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Right);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Right);
        rover.follow_command(Command::Move);
        rover.follow_command(Command::Right);
        rover.follow_command(Command::Right);
        rover.follow_command(Command::Move);

        assert_eq!(rover, Rover::new(5, 1, Direction::East));
    }
}
