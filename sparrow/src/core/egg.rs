//! Base data representation.

use chrono::prelude::{DateTime, Utc};
use std::fmt;
use std::time::SystemTime;

/// Egg is the base representation of data into Sparrow.
///
/// It stores the `key` - `value` pair along with some metadata.
#[derive(Debug, Clone)]
pub struct Egg {
  key: String,
  value: String,
  created_at: DateTime<Utc>,
}

impl Egg {
  /// Return a new [Egg].
  ///
  /// # Arguments
  /// * `key` - The key to store
  /// * `value` - The value to store
  pub fn new(key: &str, value: &str) -> Egg {
    let created_at: DateTime<Utc> = SystemTime::now().into();
    Egg {
      key: key.to_string(),
      value: value.to_string(),
      created_at,
    }
  }
  /// Return private field `key`
  pub fn key(&self) -> &String {
    &self.key
  }
  /// Return private field `value`
  #[allow(unused)]
  pub fn value(&self) -> &String {
    &self.value
  }
  /// Return private field `created_at`
  #[allow(unused)]
  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }
}

impl fmt::Display for Egg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{{ key={}, value={}, created_at={} }}",
      self.key, self.value, self.created_at
    )
  }
}

impl PartialEq for Egg {
  fn eq(&self, other: &Self) -> bool {
    self.key.eq(&other.key) && self.value.eq(&other.value)
  }
}

impl Eq for Egg {}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  const TEST_EGG_KEY: &str = "test";
  const TEST_EGG_VALUE: &str = "This is a test value!";

  #[test]
  fn test_egg_new() {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE);
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE)
  }

  #[rstest]
  fn test_egg_getters(egg: Egg) {
    assert_eq!(egg.key(), TEST_EGG_KEY);
    assert_eq!(egg.value(), TEST_EGG_VALUE);

    let current_time: DateTime<Utc> = SystemTime::now().into();
    // Egg has been create before current time
    assert!(egg.created_at() < &current_time);
  }

  #[rstest]
  fn test_egg_display_impl(egg: Egg) {
    let expected = format!(
      "{{ key={}, value={}, created_at={} }}",
      egg.key(),
      egg.value(),
      egg.created_at()
    );
    assert_eq!(format!("{}", egg), expected);
  }
}
