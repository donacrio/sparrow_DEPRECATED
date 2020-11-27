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

use super::egg::Egg;
use super::errors;
use super::nest::Nest;

pub struct Sparrow {
  nest: Nest,
}

impl Sparrow {
  pub fn new() -> Sparrow {
    Sparrow { nest: Nest::new() }
  }
}

impl Sparrow {
  pub fn insert(&mut self, key: &str, value: &str) -> Option<Egg> {
    self.nest.insert(Egg::new(key, value))
  }
  pub fn get(&self, key: &str) -> Result<&Egg, errors::EggNotInNestError> {
    self.nest.get(key)
  }
  pub fn pop(&mut self, key: &str) -> Result<Egg, errors::EggNotInNestError> {
    self.nest.pop(key)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  const TEST_EGG_KEY: &str = "test";
  const TEST_EGG_VALUE: &str = "This is a test value!";

  #[test]
  fn test_sparrow_new() {
    Sparrow::new();
  }

  #[fixture]
  fn sparrow() -> Sparrow {
    Sparrow::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE)
  }

  #[rstest]
  fn test_sparrow_insert(mut sparrow: Sparrow, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow.insert(egg.key(), egg.value()), None);
    // Egg is inserted into sparrow's nest and the egg previously associated to its key is returned
    assert_eq!(sparrow.insert(egg.key(), egg.value()), Some(egg));
  }

  #[rstest]
  fn test_sparrow_get(mut sparrow: Sparrow, egg: Egg) {
    // Egg is not in sparrow's nest
    assert_eq!(
      sparrow.get(egg.key()),
      Err(errors::EggNotInNestError::new(egg.key()))
    );
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow.insert(egg.key(), egg.value()), None);
    // Egg is in sparrow's nest and its value is returned
    assert_eq!(sparrow.get(egg.key()), Ok(&egg))
  }

  #[rstest]
  fn test_sparrow_pop(mut sparrow: Sparrow, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow.insert(egg.key(), egg.value()), None);
    // Egg is popped from sparrow's nest and returned
    assert_eq!(sparrow.pop(egg.key()), Ok(egg.clone()));
    // Egg is not in sparrow's nest
    assert_eq!(
      sparrow.pop(egg.key()),
      Err(errors::EggNotInNestError::new(egg.key()))
    );
  }
}
