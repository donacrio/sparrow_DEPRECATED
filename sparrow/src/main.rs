// Copyright [2020] [Donatien Criaud]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use sparrow::{run_tcp_server, Engine};

// use sparrow::{Result, SparrowEngine, SparrowNetworkInterface};
const ADDRESS: &str = "127.0.0.1:8080";

fn main() {
  // Create a new engine
  let mut engine = Engine::new();
  let (sender, receiver) = engine.init();

  // take_hook() returns the default hook in case when a custom one is not set
  let orig_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    // invoke the default handler and exit the process
    orig_hook(panic_info);
    std::process::exit(1);
  }));

  // Run the engine
  let t1 = std::thread::spawn(move || engine.run().unwrap());

  // Run the network interface
  let t2 = std::thread::spawn(move || run_tcp_server(ADDRESS, sender, receiver).unwrap());

  t1.join().unwrap();
  t2.join().unwrap();
}
