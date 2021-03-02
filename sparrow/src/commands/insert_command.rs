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
use crate::core::{Egg, Engine};

#[derive(Clone)]
pub struct InsertCommand {
  key: String,
  value: String,
}

impl InsertCommand {
  pub fn new(key: &str, value: &str) -> InsertCommand {
    InsertCommand {
      key: key.to_string(),
      value: value.to_string(),
    }
  }
}

impl Command for InsertCommand {
  fn execute(&self, sparrow_engine: &mut Engine) -> Option<Egg> {
    sparrow_engine.insert(&self.key, &self.value)
  }
}
