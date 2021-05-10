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
//! let engine_sender = engine.init();
//! let engine_task = task::spawn(async move { engine.run().await });
//!
//! let tcp_task =
// task::spawn(async move { run_tcp_server(config.tcp_server_port, engine_sender).await });

//! try_join!(engine_task, tcp_task).map(|_| ())
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
use async_std::task;
use futures::try_join;

/// Sparrow core entrypoint.
///
/// Run Sparrow and handles propagated errors.
fn main() {
  logger::init();

  match run_cli() {
    Ok(config) => match config {
      Some(config) => {
        match task::block_on(async move { run(config).await }) {
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
async fn run(config: Config) -> Result<()> {
  log::info!("Running Sparrow with config object: {:?}", config);

  // Create a new engine
  log::debug!("Setting up engine");
  let mut engine = Engine::new();
  let engine_sender = engine.init();

  // Run the engine
  log::debug!("Spawning engine task");
  let engine_task = task::spawn(async move { engine.run().await });

  // Run the TCP server
  log::debug!("Spawning TCP server task");
  let tcp_task =
    task::spawn(async move { run_tcp_server(config.tcp_server_port, engine_sender).await });

  try_join!(engine_task, tcp_task).map(|_| ())
}
