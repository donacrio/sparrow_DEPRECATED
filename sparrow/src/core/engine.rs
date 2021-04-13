//! Core engine managing the database.

use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::errors::Result;
use crate::core::message::Message;
use crate::core::nest::Nest;
use crate::logger::BACKSPACE_CHARACTER;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

const BUS_SIZE: usize = 256;

/// Input send to the engine through an input sender.
pub type EngineInput = Message<Box<dyn Command>>;
/// Output send from the engine through the output consumer.
pub type EngineOutput = Message<Option<Egg>>;

/// Engine that manages the in-memory state and database operations.
///
/// # Examples
/// ```rust
/// async {
///   use sparrow::net::run_tcp_server;
///   use sparrow::core::Engine;
///
///   let mut engine = Engine::new();
///   let (sender, bus) = engine.init();
///
///   std::thread::spawn(move || engine.run().unwrap());
///   run_tcp_server("127.0.0.1", sender, &bus).await.unwrap();
/// };
/// ```
pub struct Engine {
  /// [`Nest`] used for in-memory data storage.
  ///
  /// [`Nest`]: sparrow::core::nest::Nest
  nest: Nest,
  /// [`mpsc`] consumer queue used to retrieve inputs for the engine.
  ///
  /// [`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/
  queue: Option<mpsc::Receiver<EngineInput>>,
  /// [`mpsc`] bus used to broadcast outputs outputs from the engine to the listening receivers.
  ///
  /// [`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/
  bus: Option<Arc<Mutex<bus::Bus<EngineOutput>>>>,
}

impl Engine {
  /// Return a new [`Engine`].
  ///
  /// [`Engine`]: sparrow::core::engine::Engine
  pub fn new() -> Engine {
    Engine {
      nest: Nest::new(),
      queue: None,
      bus: None,
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
  pub fn init(
    &mut self,
  ) -> (
    mpsc::Sender<EngineInput>,
    Arc<Mutex<bus::Bus<EngineOutput>>>,
  ) {
    log::trace!("Initializing engine");
    log::trace!("Creating engine input and output channels");
    let (input_sender, input_receiver) = mpsc::channel::<EngineInput>();
    self.queue = Some(input_receiver);
    self.bus = Some(Arc::new(Mutex::new(bus::Bus::new(BUS_SIZE))));
    log::trace!("Created engine input and output channels");
    log::trace!("Engine initialized");
    (
      input_sender,
      self.bus.as_ref().ok_or("test").unwrap().clone(),
    )
  }

  /// Run the engine.
  ///
  /// Loop infinitely to:
  /// - Get the next [`EngineInput`] from the input consumer
  /// - Process this input (i.e. execute the [`Command`] contained in the input)
  /// - Send the [`EngineOutput`] through the output producer
  ///
  /// [`EngineInput`]: sparrow::core::engine::EngineInput
  /// [`Command`]: sparrow::core::commands::command::Command
  /// [`EngineOutput`]: sparrow::core::engine::EngineOutput
  pub fn run(&mut self) -> Result<()> {
    loop {
      let queue = self
        .queue
        .as_ref()
        .ok_or("Sparrow engine is not initialized")?;

      log::trace!("Waiting for engine input");
      let input = queue.recv()?;
      log::trace!("Received input");

      log::trace!("Processing input");
      let output = self.process(input);
      log::trace!("Input processed");

      log::trace!("Sending output");
      let bus = self
        .bus
        .as_mut()
        .ok_or("Sparrow engine is not initialized")?;
      bus.lock().unwrap().broadcast(output);
      log::trace!("Output sent");
    }
  }

  /// Process an [`EngineInput`].
  ///
  /// [`EngineInput`]: sparrow::core::engine::EngineInput
  fn process(&mut self, input: EngineInput) -> EngineOutput {
    let id = input.id();
    let command = input.content();
    log::info!("{}[{}] {}", BACKSPACE_CHARACTER, id, command);
    let output = command.execute(&mut self.nest);
    log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, id, output);
    EngineOutput::new(id, output)
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::parse_command;
  use crate::core::egg::Egg;
  use crate::core::{Engine, EngineInput};
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
    let (sender, bus) = engine.init();
    let mut receiver = bus.lock().unwrap().add_rx();
    std::thread::spawn(move || {
      engine.run().unwrap();
    });

    // Send input insert to engine
    // Result should be None because there is no egg for this value
    let cmd = &format!("INSERT {} {}", TEST_KEY, TEST_VALUE);
    let cmd = parse_command(cmd).unwrap();
    sender.send(EngineInput::new(1, cmd)).unwrap();
    let output = receiver.recv().unwrap();
    assert_eq!(output.id(), 1);
    assert!(output.content().is_none());

    // Send input get to engine
    // Result should be the previously inserted egg
    let cmd = &format!("GET {}", TEST_KEY);
    let cmd = parse_command(cmd).unwrap();
    sender.send(EngineInput::new(1, cmd)).unwrap();
    let output = receiver.recv().unwrap();
    assert_eq!(output.id(), 1);
    assert_eq!(output.content().clone().unwrap(), egg);
  }
}
