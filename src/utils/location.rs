use std::fmt::{Display, Error, Formatter};

pub struct Position {
  pub line: usize,
  pub column: usize,
}

impl Position {
  pub fn new(line: usize, column: usize) -> Self {
    Self { line, column }
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}:{}", self.line, self.column)
  }
}

pub struct SourceLocation {
  pub start: Position,
  pub end: Position,
}

impl SourceLocation {
  pub fn new(start: Position, end: Position) -> Self {
    Self { start, end }
  }
}

impl Display for SourceLocation {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{},{}", self.start, self.end)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_position() {
    let pos = Position::new(1, 2);
    assert_eq!(pos.to_string(), "1:2");
  }

  #[test]
  fn test_source_location() {
    let start = Position::new(1, 2);
    let end = Position::new(3, 4);
    let loc = SourceLocation::new(start, end);
    assert_eq!(loc.to_string(), "1:2,3:4");
  }
}
