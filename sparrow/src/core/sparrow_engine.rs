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

use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::errors::{PoisonedInputQueueError, PoisonedOutputQueueError, Result};
use crate::core::nest::Nest;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub type SparrowEngineInputQueue = VecDeque<Command>;
pub type SparrowEngineOutputQueue = VecDeque<Result<Option<Egg>>>;

pub struct SparrowEngine {
  input_queue: Arc<Mutex<SparrowEngineInputQueue>>,
  nest: Nest,
  output_queue: Arc<Mutex<SparrowEngineOutputQueue>>,
}

impl SparrowEngine {
  pub fn new(
    input_queue: &Arc<Mutex<SparrowEngineInputQueue>>,
    output_queue: &Arc<Mutex<SparrowEngineOutputQueue>>,
  ) -> SparrowEngine {
    SparrowEngine {
      input_queue: Arc::clone(input_queue),
      nest: Nest::new(),
      output_queue: Arc::clone(output_queue),
    }
  }
}

impl SparrowEngine {
  pub fn run(&mut self) -> Result<()> {
    loop {
      let mut maybe_command;
      // Isolate queue access scope from computations to free
      // the Mutex quicker
      {
        let mut queue = self
          .input_queue
          .lock()
          .map_err(|err| PoisonedInputQueueError::new(&format!("{}", err)))?;
        maybe_command = queue.pop_front();
      }
      if let Some(command) = maybe_command {
        let output = self.execute(command);
        let mut queue = self
          .output_queue
          .lock()
          .map_err(|err| PoisonedOutputQueueError::new(&format!("{}", err)))?;
        queue.push_back(output);
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
  use crate::core::commands::*;
  use crate::core::errors::*;
  use crate::core::{Egg, SparrowEngine};
  use rstest::*;

  const TEST_EGG_KEY: &str = "test";
  const TEST_EGG_VALUE: &str = "This is a test value!";

  #[test]
  fn test_sparrow_engine_new() {
    let input_queue = Arc::new(Mutex::new(SparrowEngineInputQueue::new()));
    let output_queue = Arc::new(Mutex::new(SparrowEngineOutputQueue::new()));
    SparrowEngine::new(&input_queue, &output_queue);
  }

  #[fixture]
  fn sparrow_engine() -> SparrowEngine {
    let input_queue = Arc::new(Mutex::new(SparrowEngineInputQueue::new()));
    let output_queue = Arc::new(Mutex::new(SparrowEngineOutputQueue::new()));
    SparrowEngine::new(&input_queue, &output_queue)
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE)
  }

  #[fixture]
  fn insert_command(egg: Egg) -> Command {
    Command::from(InsertCommand::new(egg.key(), egg.value()))
  }

  #[fixture]
  fn get_command(egg: Egg) -> Command {
    Command::from(GetCommand::new(egg.key()))
  }

  #[fixture]
  fn pop_command(egg: Egg) -> Command {
    Command::from(PopCommand::new(egg.key()))
  }

  #[rstest]
  fn test_sparrow_engine_insert(mut sparrow_engine: SparrowEngine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), Ok(None));
    // Egg is inserted into sparrow's nest and the egg previously associated to its key is returned
    assert_eq!(
      sparrow_engine.insert(egg.key(), egg.value()),
      Ok(Some(egg.clone()))
    );
  }

  #[rstest]
  fn test_sparrow_engine_get(mut sparrow_engine: SparrowEngine, egg: Egg) {
    // Egg is not in sparrow's nest
    assert_eq!(
      sparrow_engine.get(egg.key()),
      Err(SparrowError::from(EggNotInNestError::new(egg.key())))
    );
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), Ok(None));
    // Egg is in sparrow's nest and its value is returned
    assert_eq!(sparrow_engine.get(egg.key()), Ok(Some(egg.clone())));
  }

  #[rstest]
  fn test_sparrow_engine_pop(mut sparrow_engine: SparrowEngine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), Ok(None));
    // Egg is popped from sparrow's nest and returned
    assert_eq!(sparrow_engine.pop(egg.key()), Ok(Some(egg.clone())));
    // Egg is not in sparrow's nest
    assert_eq!(
      sparrow_engine.pop(egg.key()),
      Err(SparrowError::from(EggNotInNestError::new(egg.key())))
    );
  }

  #[rstest]
  fn test_sparrow_engine_execute_insert(
    mut sparrow_engine: SparrowEngine,
    egg: Egg,
    insert_command: Command,
  ) {
    assert_eq!(sparrow_engine.execute(insert_command.clone()), Ok(None));
    assert_eq!(
      sparrow_engine.execute(insert_command.clone()),
      Ok(Some(egg.clone()))
    );
  }

  #[rstest]
  fn test_sparrow_engine_execute_get(
    mut sparrow_engine: SparrowEngine,
    egg: Egg,
    insert_command: Command,
    get_command: Command,
  ) {
    assert_eq!(
      sparrow_engine.execute(get_command.clone()),
      Err(SparrowError::from(EggNotInNestError::new(egg.key())))
    );
    assert_eq!(sparrow_engine.execute(insert_command.clone()), Ok(None));
    assert_eq!(
      sparrow_engine.execute(get_command.clone()),
      Ok(Some(egg.clone()))
    )
  }

  #[rstest]
  fn test_sparrow_engine_execute_pop(
    mut sparrow_engine: SparrowEngine,
    egg: Egg,
    insert_command: Command,
    get_command: Command,
    pop_command: Command,
  ) {
    assert_eq!(sparrow_engine.execute(insert_command.clone()), Ok(None));
    assert_eq!(
      sparrow_engine.execute(pop_command.clone()),
      Ok(Some(egg.clone()))
    );
    assert_eq!(
      sparrow_engine.execute(get_command.clone()),
      Err(SparrowError::from(EggNotInNestError::new(egg.key())))
    );
  }
}
