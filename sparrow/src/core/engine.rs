//! Core engine managing the database.

use crate::core::commands::parse_command;
use crate::core::errors::Result;
use crate::core::nest::Nest;
use crate::logger::BACKSPACE_CHARACTER;
use async_std::channel::{unbounded, Receiver, Sender};
use async_std::task;
use sparrow_resp::Data;

/// Input send to the engine through an input sender.
pub struct EngineInput {
  id: String,
  data: Data,
  sender: Sender<Data>,
}

impl EngineInput {
  pub fn new(id: String, data: Data, sender: Sender<Data>) -> EngineInput {
    EngineInput { id, data, sender }
  }
}

impl EngineInput {
  pub fn id(&self) -> &String {
    &self.id
  }
  pub fn data(&self) -> &Data {
    &self.data
  }
  pub fn sender(&self) -> &Sender<Data> {
    &self.sender
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
      log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, input.id(), input.data());
      let command = parse_command(input.data())?;
      let output = command.execute(&mut self.nest);
      log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, input.id(), output);
      log::trace!("Input processed");

      log::trace!("Sending output");
      task::block_on(async move { input.sender().send(output).await })?;
      log::trace!("Output sent");
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::core::{Engine, EngineInput};
  use async_std::channel::unbounded;
  use rstest::*;
  use sparrow_resp::Data;

  const TEST_KEY: &str = "key";
  const TEST_VALUE: &str = "value";

  #[fixture]
  fn engine() -> Engine {
    Engine::new()
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
  async fn test_run_engine(mut engine: Engine) {
    let engine_sender = engine.init();
    std::thread::spawn(move || {
      engine.run().unwrap();
    });

    // Send input insert to engine
    // Result should be None because there is no egg for this value
    let data = Data::BulkString(format!("SET {} {}", TEST_KEY, TEST_VALUE));
    let (sender, receiver) = unbounded();
    engine_sender
      .send(EngineInput::new("1".to_string(), data, sender.clone()))
      .await
      .unwrap();
    let output = receiver.recv().await.unwrap();
    assert_eq!(output, Data::SimpleString("OK".to_string()));

    // Send input get to engine
    // Result should be the previously inserted egg
    let data = Data::BulkString(format!("GET {}", TEST_KEY));
    engine_sender
      .send(EngineInput::new("1".to_string(), data, sender))
      .await
      .unwrap();
    let output = receiver.recv().await.unwrap();
    assert_eq!(output, Data::BulkString(TEST_VALUE.to_string()));
  }
}
