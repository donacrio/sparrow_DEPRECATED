//! Core engine managing the database.

use crate::core::commands::parse_command;
use crate::core::nest::Nest;
use crate::errors::Result;
use crate::logger::BACKSPACE_CHARACTER;
use async_std::channel::{unbounded, Receiver, Sender};
use sparrow_resp::Data;

/// Input send to the engine through an input sender.
pub struct EngineInput {
  /// Requester client's id.
  id: String,
  /// Data encoding the input command for the engine
  data: Data,
  /// Output sender used by the Engine to send output to the client.
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
///
/// ```rust
/// use crate::net::run_tcp_server;
/// use crate::core::Engine;
///
/// let mut engine = Engine::new();
/// let engine_sender = engine.init();
/// let engine_task = task::spawn(async move { engine.run().await });
///
/// let tcp_task =
// task::spawn(async move { run_tcp_server(config.tcp_server_port, engine_sender).await });

/// try_join!(engine_task, tcp_task).map(|_| ())
/// ```
pub struct Engine {
  /// [Nest] used for in-memory data storage.
  nest: Nest,
  /// [async_std] consumer channel used to retrieve inputs for the engine.
  inputs: Option<Receiver<EngineInput>>,
}

impl Engine {
  /// Return a new [Engine].
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
  /// Instantiate an input producer and an input consumer used to retrieve Engine inputs.
  /// The sender is returned so that it can be used by other threads.
  /// The consumer is set in the struct so it can be listened.
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
  /// - Get the next [EngineInput] from the input consumer
  /// - Parse the [Data] it contains into a command.
  /// - Process this command (i.e. execute the command contained in the input)
  /// - Send the output [Data] through the [Sender] contained in the [EngineInput]
  pub async fn run(&mut self) -> Result<()> {
    log::info!("Engine is ready to process commands");
    loop {
      let inputs = self
        .inputs
        .as_ref()
        .ok_or("Sparrow engine is not initialized")?;

      log::trace!("Waiting for engine input");
      let input = inputs.recv().await?;
      log::trace!("Received input");

      log::trace!("Processing input");
      log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, input.id(), input.data());
      let output = match parse_command(input.data()) {
        Ok(command) => command.execute(&mut self.nest),
        Err(err) => Data::Error(format!("{}", err)),
      };
      log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, input.id(), output);
      log::trace!("Input processed");

      log::trace!("Sending output");
      input.sender().send(output).await?;
      log::trace!("Output sent");
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::core::{Engine, EngineInput};
  use async_std::channel::unbounded;
  use async_std::task;
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
    task::spawn(async move {
      engine.run().await.unwrap();
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
