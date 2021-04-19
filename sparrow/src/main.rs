use sparrow::config::Config;
use sparrow::core::Engine;
use sparrow::logger;
use sparrow::net::run_tcp_server;
use std::env;

#[tokio::main]
async fn main() {
  // Load environment variables
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let opts = Config::opts();
  let matches = match opts.parse(args) {
    Ok(matches) => matches,
    Err(err) => {
      log::error!("{}", err);
      std::process::exit(1);
    }
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
    std::process::exit(0);
  }

  logger::init();

  // take_hook() returns the default hook in case when a custom one is not set
  let orig_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    // invoke the default handler and exit the process
    orig_hook(panic_info);
    std::process::exit(1);
  }));

  let config = match Config::load(matches) {
    Ok(config) => config,
    Err(e) => {
      log::error!("Cannot load config: {}", e);
      std::process::exit(1);
    }
  };

  log::info!("Using config: {:?}", config);

  if let Err(err) = run(config).await {
    log::error!("{}", err);
    std::process::exit(1);
  }
}

async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  // Create a new engine
  log::info!("Setting up engine");
  let mut engine = Engine::new();
  let (sender, bus) = engine.init(config.engine_output_bus_size);
  log::trace!("Engine set up");

  // Run the engine
  log::info!("Starting engine thread");
  let t1 = std::thread::spawn(move || engine.run().unwrap());

  // Run the TCP server
  log::info!("Starting TCP server");
  run_tcp_server(
    config.tcp_server_address,
    config.tcp_server_max_connections,
    sender,
    &bus,
  )
  .await?;

  log::info!("Shutting down Sparrow engine");
  t1.join().unwrap();
  log::info!("Sparrow engine successfully shut down");

  Ok(())
}

fn print_usage(program: &str, opts: getopts::Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}
