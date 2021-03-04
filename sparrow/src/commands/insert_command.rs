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
use crate::errors::Result;
use std::fmt;

#[derive(Clone)]
pub struct InsertCommand {
  key: String,
  value: String,
}

impl InsertCommand {
  pub fn new(args: &[&str]) -> Result<InsertCommand> {
    match args.len() {
      2 => {
        let key = args.get(0).unwrap();
        let value = args.get(1).unwrap();
        Ok(InsertCommand {
          key: key.to_string(),
          value: value.to_string(),
        })
      }
      n => Err(
        format!(
          "Cannot parse INSERT command arguments: Wrong number of arguments. Expected 2, got {}.",
          n
        )
        .into(),
      ),
    }
  }
}

impl fmt::Display for InsertCommand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "INSERT {} {}", self.key, self.value)
  }
}

impl Command for InsertCommand {
  fn execute(&self, sparrow_engine: &mut Engine) -> Option<Egg> {
    sparrow_engine.insert(&self.key, &self.value)
  }
}
