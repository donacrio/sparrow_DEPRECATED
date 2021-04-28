//! Logging module.
//!
//! Provides utilities to setup and parameterize Sparrow's logging using crate [`env_logger`].
//!
//! The following environment variables are used:
//! - `LOG_LEVEL`: logging filtering level. Values: `trace`, `debug`(default), `info`, `error`.
//! - `LOG_STYLE`: logging color style. Values: `auto`, `always` (default), `never`
//!
//! Logs are formatted using the pattern: `[timestamp][path][level] <message>`
//!
//! [`env_logger`]: https://docs.rs/env_logger/*/env_logger/

use std::io::Write;

/// Backspace character constant used to format logging.
pub const BACKSPACE_CHARACTER: &str = "\x08";

/// Initialize the logger.
pub fn init() {
  let env = env_logger::Env::default()
    .filter_or("LOG_LEVEL", "debug")
    .write_style_or("LOG_STYLE", "always");

  env_logger::Builder::from_env(env)
    .format(|buf, record| {
      let timestamp = buf.timestamp_millis();
      writeln!(
        buf,
        "[{}][{}][{}] {}",
        timestamp,
        record.target(),
        record.level(),
        record.args()
      )
    })
    .init()
}
