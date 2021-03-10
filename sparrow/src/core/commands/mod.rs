//! Engine commands.
//!
//! This module is used to define commands that will be executed by the [`Engine`].
//!
//! [`Engine`]: crate::core::engine::Engine
mod command;
mod get_command;
mod insert_command;
mod pop_command;

pub use command::{parse_command, Command};
