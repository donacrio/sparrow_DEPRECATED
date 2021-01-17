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

use sparrow::errors::Result;
use sparrow::{SparrowEngine, SparrowNetworkInterface};
use std::sync::Arc;

// use sparrow::{Result, SparrowEngine, SparrowNetworkInterface};
const ADDRESS: &str = "127.0.0.1:8080";

fn main() -> Result<()> {
  // Create a new engine
  let sparrow_engine = SparrowEngine::new();
  // Run the engine
  // TODO: run it in a different thread
  // sparrow_engine.run()?;

  // Create a new network interface
  let mut sparrow_network_interface = SparrowNetworkInterface::new(
    ADDRESS,
    Arc::clone(sparrow_engine.input_queue()),
    Arc::clone(sparrow_engine.output_queue()),
  );
  // Run the network interface
  sparrow_network_interface.run_tcp_server()?;

  Ok(())
}
