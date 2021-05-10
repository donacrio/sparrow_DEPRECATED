//! Config struct used to parse environment variable and CLI parameters.

use crate::cli::constants::TCP_SERVER_PORT;
use getopts::Matches;
use std::env;
use std::error::Error;

/// Config that holds values used to parameterize
/// Sparrow's Engine and Network Interface.
#[derive(Debug)]
pub struct Config {
  /// TCP listening port of Sparrow's Network Interface.
  pub tcp_server_port: u16,
}

impl Config {
  /// Load a new default [Config] from a .env file.
  ///
  /// The .env file path can be specified.
  /// If not provided, the default .env file will be used.Matches
  ///
  /// **Because this methods loads a default [Config],
  /// all environment variables must be defined in the .env file**
  pub fn load_env(env_filepath: Option<String>) -> Result<Config, Box<dyn Error>> {
    // Load environment variables from specified .env file
    match env_filepath {
      Some(env_file_path) => {
        log::trace!("Using .env file located at {:?}", env_file_path);
        dotenv::from_filename(env_file_path)
      }
      None => {
        log::trace!("Using default .env file");
        dotenv::dotenv()
      }
    }?;

    // Parse environment variables here
    let tcp_server_port: u16 = env::var(TCP_SERVER_PORT.evar_name)?.parse()?;

    Ok(Config { tcp_server_port })
  }
}

impl Config {
  /// Override the config with the given CLI parameters.
  pub fn update_with_cli_params(&mut self, matches: Matches) -> Result<(), Box<dyn Error>> {
    // Parse cli parameters here
    if let Some(tcp_server_port) = matches.opt_str(TCP_SERVER_PORT.long_name) {
      self.tcp_server_port = tcp_server_port.parse()?;
    };

    Ok(())
  }
}
