//! Command Line Interface managing environment variables and parameters.

mod config;
mod constants;
mod parameters;

pub use crate::cli::config::Config;

use crate::cli::constants::{ENV_FILEPATH, HELP, TCP_SERVER_PORT};
use getopts::Options;
use std::env;

/// Run the Command Line Interface.
///
/// Return an [`Option`] that is [`None`] if the `help` flag is present
/// in the CLI parameters. Otherwise a [`Some`] containing the [`Config`] object is returned.
///
/// # Usage
/// ```rust
/// use crate::cli::run_cli;
///
/// async {
///   match run_cli() {
///     Ok(config) => match config {
///       Some(config) => {
///         // Run everything here
///         std::process::exit(0)
///       }
///       None => std::process::exit(0),
///     },
///     Err(err) => {
///       log::error!("{}", err);
///       std::process::exit(1)
///     }
///   };
/// };
///```
///
/// [`Config`]: crate::cli::Config
pub fn run_cli() -> Result<Option<Config>, Box<dyn std::error::Error>> {
  // Collect cli parameters
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let opts = get_opts();
  let matches = opts.parse(args)?;

  // If help flag is present we only need to display help
  // then exit.
  if matches.opt_present(HELP.long_name) {
    print_usage(&program, opts);
    return Ok(None);
  }

  // Load default config from specified or default .env file
  // This .env file must contain all the default environment variables
  let mut config = Config::load_env(matches.opt_str(ENV_FILEPATH.long_name))?;

  // Load specific config from parsed cli parameters
  config.update_with_cli_params(matches)?;

  Ok(Some(config))
}

/// Return [`Options`] used to parse CLI parameters.cli
///
/// [`Options`]: getopts::Options
fn get_opts() -> Options {
  let mut opts = Options::new();
  // Add options to parse here
  for option in vec![ENV_FILEPATH, TCP_SERVER_PORT] {
    opts.optopt(
      option.short_name,
      option.long_name,
      option.description,
      option.placeholder,
    );
  }
  // Add flags to parse here
  for flag in vec![HELP] {
    opts.optflag(flag.short_name, flag.long_name, flag.description);
  }
  opts
}

/// Print help message.
fn print_usage(program: &str, opts: getopts::Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}
