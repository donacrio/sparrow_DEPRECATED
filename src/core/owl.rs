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

pub struct Owl {
  nest: Nest,
}

impl Owl {
  pub fn new() -> Owl {
    Owl { nest: Nest::new() }
  }
}

impl Owl {
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
  fn test_owl_new() {
    Owl::new();
  }

  #[fixture]
  fn owl() -> Owl {
    Owl::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE)
  }

  #[rstest]
  fn test_owl_insert(mut owl: Owl, egg: Egg) {
    // Egg is inserted into owl's nest and its key wasn't found
    assert_eq!(owl.insert(egg.key(), egg.value()), None);
    // Egg is inserted into owl's nest and the egg previously associated to its key is returned
    assert_eq!(owl.insert(egg.key(), egg.value()), Some(egg));
  }

  #[rstest]
  fn test_owl_get(mut owl: Owl, egg: Egg) {
    // Egg is not in owl's nest
    assert_eq!(
      owl.get(egg.key()),
      Err(errors::EggNotInNestError::new(egg.key()))
    );
    // Egg is inserted into owl's nest and its key wasn't found
    assert_eq!(owl.insert(egg.key(), egg.value()), None);
    // Egg is in owl's nest and its value is returned
    assert_eq!(owl.get(egg.key()), Ok(&egg))
  }

  #[rstest]
  fn test_owl_pop(mut owl: Owl, egg: Egg) {
    // Egg is inserted into owl's nest and its key wasn't found
    assert_eq!(owl.insert(egg.key(), egg.value()), None);
    // Egg is popped from owl's nest and returned
    assert_eq!(owl.pop(egg.key()), Ok(egg.clone()));
    // Egg is not in owl's nest
    assert_eq!(
      owl.pop(egg.key()),
      Err(errors::EggNotInNestError::new(egg.key()))
    );
  }
}
