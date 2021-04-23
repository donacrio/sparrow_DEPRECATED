use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::errors::Result;
use crate::core::nest::Nest;
use std::fmt;

/// Engine INSERT command.
#[derive(Clone, Debug)]
pub struct InsertCommand {
  key: String,
  value: String,
}

impl InsertCommand {
  /// Return a new [`InsertCommand`].
  ///
  /// # Arguments
  /// * `args` - Arguments of this command. There should be 2 argument (key, value).
  ///
  /// # Examples
  /// ```rust
  /// use crate::core::commands::InsertCommand;
  ///
  /// let args = &vec!["my key", "some value"];
  /// let cmd = InsertCommand::new(args).unwrap();
  ///
  /// assert_eq!(format!("{}", cmd), "INSERT {my key} {some value}");
  /// ```
  ///
  /// [`GetCommand`]: crate::core::commands::get_command::GetCommand
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
    write!(f, "INSERT {{{}}} {{{}}}", self.key, self.value)
  }
}

impl Command for InsertCommand {
  /// Execute the `INSERT key value` command on a given [`Nest`].
  ///
  /// [`Nest`]: crate::core::Nest
  fn execute(&self, nest: &mut Nest) -> Option<Egg> {
    nest.insert(Egg::new(&self.key, &self.value))
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::insert_command::InsertCommand;
  use crate::core::commands::Command;
  use crate::core::nest::Nest;
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
