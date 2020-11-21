use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct Egg {
  key: String,
  value: String,
  created_at: Instant,
}

impl Egg {
  pub fn new(key: &str, value: &str) -> Egg {
    Egg {
      key: key.to_string(),
      value: value.to_string(),
      created_at: Instant::now(),
    }
  }
  pub fn key(&self) -> &String {
    &self.key
  }
  pub fn value(&self) -> &String {
    &self.value
  }
  pub fn created_at(&self) -> &Instant {
    &self.created_at
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_new_egg() {
    let egg_key = "test";
    let egg_value = "This is a test value!";
    Egg::new(egg_key, egg_value);
  }

  #[test]
  fn test_egg_getters() {
    let egg_key = "test";
    let egg_value = "This is a test value!";
    let egg = Egg::new(egg_key, egg_value);

    assert_eq!(egg.key(), egg_key);
    assert_eq!(egg.value(), egg_value);
    assert!(egg.created_at() < &Instant::now());
  }
}
