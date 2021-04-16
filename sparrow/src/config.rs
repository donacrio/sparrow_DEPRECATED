use std::env;
use std::error::Error;
use std::net::SocketAddr;

// Command line arguments names
const CL_ENV_FILE: &str = "env-file";
const CL_TCP_SERVER_ADDRESS: &str = "tcp-addr";
const CL_TCP_SERVER_MAX_CONNECTIONS: &str = "max-conn";
const CL_ENGINE_OUTPUT_BUS_SIZE: &str = "output-bus";

// Environment variable names
const EVAR_TCP_SERVER_ADDRESS: &str = "TCP_SERVER_ADDRESS";
const EVAR_TCP_SERVER_MAX_CONNECTIONS: &str = "TCP_SERVER_MAX_CONNECTIONS";
const EVAR_ENGINE_OUTPUT_BUS_SIZE: &str = "ENGINE_OUTPUT_BUS_SIZE";

#[derive(Debug)]
pub struct Config {
  pub tcp_server_address: SocketAddr,
  pub tcp_server_max_connections: usize,
  pub engine_output_bus_size: usize,
}

impl Config {
  pub fn load(matches: getopts::Matches) -> Result<Config, Box<dyn Error>> {
    // Load environment variables from specified .env file
    match matches.opt_str(CL_ENV_FILE) {
      Some(env_file_path) => {
        log::trace!("Using .env file located at {:?}", env_file_path);
        dotenv::from_filename(env_file_path)
      }
      None => dotenv::dotenv(),
    }
    .ok();

    let tcp_server_address: SocketAddr =
      load_from_opts_or_else_env(&matches, CL_TCP_SERVER_ADDRESS, EVAR_TCP_SERVER_ADDRESS)?
        .parse()?;
    let tcp_server_max_connections: usize = load_from_opts_or_else_env(
      &matches,
      CL_TCP_SERVER_MAX_CONNECTIONS,
      EVAR_TCP_SERVER_MAX_CONNECTIONS,
    )?
    .parse()?;
    let engine_output_bus_size: usize = load_from_opts_or_else_env(
      &matches,
      CL_ENGINE_OUTPUT_BUS_SIZE,
      EVAR_ENGINE_OUTPUT_BUS_SIZE,
    )?
    .parse()?;

    Ok(Config {
      tcp_server_address,
      tcp_server_max_connections,
      engine_output_bus_size,
    })
  }

  pub fn opts() -> getopts::Options {
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "display this message");

    opts.optopt("", CL_ENV_FILE, "set .env file", "FILEPATH");
    opts.optopt(
      "",
      CL_TCP_SERVER_ADDRESS,
      "set tcp server address",
      "ADDRESS",
    );
    opts.optopt(
      "",
      CL_TCP_SERVER_MAX_CONNECTIONS,
      "set maximum number of tcp connections",
      "NUMBER",
    );
    opts.optopt(
      "",
      CL_ENGINE_OUTPUT_BUS_SIZE,
      "set engine output bus size",
      "SIZE",
    );

    opts
  }
}

fn load_from_opts_or_else_env(
  matches: &getopts::Matches,
  opt_name: &str,
  evar_name: &str,
) -> Result<String, String> {
  matches
    .opt_str(opt_name)
    .or_else(|| env::var(evar_name).ok())
    .ok_or(format!("Environment variable {} not found", evar_name))
}
