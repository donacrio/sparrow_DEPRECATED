use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::nest::Nest;
use crate::errors::Result;
use sparrow_resp::Data;
use std::fmt;

/// Engine INSERT command.
#[derive(Clone, Debug)]
pub struct SetCommand {
  key: String,
  value: String,
}

impl SetCommand {
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
  pub fn new(args: &[&str]) -> Result<SetCommand> {
    match args.len() {
      2 => {
        let key = args.get(0).unwrap();
        let value = args.get(1).unwrap();
        Ok(SetCommand {
          key: key.to_string(),
          value: value.to_string(),
        })
      }
      n => Err(
        format!(
          "Cannot parse SET command arguments: Wrong number of arguments. Expected 2, got {}.",
          n
        )
        .into(),
      ),
    }
  }
}

impl fmt::Display for SetCommand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "SET {} {}", self.key, self.value)
  }
}

impl Command for SetCommand {
  /// Execute the `INSERT key value` command on a given [`Nest`].
  ///
  /// [`Nest`]: crate::core::Nest
  fn execute(&self, nest: &mut Nest) -> Data {
    nest.insert(Egg::new(&self.key, &self.value));
    Data::SimpleString("OK".to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::set_command::SetCommand;
  use crate::core::commands::Command;
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
  fn test_command_new_2_args() {
    let args = &vec![TEST_KEY, TEST_VALUE];
    let command = SetCommand::new(args).unwrap();
    assert_eq!(command.key, TEST_KEY);
    assert_eq!(command.value, TEST_VALUE);
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse SET command arguments: Wrong number of arguments. Expected 2, got 0."
  )]
  fn test_command_new_0_args() {
    let args = &vec![];
    SetCommand::new(args).unwrap();
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse SET command arguments: Wrong number of arguments. Expected 2, got 3."
  )]
  fn test_command_new_3_args() {
    let args = &vec![TEST_KEY, TEST_VALUE, TEST_VALUE];
    SetCommand::new(args).unwrap();
  }

  #[rstest]
  fn test_command_execute(mut nest: Nest) {
    let args = &vec![TEST_KEY, TEST_VALUE];
    let set_command = Box::new(SetCommand::new(args).unwrap());

    let data = set_command.execute(&mut nest);
    assert_eq!(data, Data::SimpleString("OK".to_string()));

    let egg = nest.get(TEST_KEY).unwrap();
    assert_eq!(egg.key(), TEST_KEY);
    assert_eq!(egg.value(), TEST_VALUE);
  }
}
