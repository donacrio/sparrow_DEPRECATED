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
use super::EngineCommand;
use crate::core::{Egg, Nest};
use crate::errors::Result;
use std::fmt;

#[derive(Clone, Debug)]
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

impl EngineCommand for InsertCommand {
  fn execute(&self, nest: &mut Nest) -> Option<Egg> {
    nest.insert(Egg::new(&self.key, &self.value))
  }
}

#[cfg(test)]
mod tests {
  use super::{EngineCommand, InsertCommand};
  use crate::core::Nest;
  use rstest::*;

  const TEST_KEY: &str = "My key";
  const TEST_VALUE: &str = "This is a test value!";

  #[fixture]
  fn nest() -> Nest {
    Nest::new()
  }

  #[test]
  fn test_command_new_2_args() {
    let args = &vec![TEST_KEY, TEST_VALUE];
    let command = InsertCommand::new(args).unwrap();
    assert_eq!(command.key, TEST_KEY);
    assert_eq!(command.value, TEST_VALUE);
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse INSERT command arguments: Wrong number of arguments. Expected 2, got 0."
  )]
  fn test_command_new_0_args() {
    let args = &vec![];
    InsertCommand::new(args).unwrap();
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse INSERT command arguments: Wrong number of arguments. Expected 2, got 3."
  )]
  fn test_command_new_3_args() {
    let args = &vec![TEST_KEY, TEST_VALUE, TEST_VALUE];
    InsertCommand::new(args).unwrap();
  }

  #[rstest]
  fn test_command_execute(mut nest: Nest) {
    let args = &vec![TEST_KEY, TEST_VALUE];
    let command = Box::new(InsertCommand::new(args).unwrap());

    let egg = command.execute(&mut nest);
    assert!(egg.is_none());

    let egg = command.execute(&mut nest).unwrap();
    assert_eq!(egg.key(), TEST_KEY);
    assert_eq!(egg.value(), TEST_VALUE);
  }
}
