//! Sparrow is a fast, low-level, lightweight in-memory database.
//! The project is under active development and new features are shipped every week! 🥳
//!
//! # Usage
//!
//! For now Sparrow runs using two threads:
//! - The engine is ran in one thread and executes commands received
//! through an input consumer and sends the output using an output producer.
//! - The TCP server is ran in another thread. It receives commands from socket connections
//! and send them to the engine using an input producer. The outputs are retrieved using an output consumer.
//!
//! ```rust
//! use sparrow::net::run_tcp_server;
//! use sparrow::core::Engine;
//!
//! let mut engine = Engine::new();
//! let (sender, receiver) = engine.init();
//! std::thread::spawn(move || engine.run().unwrap());
//! std::thread::spawn(move || run_tcp_server("127.0.0.1", sender, receiver).unwrap());
//! ```
//! **Note that this behavior is likely to be replaced in the future with async.**

mod errors;
mod utils;

pub mod core;
pub mod logger;
pub mod net;
