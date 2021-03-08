//! Sparrow is a fast, low-level, lightweight in-memory database.
//! The project is under active development and new features are shipped every week! 🥳
//!
//! # Usage
//!
//! For now Sparrow runs using two threads:
//! - The engine is ran in one thread and executes commands received
//! through a consumer and sends the output using a producer.
//! - The TCP server is ran in another thread. It receives commands from socket connections
//! and send them to the engine using a producer. The outputs are retrieved using a consumer.
//!
//! ```rust
//! let mut engine = Engine::new();
//! let (sender, receiver) = engine.init();
//! std::thread::spawn(move || run_engine(engine).unwrap());
//! std::thread::spawn(move || run_tcp_server(ADDRESS, sender, receiver).unwrap());
//! ```
//! **Note that this behavior is likely to be replaced in the future with async.**

mod core;
mod errors;
mod net;
mod utils;

pub mod logger;

pub use self::core::run_engine;
pub use self::core::Engine;
pub use self::net::run_tcp_server;
