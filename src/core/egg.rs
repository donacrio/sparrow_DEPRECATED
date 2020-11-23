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

const DATETIME_FORMAT: &str = "%+";

#[derive(Debug, Clone, PartialEq)]
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
      self.key,
      self.value,
      self.created_at.format(DATETIME_FORMAT)
    )
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

    let current_time: DateTime<Utc> = SystemTime::now().into();
    assert!(egg.created_at() < &current_time);
  }
}
