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
mod command;
mod get_command;
mod insert_command;
mod pop_command;

pub use command::parse_command;
pub use command::Command;
pub use get_command::GetCommand;
pub use insert_command::InsertCommand;
pub use pop_command::PopCommand;

#[cfg(test)]
mod tests {
  use crate::commands::*;
  use crate::core::{Egg, SparrowEngine};
  use rstest::*;

  const TEST_KEY: &str = "test";
  const TEST_VALUE: &str = "This is a test value!";

  #[fixture]
  fn sparrow_engine() -> SparrowEngine {
    SparrowEngine::new()
  }

  #[fixture]
  fn egg() -> Egg {
    Egg::new(TEST_KEY, TEST_VALUE)
  }

  #[fixture]
  fn insert_command() -> InsertCommand {
    InsertCommand::new(TEST_KEY, TEST_VALUE)
  }

  #[fixture]
  fn get_command() -> GetCommand {
    GetCommand::new(TEST_KEY)
  }

  #[fixture]
  fn pop_command() -> PopCommand {
    PopCommand::new(TEST_KEY)
  }

  #[rstest]
  fn test_execute_insert_command(
    mut sparrow_engine: SparrowEngine,
    egg: Egg,
    insert_command: InsertCommand,
  ) {
    assert_eq!(insert_command.execute(&mut sparrow_engine), None);
    assert_eq!(insert_command.execute(&mut sparrow_engine), Some(egg));
  }

  #[rstest]
  fn test_execute_get_command(
    mut sparrow_engine: SparrowEngine,
    egg: Egg,
    get_command: GetCommand,
  ) {
    assert_eq!(get_command.execute(&mut sparrow_engine), None);
    sparrow_engine.insert(TEST_KEY, TEST_VALUE);
    assert_eq!(get_command.execute(&mut sparrow_engine), Some(egg))
  }

  #[rstest]
  fn test_execute_pop_command(
    mut sparrow_engine: SparrowEngine,
    egg: Egg,
    pop_command: PopCommand,
  ) {
    sparrow_engine.insert(TEST_KEY, TEST_VALUE);
    assert_eq!(pop_command.execute(&mut sparrow_engine), Some(egg));
    assert_eq!(sparrow_engine.get(TEST_KEY), None);
  }
}
