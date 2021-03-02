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
use super::message::Message;
use super::nest::Nest;
use crate::commands::Command;
use crate::errors::Result;
use std::sync::mpsc;

// TODO: refactor this for generic immutable struct
pub type EngineInput = Message<Box<dyn Command>>;
pub type EngineOutput = Message<Option<Egg>>;

pub struct Engine {
  nest: Nest,
  receiver: Option<mpsc::Receiver<EngineInput>>,
  sender: Option<mpsc::Sender<EngineOutput>>,
}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      nest: Nest::new(),
      receiver: None,
      sender: None,
    }
  }

  pub fn init(&mut self) -> (mpsc::Sender<EngineInput>, mpsc::Receiver<EngineOutput>) {
    let (input_sender, input_receiver) = mpsc::channel::<EngineInput>();
    let (output_sender, output_receiver) = mpsc::channel::<EngineOutput>();
    self.receiver = Some(input_receiver);
    self.sender = Some(output_sender);
    (input_sender, output_receiver)
  }

  pub fn run(&mut self) -> Result<()> {
    loop {
      // TODO: create corresponding sparrow error
      let receiver = self.receiver.as_ref().unwrap();
      // TODO: create corresponding sparrow error
      let input = receiver.recv().unwrap();
      let command = input.content();
      let output = command.execute(self);
      let sender = self.sender.as_ref().unwrap();
      // TODO: create corresponding sparrow error
      sender.send(EngineOutput::new(input.id(), output)).unwrap();
    }
  }
}

impl Default for Engine {
  fn default() -> Self {
    Self::new()
  }
}

impl Engine {
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
  use crate::core::{Egg, Engine};
  use rstest::*;

  const TEST_EGG_KEY: &str = "test";
  const TEST_EGG_VALUE: &str = "This is a test value!";

  #[test]
  fn test_sparrow_engine_new() {
    Engine::new();
  }

  #[fixture]
  fn sparrow_engine() -> Engine {
    Engine::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_EGG_KEY, TEST_EGG_VALUE)
  }

  #[rstest]
  fn test_sparrow_engine_insert(mut sparrow_engine: Engine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
    // Egg is inserted into sparrow's nest and the egg previously associated to its key is returned
    assert_eq!(
      sparrow_engine.insert(egg.key(), egg.value()),
      Some(egg.clone())
    );
  }

  #[rstest]
  fn test_sparrow_engine_get(mut sparrow_engine: Engine, egg: Egg) {
    // Egg is not in sparrow's nest
    assert_eq!(sparrow_engine.get(egg.key()), None);
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
    // Egg is in sparrow's nest and its value is returned
    assert_eq!(sparrow_engine.get(egg.key()), Some(egg.clone()));
  }

  #[rstest]
  fn test_sparrow_engine_pop(mut sparrow_engine: Engine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(sparrow_engine.insert(egg.key(), egg.value()), None);
    // Egg is popped from sparrow's nest and returned
    assert_eq!(sparrow_engine.pop(egg.key()), Some(egg.clone()));
    // Egg is not in sparrow's nest
    assert_eq!(sparrow_engine.pop(egg.key()), None);
  }
}
