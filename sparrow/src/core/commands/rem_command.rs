use crate::core::commands::Command;
use crate::core::nest::Nest;
use crate::errors::Result;
use sparrow_resp::Data;
use std::fmt;

/// Engine REM command.
#[derive(Clone, Debug)]
pub struct RemCommand {
  key: String,
}

impl RemCommand {
  /// Return a new [RemCommand].
  ///
  /// # Arguments
  /// * `args` - Arguments of this command. There should be 1 argument (key).
  ///
  /// # Examples
  /// ```rust
  /// use crate::core::commands::RemCommand;
  ///
  /// let args = &vec!["key"];
  /// let cmd = RemCommand::new(args).unwrap();
  ///
  /// assert_eq!(format!("{}", cmd), "REM key");
  /// ```
  pub fn new(args: &[&str]) -> Result<RemCommand> {
    match args.len() {
      1 => {
        let key = args.get(0).unwrap();
        Ok(RemCommand {
          key: key.to_string(),
        })
      }
      n => Err(
        format!(
          "Cannot parse REM command arguments: Wrong number of arguments. Expected 1, got {}.",
          n
        )
        .into(),
      ),
    }
  }
}

impl fmt::Display for RemCommand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "REM {}", self.key)
  }
}

impl Command for RemCommand {
  /// Execute the `POP key` command on a given [Nest].
  fn execute(&self, nest: &mut Nest) -> Data {
    nest.rem(&self.key);
    Data::SimpleString("OK".to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::rem_command::RemCommand;
  use crate::core::commands::Command;
  use crate::core::egg::Egg;
  use crate::core::nest::Nest;
  use rstest::*;
  use sparrow_resp::Data;

  const TEST_KEY: &str = "My key";
  const TEST_VALUE: &str = "This is a test value!";

  #[fixture]
  fn nest() -> Nest {
    Nest::new()
  }

  #[test]
  fn test_command_new_1_args() {
    let args = &vec![TEST_KEY];
    let command = RemCommand::new(args).unwrap();
    assert_eq!(command.key, TEST_KEY)
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse REM command arguments: Wrong number of arguments. Expected 1, got 0."
  )]
  fn test_command_new_0_args() {
    let args = &vec![];
    RemCommand::new(args).unwrap();
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse REM command arguments: Wrong number of arguments. Expected 1, got 2."
  )]
  fn test_command_new_2_args() {
    let args = &vec![TEST_KEY, TEST_VALUE];
    RemCommand::new(args).unwrap();
  }

  #[rstest]
  fn test_command_execute(mut nest: Nest) {
    let args = &vec![TEST_KEY];
    let command = Box::new(RemCommand::new(args).unwrap());

    let data = command.execute(&mut nest);
    assert_eq!(data, Data::SimpleString("OK".to_string()));

    nest.set(Egg::new(TEST_KEY, TEST_VALUE));
    let data = command.execute(&mut nest);
    assert_eq!(data, Data::SimpleString("OK".to_string()));

    let egg = nest.get(TEST_KEY);
    assert!(egg.is_none());
  }
}
