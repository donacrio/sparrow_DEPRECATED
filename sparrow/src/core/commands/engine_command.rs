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
use crate::core::{Egg, Nest};
use crate::errors::Result;
use std::fmt::{Debug, Display};

pub trait EngineCommand: Send + Display + Debug {
  fn execute(&self, nest: &mut Nest) -> Option<Egg>;
}

pub fn parse_engine_command(input: &str) -> Result<Option<Box<dyn EngineCommand + Send>>> {
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

#[cfg(test)]
mod tests {
  use crate::parse_engine_command;

  #[test]
  fn test_parse_command_valid() {
    let get_cmd = parse_engine_command("GET key").unwrap().unwrap();
    assert_eq!(format!("{}", get_cmd), "GET key");

    let insert_cmd = parse_engine_command("INSERT key value").unwrap().unwrap();
    assert_eq!(format!("{}", insert_cmd), "INSERT key value");

    let pop_cmd = parse_engine_command("POP key").unwrap().unwrap();
    assert_eq!(format!("{}", pop_cmd), "POP key");

    let exit_cmd = parse_engine_command("EXIT").unwrap();
    assert!(exit_cmd.is_none());
  }

  #[test]
  #[should_panic(expected = "Command not found: TOTO")]
  fn test_parse_command_unknown() {
    parse_engine_command("TOTO key").unwrap();
  }

  #[test]
  #[should_panic(expected = "Command not found:")]
  fn test_parse_command_empty() {
    parse_engine_command("").unwrap();
  }
}
