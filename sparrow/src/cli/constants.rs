//! Constant variables used to parse CLI parameters.

use crate::cli::parameters::{CliFlag, CliOpt};

pub const ENV_FILEPATH: CliOpt = CliOpt::new(
  "",
  "env-file",
  "set .env filepath",
  "FILEPATH",
  "ENV_FILEPATH",
);
pub const TCP_SERVER_PORT: CliOpt = CliOpt::new(
  "p",
  "port",
  "set tcp server port",
  "PORT",
  "TCP_SERVER_PORT",
);

pub const HELP: CliFlag = CliFlag::new("h", "help", "display this message");
