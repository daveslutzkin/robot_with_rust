extern crate robot_with_rust;
#[cfg(not(test))]
use std::io;
#[cfg(not(test))]
use robot_with_rust::Robot;
#[cfg(not(test))]
use robot_with_rust::table::Table;

#[cfg(not(test))]
fn main() {
  println!("Rusty Robot");

  let table = Table::new(5, 5);
  let mut robot = Robot::new(&table);

  loop {
    let line = match io::stdin().read_line() {
        Ok(line)  => line,
        Err(_)    => break
      };

    match from_str(line.as_slice()) {
      Some(command) => robot.perform(command),
      None          => println!("No command")
    }
  }
}
