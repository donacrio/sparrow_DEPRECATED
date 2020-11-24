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

use super::command::Command;
use crate::core::Sparrow;
use std::fmt;

pub struct InsertCommand {
  key: String,
  value: String,
}

impl Command for InsertCommand {
  fn new(args: Vec<&str>) -> Self {
    if args.len() != 2 {
      panic!(
        "Insert command requires exactly two arguments, {} were provided",
        args.len()
      );
    }
    InsertCommand {
      key: args.get(0).unwrap().to_string(),
      value: args.get(1).unwrap().to_string(),
    }
  }
  fn execute(&self, engine: &mut Sparrow) -> Box<dyn fmt::Display> {
    match engine.insert(&self.key, &self.value) {
      Some(egg) => Box::new(egg),
      None => Box::new(String::from("Inserted new value!")),
    }
  }
}
