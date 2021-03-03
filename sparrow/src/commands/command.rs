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

use super::{GetCommand, InsertCommand, PopCommand};
use crate::core::{Egg, Engine};
use crate::errors::Result;

pub trait Command: Send {
  fn execute(&self, sparrow_engine: &mut Engine) -> Option<Egg>;
}

pub fn parse_command(input: &str) -> Result<Box<dyn Command + Send>> {
  let inputs = input.split(' ').collect::<Vec<&str>>();
  match inputs.get(0) {
    Some(name) => match *name {
      "GET" => Ok(Box::new(GetCommand::new("test"))),
      "INSERT" => Ok(Box::new(InsertCommand::new("test", "test"))),
      "POP" => Ok(Box::new(PopCommand::new("test"))),
      unknown => Err(format!("Command not found: {}", unknown))?,
    },
    None => Err(format!(
      "Command not parsable: Input string not space-separated"
    ))?,
  }
}
