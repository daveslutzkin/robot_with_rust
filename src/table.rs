#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Table {
  width: int,
  height: int,
}

impl Table {
  pub fn new(width: int, height: int) -> Table {
    Table { width: width, height: height }
  }

  pub fn check_position(&self, x: int, y: int) -> bool {
    x >= 0 && y >= 0 && x < self.width && y < self.height
  }
}
