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
