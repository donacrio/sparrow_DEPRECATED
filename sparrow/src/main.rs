//! Sparrow is a fast, low-level, lightweight in-memory database.
//! The project is under active development and new features are shipped every week! 🥳
//!
//! # Usage
//!
//! For now Sparrow runs as the following:
//! - The engine is ran in one thread and executes commands received
//! through an input consumer and sends the output using a sender.
//! - The TCP socket server is ran asynchronously using [async_std] in the main thread. It receives commands from socket connections
//! and send them to the engine using an input producer. The outputs are retrieved using the engine output sender.
//!
//! # Examples
//!
//! ```rust
//! use crate::net::run_tcp_server;
//! use crate::core::Engine;
//!
//! let mut engine = Engine::new();
//! let sender = engine.init();
//!
//! let t1 = std::thread::spawn(move || engine.run().unwrap());
//! run_tcp_server(3000,sender).await.unwrap();
//!
//! t1.join().unwrap();
//! ```
mod cli;
mod core;
mod errors;
mod logger;
mod tcp_server;

use crate::cli::{run_cli, Config};
use crate::core::Engine;
use crate::errors::Result;
use crate::tcp_server::run_tcp_server;

/// Sparrow core entrypoint.
///
/// Run Sparrow and handles propagated errors.
fn main() {
  logger::init();

  match run_cli() {
    Ok(config) => match config {
      Some(config) => {
        match run(config) {
          Ok(_) => {
            log::info!("Sparrow exited successfully!");
            std::process::exit(0);
          }
          Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
          }
        };
      }
      None => std::process::exit(0),
    },
    Err(err) => {
      log::error!("{}", err);
      std::process::exit(1)
    }
  };
}

/// Run Sparrow engine and TCP socket server.
fn run(config: Config) -> Result<()> {
  log::info!("Running Sparrow with config config: {:?}", config);

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
  run_tcp_server(config.tcp_server_port, engine_sender)?;

  log::info!("Shutting down Sparrow engine");
  t1.join().unwrap();
  log::info!("Sparrow engine successfully shut down");

  Ok(())
}
