//! Engine GET command.
//!
use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::nest::Nest;
use crate::errors::Result;
use std::fmt;

/// Engine GET command.
#[derive(Clone, Debug)]
pub struct GetCommand {
  key: String,
}

impl GetCommand {
  /// Return a new [`GetCommand`].
  ///
  /// # Arguments
  /// * `args` - Arguments of this command. There should be 1 argument (key).
  ///
  /// # Examples
  /// ```rust
  /// use sparrow::core::commands::GetCommand;
  ///
  /// let args = &vec!["my key"];
  /// let cmd = GetCommand::new(args).unwrap();
  ///
  /// assert_eq!(format!("{}", cmd), "GET {my key}");
  /// ```
  ///
  /// [`GetCommand`]: sparrow::core::commands::get_command::GetCommand
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

impl fmt::Display for GetCommand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "GET {{{}}}", self.key)
  }
}

impl Command for GetCommand {
  /// Execute the `GET key` command on a given [`Nest`].
  ///
  /// [`Nest`]: sparrow::core::nest::Nest
  fn execute(&self, nest: &mut Nest) -> Option<Egg> {
    nest.get(&self.key).cloned()
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::get_command::GetCommand;
  use crate::core::commands::Command;
  use crate::core::egg::Egg;
  use crate::core::nest::Nest;
  use rstest::*;

  const TEST_KEY: &str = "My key";
  const TEST_VALUE: &str = "This is a test value!";

  #[fixture]
  fn nest() -> Nest {
    Nest::new()
  }

  #[test]
  fn test_command_new_1_args() {
    let args = &vec![TEST_KEY];
    let command = GetCommand::new(args).unwrap();
    assert_eq!(command.key, TEST_KEY)
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse GET command arguments: Wrong number of arguments. Expected 1, got 0."
  )]
  fn test_command_new_0_args() {
    let args = &vec![];
    GetCommand::new(args).unwrap();
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse GET command arguments: Wrong number of arguments. Expected 1, got 2."
  )]
  fn test_command_new_2_args() {
    let args = &vec![TEST_KEY, TEST_VALUE];
    GetCommand::new(args).unwrap();
  }

  #[rstest]
  fn test_command_execute(mut nest: Nest) {
    let args = &vec![TEST_KEY];
    let command = Box::new(GetCommand::new(args).unwrap());

    let egg = command.execute(&mut nest);
    assert!(egg.is_none());

    nest.insert(Egg::new(TEST_KEY, TEST_VALUE));
    let egg = command.execute(&mut nest).unwrap();
    assert_eq!(egg.key(), TEST_KEY);
    assert_eq!(egg.value(), TEST_VALUE);
  }
}
