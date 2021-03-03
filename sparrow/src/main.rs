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
  let mut sparrow_engine = Engine::new();
  let (sender, receiver) = sparrow_engine.init();
  // Run the engine
  // TODO: run it in a different thread
  let sparrow_engine_thread = std::thread::spawn(move || sparrow_engine.run().unwrap());

  // Run the network interface
  let sparrow_net_thread =
    std::thread::spawn(move || run_tcp_server(ADDRESS, sender, receiver).unwrap());

  sparrow_engine_thread.join().unwrap();
  sparrow_net_thread.join().unwrap();
}
