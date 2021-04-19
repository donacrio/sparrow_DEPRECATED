use sparrow::cli::{run_cli, Config};
use sparrow::core::Engine;
use sparrow::logger;
use sparrow::net::run_tcp_server;

#[tokio::main]
async fn main() {
  logger::init();

  match run_cli() {
    Ok(config) => match config {
      Some(config) => {
        if let Err(err) = run(config).await {
          log::error!("{}", err);
          std::process::exit(1);
        };
        std::process::exit(0)
      }
      None => std::process::exit(0),
    },
    Err(err) => {
      log::error!("{}", err);
      std::process::exit(1)
    }
  };
}

async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  log::info!("Using config: {:?}", config);

  // take_hook() returns the default hook in case when a custom one is not set
  let orig_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    // invoke the default handler and exit the process
    orig_hook(panic_info);
    std::process::exit(1);
  }));

  // Create a new engine
  log::info!("Setting up engine");
  let mut engine = Engine::new();
  let (sender, bus) = engine.init(config.tcp_server_max_connections);
  log::trace!("Engine set up");

  // Run the engine
  log::info!("Starting engine thread");
  let t1 = std::thread::spawn(move || engine.run().unwrap());

  // Run the TCP server
  log::info!("Starting TCP server");
  run_tcp_server(
    config.tcp_server_port,
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
