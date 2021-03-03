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

use sparrow::commands::*;
use sparrow::{Engine, EngineInput};

const TEST_KEY: &str = "test";
const TEST_VALUE: &str = "This is a test value!";

#[test]
fn test_sparrow_engine() {
  // Create a new engine
  let mut sparrow_engine = Engine::new();

  // Hold new shared reference of the engine's queues
  let (sender, receiver) = sparrow_engine.init();

  // Run the engine in a separate thread
  std::thread::spawn(move || sparrow_engine.run().unwrap());

  // Create then insert new commands in the input queue
  let input_command = InsertCommand::new(TEST_KEY, TEST_VALUE);
  let get_command = GetCommand::new(TEST_KEY);
  let pop_command = PopCommand::new(TEST_KEY);
  sender
    .send(EngineInput::new(1, Box::new(input_command)))
    .unwrap();
  sender
    .send(EngineInput::new(1, Box::new(get_command.clone())))
    .unwrap();
  sender
    .send(EngineInput::new(1, Box::new(pop_command)))
    .unwrap();
  sender
    .send(EngineInput::new(1, Box::new(get_command)))
    .unwrap();

  {
    let insert_output = receiver.recv().unwrap();
    assert!(insert_output.content().is_none());
  }
  {
    let get_output = receiver.recv().unwrap();
    assert!(get_output.content().is_some());
    let egg = get_output.content().as_ref().unwrap();
    assert_eq!(egg.key(), TEST_KEY);
    assert_eq!(egg.value(), TEST_VALUE);
  }
  {
    let pop_output = receiver.recv().unwrap();
    assert!(pop_output.content().is_some());
    let egg = pop_output.content().as_ref().unwrap();
    assert_eq!(egg.key(), TEST_KEY);
    assert_eq!(egg.value(), TEST_VALUE);
  }
  {
    let get_output = receiver.recv().unwrap();
    assert!(get_output.content().is_none());
  }
}
