// Copyright [2020] [Donatien Criaud]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use chrono::prelude::{DateTime, Utc};
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Egg {
  key: String,
  value: String,
  created_at: DateTime<Utc>,
}

impl Egg {
  pub fn new(key: &str, value: &str) -> Egg {
    let created_at: DateTime<Utc> = SystemTime::now().into();
    Egg {
      key: key.to_string(),
      value: value.to_string(),
      created_at,
    }
  }
  pub fn key(&self) -> &String {
    &self.key
  }
  pub fn value(&self) -> &String {
    &self.value
  }
  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }
}

impl fmt::Display for Egg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Egg {{ key={}, value={}, created_at={} }}",
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
      "Egg {{ key={}, value={}, created_at={} }}",
      egg.key(),
      egg.value(),
      egg.created_at()
    );
    assert_eq!(format!("{}", egg), expected);
  }
}
