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

#[derive(Clone)]
pub struct GetCommand {
  key: String,
}

impl GetCommand {
  pub fn new(args: &[&str]) -> Result<GetCommand> {
    match args.len() {
      1 => {
        let key = args.get(0).unwrap();
        Ok(GetCommand {
          key: key.to_string(),
        })
      }
      n => Err(
        format!(
          "Cannot parse GET command arguments: Wrong number of arguments. Expected 1, got {}.",
          n
        )
        .into(),
      ),
    }
  }
}

impl Command for GetCommand {
  fn execute(&self, sparrow_engine: &mut Engine) -> Option<Egg> {
    sparrow_engine.get(&self.key)
  }
}
