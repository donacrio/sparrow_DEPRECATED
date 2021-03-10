//! In-memory data storage.

use super::egg::Egg;
use std::collections::HashMap;

/// Nest is the in-memory data storage of Sparrow.
///
/// It contains an [`HashMap`] to store multiple [`Egg`] along with their key.
///
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`Egg`]: sparrow::core::egg::Egg
pub struct Nest {
  map: HashMap<String, Egg>,
}

impl Nest {
  /// Return a new [`Nest`].
  ///
  /// [`Nest`]: sparrow::core::nest::Nest
  pub fn new() -> Nest {
    Nest {
      map: HashMap::new(),
    }
  }
}

impl Default for Nest {
  fn default() -> Self {
    Self::new()
  }
}

impl Nest {
  /// Insert an [`Egg`] into the `map` field
  ///
  /// # Arguments
  /// * `egg` - [`Egg`] to insert
  ///
  /// [`Egg`]: sparrow::core::egg::Egg
  pub fn insert(&mut self, egg: Egg) -> Option<Egg> {
    self.map.insert(egg.key().clone(), egg)
  }
  /// Get an [`Egg`] from the `map` field
  ///
  /// # Arguments
  /// * `key` - Key value of the [`Egg`] to get
  ///
  /// [`Egg`]: sparrow::core::egg::Egg
  pub fn get(&self, key: &str) -> Option<&Egg> {
    self.map.get(key)
  }
  /// Pop an [`Egg`] from the `map` field
  ///
  /// # Arguments
  /// * `key` - Key value of the [`Egg`] to pop
  ///
  /// [`Egg`]: sparrow::core::egg::Egg
  pub fn pop(&mut self, key: &str) -> Option<Egg> {
    self.map.remove(key)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  const TEST_KEY: &str = "My key";
  const TEST_VALUE: &str = "This is a test value!";

  #[test]
  fn test_new_nest() {
    Nest::new();
  }

  #[fixture]
  fn nest() -> Nest {
    Nest::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_KEY, TEST_VALUE)
  }

  #[test]
  fn test_nest_new() {
    Nest::new();
  }

  #[rstest]
  fn test_nest_insert(mut nest: Nest, egg: Egg) {
    // Egg is not in nest
    assert_eq!(nest.insert(egg.clone()), None);
    // Egg is inserted into the nest and the egg previously associated to its key is returned
    assert_eq!(nest.insert(egg.clone()), Some(egg));
  }

  #[rstest]
  fn test_nest_get(mut nest: Nest, egg: Egg) {
    // Egg is not in the nest
    assert_eq!(nest.get(egg.key()), None);
    // Egg is inserted into the nest and its key wasn't found
    assert_eq!(nest.insert(egg.clone()), None);
    // Egg is in the nest and its value is returned
    assert_eq!(nest.get(egg.key()), Some(&egg))
  }

  #[rstest]
  fn test_nest_pop(mut nest: Nest, egg: Egg) {
    // Egg is inserted into the nest and its key wasn't found
    assert_eq!(nest.insert(egg.clone()), None);
    // Egg is popped from the nest and returned
    assert_eq!(nest.pop(egg.key()), Some(egg.clone()));
    // Egg is not in the nest
    assert_eq!(nest.pop(egg.key()), None);
  }
}
