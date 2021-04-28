//! Core engine managing the database.

use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::errors::Result;
use crate::core::nest::Nest;
use crate::logger::BACKSPACE_CHARACTER;
use async_std::channel::{unbounded, Receiver, Sender};
use async_std::task;

/// Input send to the engine through an input sender.
pub struct EngineInput {
  id: String,
  command: Box<dyn Command>,
  sender: Sender<EngineOutput>,
}

impl EngineInput {
  pub fn new(id: String, command: Box<dyn Command>, sender: Sender<EngineOutput>) -> EngineInput {
    EngineInput {
      id,
      command,
      sender,
    }
  }
}

impl EngineInput {
  pub fn id(&self) -> &String {
    &self.id
  }
  pub fn command(&self) -> &Box<dyn Command> {
    &self.command
  }
  pub fn sender(&self) -> &Sender<EngineOutput> {
    &self.sender
  }
}

/// Output send from the engine through the output sender.
#[derive(Debug)]
pub struct EngineOutput {
  id: String,
  output: Option<Egg>,
}

impl EngineOutput {
  pub fn id(&self) -> &String {
    &self.id
  }
  pub fn output(&self) -> &Option<Egg> {
    &self.output
  }
}

impl EngineOutput {
  pub fn new(id: String, output: Option<Egg>) -> EngineOutput {
    EngineOutput { id, output }
  }
}

/// Engine that manages the in-memory state and database operations.
///
/// # Examples
/// ```rust
/// async {
///   use crate::net::run_tcp_server;
///   use crate::core::Engine;
///
///   let mut engine = Engine::new();
///   let (sender, bus) = engine.init(256);
///
///   std::thread::spawn(move || engine.run().unwrap());
///   run_tcp_server("127.0.0.1:8080".parse().unwrap(), 256, sender, &bus).await.unwrap();
/// };
/// ```
pub struct Engine {
  /// [`Nest`] used for in-memory data storage.
  ///
  /// [`Nest`]: crate::core::Nest
  nest: Nest,
  /// [`mpsc`] consumer queue used to retrieve inputs for the engine.
  ///
  /// [`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/
  inputs: Option<Receiver<EngineInput>>,
}

impl Engine {
  /// Return a new [`Engine`].
  ///
  /// [`Engine`]: crate::core::Engine
  pub fn new() -> Engine {
    Engine {
      nest: Nest::new(),
      inputs: None,
    }
  }
}

impl Default for Engine {
  fn default() -> Self {
    Self::new()
  }
}

impl Engine {
  /// Initialize the engine
  ///
  /// Instantiate an return the input and output producers and consumers
  /// use to communicate with the engine through threads.
  pub fn init(&mut self) -> Sender<EngineInput> {
    log::trace!("Initializing engine");
    let (input_sender, input_receiver) = unbounded();
    self.inputs = Some(input_receiver);
    log::trace!("Engine initialized");
    input_sender
  }

  /// Run the engine.
  ///
  /// Loop infinitely to:
  /// - Get the next [`EngineInput`] from the input consumer
  /// - Process this input (i.e. execute the [`Command`] contained in the input)
  /// - Send the [`EngineOutput`] through the output producer
  ///
  /// [`EngineInput`]: crate::core::EngineInput
  /// [`Command`]: crate::core::commands::Command
  /// [`EngineOutput`]: crate::core::EngineOutput
  pub fn run(&mut self) -> Result<()> {
    loop {
      let inputs = self
        .inputs
        .as_ref()
        .ok_or("Sparrow engine is not initialized")?
        .clone();

      log::trace!("Waiting for engine input");
      let input = task::block_on(async move { inputs.recv().await })?;
      log::trace!("Received input");

      log::trace!("Processing input");
      log::info!(
        "{}[{}] {}",
        BACKSPACE_CHARACTER,
        input.id(),
        input.command()
      );
      let output = input.command().execute(&mut self.nest);
      log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, input.id(), output);
      log::trace!("Input processed");

      log::trace!("Sending output");
      task::block_on(async move {
        input
          .sender()
          .send(EngineOutput::new(input.id().clone(), output))
          .await
      })?;
      log::trace!("Output sent");
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::parse_command;
  use crate::core::egg::Egg;
  use crate::core::{Engine, EngineInput};
  use async_std::channel::unbounded;
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
  #[async_std::test]
  async fn test_run_engine(mut engine: Engine, egg: Egg) {
    let engine_sender = engine.init();
    std::thread::spawn(move || {
      engine.run().unwrap();
    });

    // Send input insert to engine
    // Result should be None because there is no egg for this value
    let command = format!("INSERT {} {}", TEST_KEY, TEST_VALUE);
    let command = parse_command(command).unwrap();
    let (sender, receiver) = unbounded();
    engine_sender
      .send(EngineInput::new("1".to_string(), command, sender.clone()))
      .await
      .unwrap();
    let output = receiver.recv().await.unwrap();
    assert_eq!(output.id(), "1");
    assert!(output.output().is_none());

    // Send input get to engine
    // Result should be the previously inserted egg
    let command = format!("GET {}", TEST_KEY);
    let command = parse_command(command).unwrap();
    engine_sender
      .send(EngineInput::new("1".to_string(), command, sender))
      .await
      .unwrap();
    let output = receiver.recv().await.unwrap();
    assert_eq!(output.id(), "1");
    assert_eq!(output.output().clone().unwrap(), egg);
  }
}
