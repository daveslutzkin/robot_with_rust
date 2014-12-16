use std::str::FromStr;
use compass_direction::CompassDirection;

#[deriving(Eq, PartialEq, Show)]
pub enum Command {
  Place(int, int, CompassDirection),
  Move,
  Left,
  Right,
  Report,
}

impl Command {
  pub fn is_place(&self) -> bool {
    match *self {
      Command::Place(_, _, _) => true,
      _                       => false,
    }
  }
}

impl Copy for Command {}

impl FromStr for Command {
  fn from_str(s: &str) -> Option<Command> {
    let parts = s.trim().split(' ').collect::<Vec<&str>>();
    match parts[0] {
      "PLACE"   => {
        let place_parts = parts[1].split(',').collect::<Vec<&str>>();
        let x_result: Option<int> = from_str(place_parts[0]);
        let y_result: Option<int> = from_str(place_parts[1]);
        let dir_result: Option<CompassDirection> = from_str(place_parts[2]);
        match (x_result, y_result, dir_result) {
          (Some(x), Some(y), Some(dir)) => Some(Command::Place(x, y, dir)),
          _                             => None
        }
      },
      "MOVE"    => Some(Command::Move),
      "LEFT"    => Some(Command::Left),
      "RIGHT"   => Some(Command::Right),
      "REPORT"  => Some(Command::Report),
      _         => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Command;
  use compass_direction::CompassDirection;

  fn test_command(raw_command: &str, expected_command: Command) {
    match from_str(raw_command) {
      Some(command) if command == expected_command
        => assert!(true),
      Some(Command::Place(x,y,d))
        => assert!(false, "{} became an unexpected place {},{},{}", raw_command, x, y, d),
      Some(unexpected_command)
        => assert!(false, "{} became an unexpected return {}", raw_command, unexpected_command),
      None
        => assert!(false, "No command read"),
    }
  }

  #[test]
  fn command_place() {
    test_command("PLACE 3,4,WEST", Command::Place(3, 4, CompassDirection::West))
  }

  #[test]
  fn command_move() {
    test_command("MOVE", Command::Move)
  }

  #[test]
  fn command_left() {
    test_command("LEFT", Command::Left)
  }

  #[test]
  fn command_right() {
    test_command("RIGHT", Command::Right)
  }

  #[test]
  fn command_report() {
    test_command("REPORT", Command::Report)
  }

  #[test]
  fn command_report_with_whitespace() {
    test_command("REPORT\n\r", Command::Report)
  }
}
