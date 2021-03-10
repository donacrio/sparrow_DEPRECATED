//! Generic engine command interface.

use crate::core::commands::get_command::GetCommand;
use crate::core::commands::insert_command::InsertCommand;
use crate::core::commands::pop_command::PopCommand;
use crate::core::egg::Egg;
use crate::core::nest::Nest;
use crate::errors::Result;
use std::fmt::{Debug, Display};

/// Trait shared by all engine commands.
pub trait Command: Send + Display + Debug {
  /// Execute the command on a given [`Nest`].
  ///
  /// This method is called by [`Engine`] using the `process()` method to pass its [`Nest`].
  ///
  /// # Arguments
  /// * `nest` - The nest containing in-memory data. See [`Nest`]
  ///
  /// # Examples
  /// ```rust
  /// use sparrow::core::commands::Command;
  ///
  ///
  /// ```
  ///
  /// [`Nest`]: crate::core::nest::Nest
  /// [`Engine`]: crate::core::engine::Engine
  fn execute(&self, nest: &mut Nest) -> Option<Egg>;
}

/// Parse a string slice into a command.
///
/// This function returns an [`Option::Some`] containing the Command or [`Option::None`] if the parsed string slice is `"EXIT"`.
///
/// # Arguments
/// * `input` - Input string slice to be parsed
///
/// # Examples
/// ```rust
/// use sparrow::core::commands::parse_command;
///
/// let cmd = parse_command("GET key").unwrap();
///
/// assert_eq!(format!("{}", cmd.unwrap()), "GET {key}");
/// ```
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html
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

#[cfg(test)]
mod tests {
  use crate::core::commands::parse_command;

  #[test]
  fn test_parse_command_valid() {
    let get_cmd = parse_command("GET key").unwrap().unwrap();
    assert_eq!(format!("{}", get_cmd), "GET {key}");

    let insert_cmd = parse_command("INSERT key value").unwrap().unwrap();
    assert_eq!(format!("{}", insert_cmd), "INSERT {key} {value}");

    let pop_cmd = parse_command("POP key").unwrap().unwrap();
    assert_eq!(format!("{}", pop_cmd), "POP {key}");

    let exit_cmd = parse_command("EXIT").unwrap();
    assert!(exit_cmd.is_none());
  }

  #[test]
  #[should_panic(expected = "Command not found: TOTO")]
  fn test_parse_command_unknown() {
    parse_command("TOTO key").unwrap();
  }

  #[test]
  #[should_panic(expected = "Command not found:")]
  fn test_parse_command_empty() {
    parse_command("").unwrap();
  }
}
