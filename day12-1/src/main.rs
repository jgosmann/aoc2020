use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::num::ParseIntError;

#[derive(Copy, Clone, Debug, PartialEq)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    fn as_cartesian_vector(&self, scale: i64) -> (i64, i64) {
        match self {
            Self::North => (0, scale),
            Self::South => (0, -scale),
            Self::East => (scale, 0),
            Self::West => (-scale, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RotationDirection {
    Left,
    Right,
}

impl RotationDirection {
    fn rotate90(&self, heading: CardinalDirection) -> CardinalDirection {
        match (self, heading) {
            (Self::Left, CardinalDirection::North) => CardinalDirection::West,
            (Self::Left, CardinalDirection::South) => CardinalDirection::East,
            (Self::Left, CardinalDirection::East) => CardinalDirection::North,
            (Self::Left, CardinalDirection::West) => CardinalDirection::South,
            (Self::Right, CardinalDirection::North) => CardinalDirection::East,
            (Self::Right, CardinalDirection::South) => CardinalDirection::West,
            (Self::Right, CardinalDirection::East) => CardinalDirection::South,
            (Self::Right, CardinalDirection::West) => CardinalDirection::North,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Action {
    Move(CardinalDirection, u32),
    MoveForward(u32),
    Turn(RotationDirection, u32),
}

impl TryFrom<&str> for Action {
    type Error = ActionParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (action, value) = input.split_at(1);
        let value = value
            .parse()
            .or_else(|err| Err(ActionParseError::InvalidValue(err)))?;
        match action {
            "N" => Ok(Action::Move(CardinalDirection::North, value)),
            "S" => Ok(Action::Move(CardinalDirection::South, value)),
            "E" => Ok(Action::Move(CardinalDirection::East, value)),
            "W" => Ok(Action::Move(CardinalDirection::West, value)),
            "L" => Ok(Action::Turn(RotationDirection::Left, value)),
            "R" => Ok(Action::Turn(RotationDirection::Right, value)),
            "F" => Ok(Action::MoveForward(value)),
            action => Err(ActionParseError::InvalidAction(String::from(action))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ActionParseError {
    InvalidAction(String),
    InvalidValue(ParseIntError),
}

struct Ship {
    position: (i64, i64),
    heading: CardinalDirection,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: (0, 0),
            heading: CardinalDirection::East,
        }
    }

    fn excute_action(&mut self, action: Action) {
        match action {
            Action::Move(direction, distance) => {
                let move_vector = direction.as_cartesian_vector(distance.into());
                self.position = (
                    self.position.0 + move_vector.0,
                    self.position.1 + move_vector.1,
                );
            }
            Action::MoveForward(distance) => {
                let move_vector = self.heading.as_cartesian_vector(distance.into());
                self.position = (
                    self.position.0 + move_vector.0,
                    self.position.1 + move_vector.1,
                );
            }
            Action::Turn(rotation, amount) => {
                if amount % 90 != 0 {
                    panic!("Unsupported rotation.");
                }
                let times = (amount / 90) % 4;
                for _ in 0..times {
                    self.heading = rotation.rotate90(self.heading);
                }
            }
        }
    }

    fn manhatten_dist(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs()
    }
}

fn main() {
    let mut ship = Ship::new();
    io::stdin().lock().lines().for_each(|line| {
        let action = line.unwrap();
        ship.excute_action(Action::try_from(action.as_ref()).unwrap());
    });
    println!("Manhatten distance: {}", ship.manhatten_dist());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(
        input,
        expected,
        case("N10", Ok(Action::Move(CardinalDirection::North, 10))),
        case("S11", Ok(Action::Move(CardinalDirection::South, 11))),
        case("E12", Ok(Action::Move(CardinalDirection::East, 12))),
        case("W13", Ok(Action::Move(CardinalDirection::West, 13))),
        case("L20", Ok(Action::Turn(RotationDirection::Left, 20))),
        case("R21", Ok(Action::Turn(RotationDirection::Right, 21))),
        case("F30", Ok(Action::MoveForward(30))),
        case("X40", Err(ActionParseError::InvalidAction(String::from("X")))),
        case("FXX", Err(ActionParseError::InvalidValue("XX".parse::<u32>().unwrap_err())))
    )]
    fn test_action_parsing(input: &str, expected: Result<Action, ActionParseError>) {
        assert_eq!(Action::try_from(input), expected);
    }

    #[test]
    fn test_ship_navigation() {
        let actions = vec!["F10", "N3", "F7", "R90", "F11"];
        let mut ship = Ship::new();
        actions.iter().for_each(|&action| {
            ship.excute_action(Action::try_from(action).unwrap());
        });
        assert_eq!(ship.manhatten_dist(), 25);
    }
}
