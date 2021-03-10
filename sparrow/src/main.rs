use sparrow::core::Engine;
use sparrow::logger;
use sparrow::net::run_tcp_server;

const ADDRESS: &str = "127.0.0.1:8080";

fn main() {
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
  let (sender, receiver) = engine.init();
  log::trace!("Engine set up");

  // Run the engine
  log::info!("Starting engine thread");
  let t1 = std::thread::spawn(move || engine.run().unwrap());

  // Run the TCP server
  log::info!("Starting TCP server thread");
  let t2 = std::thread::spawn(move || run_tcp_server(ADDRESS, sender, receiver).unwrap());

  t1.join().unwrap();
  t2.join().unwrap();
  log::info!("Joined threads");
}
