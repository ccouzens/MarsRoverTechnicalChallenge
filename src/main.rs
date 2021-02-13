type CoordinateUnit = i16;

enum Command {
    Left,
    Right,
    Move,
}

#[derive(Debug, PartialEq)]
enum Direction {
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

#[derive(Debug, PartialEq)]
struct Rover {
    x: CoordinateUnit,
    y: CoordinateUnit,
    direction: Direction,
}

impl Rover {
    fn new(x: CoordinateUnit, y: CoordinateUnit, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn follow_command(&mut self, command: Command) {
        use Command::*;
        match command {
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

fn main() {
    println!("Hello, world!");
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
