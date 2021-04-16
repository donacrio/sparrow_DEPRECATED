use sparrow::core::Engine;
use sparrow::logger;
use sparrow::net::run_tcp_server;

const ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
  logger::init();

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
  let (sender, bus) = engine.init();
  log::trace!("Engine set up");

  // Run the engine
  log::info!("Starting engine thread");
  let t1 = std::thread::spawn(move || engine.run().unwrap());

  // Run the TCP server
  log::info!("Starting TCP server");
  run_tcp_server(ADDRESS, sender, &bus).await.unwrap();

  log::info!("Shutting down Sparrow");
  t1.join().unwrap();
  log::info!("Sparrow successfully shut down");
}
