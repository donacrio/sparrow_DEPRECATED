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

use super::commands::Command;
use super::egg::Egg;
use super::errors::Result;
use super::nest::Nest;
use std::collections::VecDeque;

pub struct SparrowEngine {
  commands: VecDeque<Command>,
  nest: Nest,
  outputs: VecDeque<Result<Option<Egg>>>,
}

impl SparrowEngine {
  pub fn new() -> SparrowEngine {
    SparrowEngine {
      commands: VecDeque::new(),
      nest: Nest::new(),
      outputs: VecDeque::new(),
    }
  }
}

impl SparrowEngine {
  pub fn run(&mut self) -> Result<()> {
    loop {
      if let Some(command) = self.commands.pop_front() {
        let output = self.execute(command);
        self.outputs.push_back(output);
      }
    }
  }
  fn execute(&mut self, command: Command) -> Result<Option<Egg>> {
    match command {
      Command::Insert(insert_command) => self.insert(insert_command.key(), insert_command.value()),
      Command::Get(get_command) => self.get(get_command.key()),
      Command::Pop(pop_command) => self.pop(pop_command.key()),
    }
  }
  fn insert(&mut self, key: &str, value: &str) -> Result<Option<Egg>> {
    Ok(self.nest.insert(Egg::new(key, value)))
  }
  fn get(&self, key: &str) -> Result<Option<Egg>> {
    Ok(Some(self.nest.get(key)?.clone()))
  }
  fn pop(&mut self, key: &str) -> Result<Option<Egg>> {
    Ok(Some(self.nest.pop(key)?))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  const TEST_EGG_KEY: &str = "test";
  const TEST_EGG_VALUE: &str = "This is a test value!";

  #[test]
  fn test_sparrow_engine_new() {
    SparrowEngine::new();
  }

  #[fixture]
  fn sparrow_engine() -> SparrowEngine {
    SparrowEngine::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE)
  }

  // #[rstest]
  // fn test_sparrow_engine_insert(mut sparrow_engine: SparrowEngine, egg: Egg) {
  //   // Egg is inserted into sparrow's nest and its key wasn't found
  //   assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
  //   // Egg is inserted into sparrow's nest and the egg previously associated to its key is returned
  //   assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), Some(egg));
  // }

  // #[rstest]
  // fn test_sparrow_engine_get(mut sparrow_engine: SparrowEngine, egg: Egg) {
  //   // Egg is not in sparrow's nest
  //   assert_eq!(
  //     sparrow_engine.get(egg.key()),
  //     Err(errors::EggNotInNestError::new(egg.key()))
  //   );
  //   // Egg is inserted into sparrow's nest and its key wasn't found
  //   assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
  //   // Egg is in sparrow's nest and its value is returned
  //   assert_eq!(sparrow_engine.get(egg.key()), Ok(&egg))
  // }

  // #[rstest]
  // fn test_sparrow_engine_pop(mut sparrow: SparrowEngine, egg: Egg) {
  //   // Egg is inserted into sparrow's nest and its key wasn't found
  //   assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
  //   // Egg is popped from sparrow's nest and returned
  //   assert_eq!(sparrow_engine.pop(egg.key()), Ok(egg.clone()));
  //   // Egg is not in sparrow's nest
  //   assert_eq!(
  //     sparrow_engine.pop(egg.key()),
  //     Err(errors::EggNotInNestError::new(egg.key()))
  //   );
  // }
}
