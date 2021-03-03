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

pub fn parse_command(input: &str) -> Result<Option<Box<dyn Command + Send>>> {
  let inputs = input.split(' ').collect::<Vec<&str>>();
  match inputs.get(0) {
    Some(name) => {
      let args = &inputs[1..];
      match *name {
        "GET" => Ok(Some(Box::new(GetCommand::new(args)?))),
        "INSERT" => Ok(Some(Box::new(InsertCommand::new(args)?))),
        "POP" => Ok(Some(Box::new(PopCommand::new(args)?))),
        "EXIT" => Ok(None),
        unknown => Err(format!("Command not found: {}", unknown).into()),
      }
    }
    None => Err("Command not parsable: Input string not space-separated".into()),
  }
}
