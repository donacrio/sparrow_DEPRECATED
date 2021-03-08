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
use crate::logger::BACKSPACE_CHARACTER;
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
}

impl Default for Engine {
  fn default() -> Self {
    Self::new()
  }
}

impl Engine {
  pub fn init(&mut self) -> (mpsc::Sender<EngineInput>, mpsc::Receiver<EngineOutput>) {
    log::trace!("Initializing engine");
    log::trace!("Creating engine input and output mpsc channels");
    let (input_sender, input_receiver) = mpsc::channel::<EngineInput>();
    let (output_sender, output_receiver) = mpsc::channel::<EngineOutput>();
    log::trace!("Created engine channels");
    self.receiver = Some(input_receiver);
    self.sender = Some(output_sender);
    log::trace!("Engine initialized");
    (input_sender, output_receiver)
  }

  pub fn process(&mut self, input: EngineInput) -> EngineOutput {
    let id = input.id();
    let command = input.content();
    log::info!("{}[{}] {}", BACKSPACE_CHARACTER, id, command);
    let output = command.execute(self);
    log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, id, output);
    EngineOutput::new(id, output)
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

pub fn run_engine(mut engine: Engine) -> Result<()> {
  loop {
    let receiver = engine
      .receiver
      .as_ref()
      .ok_or("Sparrow engine is not initialized")?;

    log::trace!("Waiting for engine input");
    let input = receiver.recv()?;
    log::trace!("Received input");

    log::trace!("Processing input");
    let output = engine.process(input);
    log::trace!("Input processed");

    log::trace!("Sending output");
    let sender = engine
      .sender
      .as_ref()
      .ok_or("Sparrow engine is not initialized")?;
    sender.send(output)?;
    log::trace!("Output sent");
  }
}

#[cfg(test)]
mod tests {
  use crate::commands::parse_command;
  use crate::core::{run_engine, Egg, Engine, EngineInput};
  use rstest::*;

  const TEST_KEY: &str = "key";
  const TEST_VALUE: &str = "value";

  #[fixture]
  fn engine() -> Engine {
    Engine::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_KEY, TEST_VALUE)
  }

  #[test]
  fn test_engine_new() {
    Engine::new();
  }

  #[rstest]
  fn test_engine_init(mut engine: Engine) {
    engine.init();
  }

  #[rstest]
  fn test_run_engine(mut engine: Engine, egg: Egg) {
    let (sender, receiver) = engine.init();
    std::thread::spawn(move || {
      run_engine(engine).unwrap();
    });

    // Send input insert to engine
    // Result should be None because there is no egg for this value
    let cmd = &format!("INSERT {} {}", TEST_KEY, TEST_VALUE);
    let cmd = parse_command(cmd).unwrap().unwrap();
    sender.send(EngineInput::new(1, cmd)).unwrap();
    let output = receiver.recv().unwrap();
    assert_eq!(output.id(), 1);
    assert!(output.content().is_none());

    // Send input get to engine
    // Result should be the previously inserted egg
    let cmd = &format!("GET {}", TEST_KEY);
    let cmd = parse_command(cmd).unwrap().unwrap();
    sender.send(EngineInput::new(1, cmd)).unwrap();
    let output = receiver.recv().unwrap();
    assert_eq!(output.id(), 1);
    assert_eq!(output.content().clone().unwrap(), egg);
  }

  #[rstest]
  fn test_engine_insert(mut engine: Engine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(engine.insert(egg.key(), egg.value()), None);
    // Egg is inserted into sparrow's nest and the egg previously associated to its key is returned
    assert_eq!(engine.insert(egg.key(), egg.value()), Some(egg.clone()));
  }

  #[rstest]
  fn test_engine_get(mut engine: Engine, egg: Egg) {
    // Egg is not in sparrow's nest
    assert_eq!(engine.get(egg.key()), None);
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(engine.insert(egg.key(), egg.value()), None);
    // Egg is in sparrow's nest and its value is returned
    assert_eq!(engine.get(egg.key()), Some(egg.clone()));
  }

  #[rstest]
  fn test_sparrow_engine_pop(mut engine: Engine, egg: Egg) {
    // Egg is inserted into sparrow's nest and its key wasn't found
    assert_eq!(engine.insert(egg.key(), egg.value()), None);
    // Egg is popped from sparrow's nest and returned
    assert_eq!(engine.pop(egg.key()), Some(egg.clone()));
    // Egg is not in sparrow's nest
    assert_eq!(engine.pop(egg.key()), None);
  }
}
