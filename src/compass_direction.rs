use std::str::FromStr;

#[deriving(Eq, PartialEq, Clone, Show)]
pub enum CompassDirection {
  North,
  East,
  South,
  West,
}

impl FromStr for CompassDirection {
  fn from_str(s: &str) -> Option<CompassDirection> {
    match s {
      "NORTH" => Some(CompassDirection::North),
      "EAST"  => Some(CompassDirection::East),
      "SOUTH" => Some(CompassDirection::South),
      "WEST"  => Some(CompassDirection::West),
      _       => None
    }
  }
}
