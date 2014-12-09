use std::fmt;
use command::Command;
use compass_direction::CompassDirection;
use table::Table;

pub mod command;
pub mod compass_direction;
pub mod table;

#[deriving(Eq, PartialEq)]
pub struct Robot<'t> {
  table: &'t Table,
  x: int,
  y: int,
  heading: CompassDirection,
}

impl<'t> Robot<'t> {
  pub fn new(table: &'t Table) -> Robot<'t> {
    Robot { table: table, x: -1, y: -1, heading: CompassDirection::North }
  }

  pub fn is_placed(&self) -> bool {
    self.x >= 0 && self.y >= 0
  }

  pub fn perform(&mut self, command: Command) {
    if !self.is_placed() && !command.is_place() {
      return
    }
    let (new_x, new_y, new_heading) = match command {
        Command::Place(x, y, heading) => (x, y, heading),
        Command::Move                 => {
          let (new_x, new_y) = match self.heading {
              CompassDirection::North => (self.x, self.y + 1),
              CompassDirection::East  => (self.x + 1, self.y),
              CompassDirection::South => (self.x, self.y - 1),
              CompassDirection::West  => (self.x - 1, self.y),
            };
          (new_x, new_y, self.heading)
        },
        Command::Left                 => {
          let new_heading = match self.heading {
              CompassDirection::North => CompassDirection::West,
              CompassDirection::East  => CompassDirection::North,
              CompassDirection::South => CompassDirection::East,
              CompassDirection::West  => CompassDirection::South,
          };
          (self.x, self.y, new_heading)
        },
        Command::Right                 => {
          let new_heading = match self.heading {
              CompassDirection::North => CompassDirection::East,
              CompassDirection::East  => CompassDirection::South,
              CompassDirection::South => CompassDirection::West,
              CompassDirection::West  => CompassDirection::North,
          };
          (self.x, self.y, new_heading)
        },
        Command::Report               => {
          println!("{}", self);
          (self.x, self.y, self.heading)
        },
      };

    if self.table.check_position(new_x, new_y) {
      self.x = new_x;
      self.y = new_y;
      self.heading = new_heading
    }
  }

  pub fn get_x(&self) -> int {
    self.x
  }
  pub fn get_y(&self) -> int {
    self.y
  }
  pub fn get_heading(&self) -> CompassDirection {
    self.heading
  }
}

impl<'t> fmt::Show for Robot<'t> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{},{},{}", self.x, self.y, self.heading)
  }
}
