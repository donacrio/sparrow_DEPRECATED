use std::fmt;

#[derive(Debug, PartialEq)]
pub struct EggNotInNestError {
  key: String,
}

impl EggNotInNestError {
  pub fn new(key: &str) -> EggNotInNestError {
    EggNotInNestError {
      key: key.to_string(),
    }
  }
}

impl fmt::Display for EggNotInNestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "No egg with key \"{}\" was found in the nest", self.key)
  }
}
