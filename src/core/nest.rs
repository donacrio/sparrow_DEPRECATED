use super::egg::Egg;
use super::errors;
use std::collections::HashMap;

pub struct Nest {
  map: HashMap<String, Egg>,
}

impl Nest {
  pub fn new() -> Nest {
    Nest {
      map: HashMap::new(),
    }
  }
}

impl Nest {
  pub fn insert(&mut self, egg: Egg) -> Option<Egg> {
    self.map.insert(egg.key().clone(), egg)
  }
  pub fn get(&self, key: &str) -> Result<&Egg, errors::EggNotInNestError> {
    self.map.get(key).ok_or(errors::EggNotInNestError::new(key))
  }
  pub fn pop(&mut self, key: &str) -> Result<Egg, errors::EggNotInNestError> {
    self
      .map
      .remove(key)
      .ok_or(errors::EggNotInNestError::new(key))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  #[fixture]
  fn nest() -> Nest {
    Nest::new()
  }

  #[fixture]
  fn egg() -> Egg {
    let egg_key = "test";
    let egg_value = "This is a test value!";
    Egg::new(egg_key, egg_value)
  }

  #[test]
  fn test_nest_new() {
    Nest::new();
  }

  #[rstest]
  fn test_nest_insert(mut nest: Nest, egg: Egg) {
    // Egg is not in nest
    assert_eq!(nest.insert(egg.clone()), None);
    // Egg is in nest
    assert_eq!(nest.insert(egg.clone()), Some(egg));
  }

  #[rstest]
  fn test_nest_get(mut nest: Nest, egg: Egg) {
    // Egg is not in nest
    assert_eq!(nest.insert(egg.clone()), None);
    assert_eq!(nest.get(egg.key()), Ok(&egg))
  }

  #[rstest]
  fn test_nest_pop(mut nest: Nest, egg: Egg) {
    // Egg is not in nest
    assert_eq!(nest.insert(egg.clone()), None);
    assert_eq!(nest.pop(egg.key()), Ok(egg.clone()));
    assert_eq!(
      nest.pop(egg.key()),
      Err(errors::EggNotInNestError::new(egg.key()))
    );
  }
}
