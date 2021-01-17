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
use super::nest::Nest;
use crate::commands::Command;
use crate::errors::{PoisonedQueueError, Result};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub type SparrowEngineInputQueue = VecDeque<Box<dyn Command + Send>>;
pub type SparrowEngineOutputQueue = VecDeque<Option<Egg>>;

pub struct SparrowEngine {
  input_queue: Arc<Mutex<SparrowEngineInputQueue>>,
  nest: Nest,
  output_queue: Arc<Mutex<SparrowEngineOutputQueue>>,
}

impl SparrowEngine {
  pub fn new() -> SparrowEngine {
    SparrowEngine {
      input_queue: Arc::new(Mutex::new(SparrowEngineInputQueue::new())),
      nest: Nest::new(),
      output_queue: Arc::new(Mutex::new(SparrowEngineOutputQueue::new())),
    }
  }
  pub fn input_queue(&self) -> &Arc<Mutex<SparrowEngineInputQueue>> {
    &self.input_queue
  }

  pub fn output_queue(&self) -> &Arc<Mutex<SparrowEngineOutputQueue>> {
    &self.output_queue
  }
  pub fn run(&mut self) -> Result<()> {
    loop {
      let maybe_command;
      // Isolate queue access scope from computations to free
      // the Mutex quicker
      {
        let mut queue = self
          .input_queue
          .lock()
          .map_err(|err| PoisonedQueueError::new(&format!("{}", err)))?;
        maybe_command = queue.pop_front();
      }
      if let Some(command) = maybe_command {
        let output = command.execute(self);
        let mut queue = self
          .output_queue
          .lock()
          .map_err(|err| PoisonedQueueError::new(&format!("{}", err)))?;
        queue.push_back(output);
      }
    }
  }
}

impl Default for SparrowEngine {
  fn default() -> Self {
    Self::new()
  }
}

impl SparrowEngine {
  pub fn insert(&mut self, key: &str, value: &str) -> Option<Egg> {
    self.nest.insert(Egg::new(key, value))
  }
  pub fn get(&self, key: &str) -> Option<Egg> {
    self.nest.get(key).cloned()
  }
  pub fn pop(&mut self, key: &str) -> Option<Egg> {
    self.nest.pop(key)
  }
}

#[cfg(test)]
mod tests {
  use crate::core::{Egg, SparrowEngine};
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

  #[rstest]
  fn test_sparrow_engine_insert(mut sparrow_engine: SparrowEngine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
    // Egg is inserted into sparrow's nest and the egg previously associated to its key is returned
    assert_eq!(
      sparrow_engine.insert(egg.key(), egg.value()),
      Some(egg.clone())
    );
  }

  #[rstest]
  fn test_sparrow_engine_get(mut sparrow_engine: SparrowEngine, egg: Egg) {
    // Egg is not in sparrow's nest
    assert_eq!(sparrow_engine.get(egg.key()), None);
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
    // Egg is in sparrow's nest and its value is returned
    assert_eq!(sparrow_engine.get(egg.key()), Some(egg.clone()));
  }

  #[rstest]
  fn test_sparrow_engine_pop(mut sparrow_engine: SparrowEngine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
    // Egg is popped from sparrow's nest and returned
    assert_eq!(sparrow_engine.pop(egg.key()), Some(egg.clone()));
    // Egg is not in sparrow's nest
    assert_eq!(sparrow_engine.pop(egg.key()), None);
  }
}
