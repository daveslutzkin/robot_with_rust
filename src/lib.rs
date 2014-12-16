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
          (new_x, new_y, self.heading.clone())
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
          (self.x, self.y, self.heading.clone())
        },
      };

    if self.table.check_position(new_x, new_y) {
      self.x = new_x;
      self.y = new_y;
      self.heading = new_heading
    }
  }
}

impl<'t> fmt::Show for Robot<'t> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{},{},{}", self.x, self.y, self.heading)
  }
}

#[cfg(test)]
mod tests {
  use super::Robot;
  use table::Table;
  use command::Command;
  use compass_direction::CompassDirection;

  #[test]
  fn new_robot_is_not_placed() {
    let table = Table::new(10, 8);
    let robot = Robot::new(&table);
    assert_eq!(robot.is_placed(), false);
  }

  #[test]
  fn command_before_place_does_nothing() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Move);
    assert_eq!(robot.x, -1);
    assert_eq!(robot.y, -1);
    assert_eq!(robot.heading, CompassDirection::North);
  }

  #[test]
  fn place_command_places_robot() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(3, 6, CompassDirection::West));
    assert_eq!(robot.is_placed(), true);
  }

  #[test]
  fn place_command_places_robot_correctly() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(3, 6, CompassDirection::West));
    assert_eq!(robot.x, 3);
    assert_eq!(robot.y, 6);
    assert_eq!(robot.heading, CompassDirection::West)
  }

  #[test]
  fn move_command_moves_robot_correctly() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(3, 6, CompassDirection::West));
    robot.perform(Command::Move);
    assert_eq!(robot.x, 2);
    assert_eq!(robot.y, 6);
    assert_eq!(robot.heading, CompassDirection::West)
  }

  #[test]
  fn left_command_turns_robot_correctly() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(3, 6, CompassDirection::West));
    robot.perform(Command::Left);
    assert_eq!(robot.x, 3);
    assert_eq!(robot.y, 6);
    assert_eq!(robot.heading, CompassDirection::South)
  }

  #[test]
  fn right_command_turns_robot_correctly() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(3, 6, CompassDirection::West));
    robot.perform(Command::Right);
    assert_eq!(robot.x, 3);
    assert_eq!(robot.y, 6);
    assert_eq!(robot.heading, CompassDirection::North)
  }

  #[test]
  fn report_command_outputs_position() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(3, 6, CompassDirection::West));
    robot.perform(Command::Report);
    // What assertion?
  }

  #[test]
  fn move_off_the_left_edge_is_stopped() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(0, 5, CompassDirection::West));
    robot.perform(Command::Move);
    assert_eq!(robot.x, 0);
    assert_eq!(robot.y, 5);
  }

  #[test]
  fn move_off_the_bottom_edge_is_stopped() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(5, 0, CompassDirection::South));
    robot.perform(Command::Move);
    assert_eq!(robot.x, 5);
    assert_eq!(robot.y, 0);
  }

  #[test]
  fn move_off_the_right_edge_is_stopped() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(9, 0, CompassDirection::East));
    robot.perform(Command::Move);
    assert_eq!(robot.x, 9);
    assert_eq!(robot.y, 0);
  }

  #[test]
  fn move_off_the_top_edge_is_stopped() {
    let table = Table::new(10, 8);
    let mut robot = Robot::new(&table);
    robot.perform(Command::Place(8, 7, CompassDirection::North));
    robot.perform(Command::Move);
    assert_eq!(robot.x, 8);
    assert_eq!(robot.y, 7);
  }
}
