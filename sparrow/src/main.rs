//! Sparrow is a fast, low-level, lightweight in-memory database.
//! The project is under active development and new features are shipped every week! 🥳
//!
//! # Usage
//!
//! For now Sparrow runs as the following:
//! - The engine is ran in one thread and executes commands received
//! through an input consumer and sends the output using a broadcasting bus.
//! - The TCP server is ran asynchronously using [tokio] backend in the main thread. It receives commands from socket connections
//! and send them to the engine using an input producer. The outputs are retrieved using the engine bus.
//!
//! ```rust
//! async {
//!   use crate::net::run_tcp_server;
//!   use crate::core::Engine;
//!
//!   let mut engine = Engine::new();
//!   let (sender, bus) = engine.init(256);
//!
//!   std::thread::spawn(move || engine.run().unwrap());
//!   run_tcp_server("127.0.0.1:8080".parse().unwrap(), 256, sender, &bus).await.unwrap();
//! };
//! ```
//!
//! [tokio]: tokio
mod cli;
mod core;
mod errors;
mod logger;
mod tcp_server;

use crate::cli::{run_cli, Config};
use crate::core::Engine;
use crate::tcp_server::run_tcp_server;

fn main() {
  logger::init();

  match run_cli() {
    Ok(config) => match config {
      Some(config) => {
        if let Err(err) = run(config) {
          log::error!("{}", err);
          std::process::exit(1);
        };
        std::process::exit(0)
      }
      None => std::process::exit(0),
    },
    Err(err) => {
      log::error!("{}", err);
      std::process::exit(1)
    }
  };
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  log::info!("Using config: {:?}", config);

  // take_hook() returns the default hook in case when a custom one is not set
  let orig_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    // invoke the default handler and exit the process
    orig_hook(panic_info);
    std::process::exit(1);
  }));

  // Create a new engine
  log::info!("Setting up engine");
  let mut engine = Engine::new();
  let engine_sender = engine.init();
  log::debug!("Engine set up");

  // Run the engine
  log::info!("Starting engine thread");
  let t1 = std::thread::spawn(move || engine.run().unwrap());

  // Run the TCP server
  log::info!("Starting TCP server");
  if let Err(err) = run_tcp_server(config.tcp_server_port, engine_sender) {
    log::error!("{}", err);
  }

  log::info!("Shutting down Sparrow engine");
  t1.join().unwrap();
  log::info!("Sparrow engine successfully shut down");

  Ok(())
}
