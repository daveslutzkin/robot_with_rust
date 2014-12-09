extern crate robot_with_rust;
use robot_with_rust::Robot;
use robot_with_rust::table::Table;
use robot_with_rust::command::Command;
use robot_with_rust::compass_direction::CompassDirection;

fn test_command(raw_command: &str, expected_command: Command) {
  match from_str(raw_command) {
    Some(command) if command == expected_command  => assert!(true),
    Some(Command::Place(x,y,d))                   => assert!(false, "{} became an unexpected place {},{},{}", raw_command, x, y, d),
    Some(unexpected_command)                      => assert!(false, "{} became an unexpected return {}", raw_command, unexpected_command),
    None                                          => assert!(false, "No command read"),
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

#[test]
fn new_robot_is_not_placed() {
  let table = Table::new(10, 8);
  let robot = Robot::new(&table);
  assert!(!robot.is_placed());
}

#[test]
fn command_before_place_does_nothing() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Move);
  assert!(robot.get_x() == -1);
  assert!(robot.get_y() == -1);
  assert!(robot.get_heading() == CompassDirection::North);
}

#[test]
fn place_command_places_robot() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(3, 6, CompassDirection::West));
  assert!(robot.is_placed());
}

#[test]
fn place_command_places_robot_correctly() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(3, 6, CompassDirection::West));
  assert!(robot.get_x() == 3, "Robot x was not 3 ({})", robot);
  assert!(robot.get_y() == 6, "Robot y was not 6 ({})", robot);
  assert!(robot.get_heading() == CompassDirection::West, "Robot heading was not West ({})", robot)
}

#[test]
fn move_command_moves_robot_correctly() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(3, 6, CompassDirection::West));
  robot.perform(Command::Move);
  assert!(robot.get_x() == 2, "Robot x was not 2 ({})", robot);
  assert!(robot.get_y() == 6, "Robot y was not 6 ({})", robot);
  assert!(robot.get_heading() == CompassDirection::West, "Robot heading was not West ({})", robot)
}

#[test]
fn left_command_turns_robot_correctly() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(3, 6, CompassDirection::West));
  robot.perform(Command::Left);
  assert!(robot.get_x() == 3, "Robot x was not 3 ({})", robot);
  assert!(robot.get_y() == 6, "Robot y was not 6 ({})", robot);
  assert!(robot.get_heading() == CompassDirection::South, "Robot heading was not South ({})", robot)
}

#[test]
fn right_command_turns_robot_correctly() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(3, 6, CompassDirection::West));
  robot.perform(Command::Right);
  assert!(robot.get_x() == 3, "Robot x was not 3 ({})", robot);
  assert!(robot.get_y() == 6, "Robot y was not 6 ({})", robot);
  assert!(robot.get_heading() == CompassDirection::North, "Robot heading was not North ({})", robot)
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
  assert!(robot.get_x() == 0, "Robot moved to {}", robot);
  assert!(robot.get_y() == 5, "Robot moved to {}", robot);
}

#[test]
fn move_off_the_bottom_edge_is_stopped() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(5, 0, CompassDirection::South));
  robot.perform(Command::Move);
  assert!(robot.get_x() == 5, "Robot moved to {}", robot);
  assert!(robot.get_y() == 0, "Robot moved to {}", robot);
}

#[test]
fn move_off_the_right_edge_is_stopped() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(9, 0, CompassDirection::East));
  robot.perform(Command::Move);
  assert!(robot.get_x() == 9, "Robot moved to {}", robot);
  assert!(robot.get_y() == 0, "Robot moved to {}", robot);
}

#[test]
fn move_off_the_top_edge_is_stopped() {
  let table = Table::new(10, 8);
  let mut robot = Robot::new(&table);
  robot.perform(Command::Place(9, 7, CompassDirection::North));
  robot.perform(Command::Move);
  assert!(robot.get_x() == 9, "Robot moved to {}", robot);
  assert!(robot.get_y() == 7, "Robot moved to {}", robot);
}
