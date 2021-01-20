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
use sparrow::{EngineInput, SparrowEngine};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const TEST_KEY: &str = "test";
const TEST_VALUE: &str = "This is a test value!";

#[test]
fn test_sparrow_engine() {
  // Create a new engine
  let mut sparrow_engine = SparrowEngine::new();

  // Hold new shared reference of the engine's queues
  let inputs = Arc::clone(sparrow_engine.inputs());
  let outputs = Arc::clone(sparrow_engine.outputs());

  assert!(inputs.lock().unwrap().is_empty());
  assert!(outputs.lock().unwrap().is_empty());

  // Create then insert new commands in the input queue
  let input_command = InsertCommand::new(TEST_KEY, TEST_VALUE);
  let get_command = GetCommand::new(TEST_KEY);
  let pop_command = PopCommand::new(TEST_KEY);
  inputs
    .lock()
    .unwrap()
    .push_back(EngineInput::new(1, Box::new(input_command)));
  inputs
    .lock()
    .unwrap()
    .push_back(EngineInput::new(1, Box::new(get_command.clone())));
  inputs
    .lock()
    .unwrap()
    .push_back(EngineInput::new(1, Box::new(pop_command)));
  inputs
    .lock()
    .unwrap()
    .push_back(EngineInput::new(1, Box::new(get_command)));

  assert_eq!(inputs.lock().unwrap().len(), 4);

  // Run the engine in a separate thread
  thread::spawn(move || sparrow_engine.run());

  // Sleep current thread to let the engine process the input commands
  thread::sleep(Duration::from_millis(10));

  assert_eq!(inputs.lock().unwrap().len(), 0);

  // TODO: rewrite this test

  // Check the content of outputs
  // let outputs_content = outputs.lock().unwrap();
  // assert_eq!(outputs_content.len(), 1);
  // let outputs_content = outputs_content.get(&1).unwrap();
  // assert_eq!(outputs_content.len(), 4);

  // {
  //   let insert_output = outputs_content.get(0).unwrap();
  //   assert!(insert_output.output().is_none());
  // }
  // {
  //   let get_output = outputs_content.get(1).unwrap().clone();
  //   assert!(get_output.output().is_some());
  //   let egg = get_output.output().as_ref().unwrap();
  //   assert_eq!(egg.key(), TEST_KEY);
  //   assert_eq!(egg.value(), TEST_VALUE);
  // }
  // {
  //   let pop_output = outputs_content.get(2).unwrap().clone();
  //   assert!(pop_output.output().is_some());
  //   let egg = pop_output.output().as_ref().unwrap();
  //   assert_eq!(egg.key(), TEST_KEY);
  //   assert_eq!(egg.value(), TEST_VALUE);
  // }
  // {
  //   let get_output = outputs_content.get(3).unwrap().clone();
  //   assert!(get_output.output().is_none());
  // }
}
