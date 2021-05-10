//! Generic engine command interface.

use crate::core::commands::get_command::GetCommand;
use crate::core::commands::rem_command::RemCommand;
use crate::core::commands::set_command::SetCommand;
use crate::core::nest::Nest;
use crate::errors::Result;
use sparrow_resp::Data;
use std::fmt::{Debug, Display};

/// Trait shared by all engine commands.
pub trait Command: Send + Sync + Display + Debug {
  /// Execute the command on a given [Nest].
  ///
  /// # Arguments
  /// * `nest` - The nest containing in-memory data. See [Nest]
  ///
  /// # Examples
  /// ```rust
  /// use crate::core::commands::{Command, GetCommand};
  /// use crate::core::nest::Nest;
  ///
  /// let nest = Nest::new();
  /// let command = GetCommand::new(&vec!["key"]);
  /// command.execute(&mut self.nest)
  /// ```
  fn execute(&self, nest: &mut Nest) -> Data;
}

pub fn parse_command(input: &Data) -> Result<Box<dyn Command>> {
  match input {
    Data::BulkString(input) => parse_string_command(input),
    _ => Err("Cannot parse command: data is not a bulk string".into()),
  }
}
/// Parse a string slice into a command.
///
/// This function returns an [Option::Some] containing the Command or [Option::None] if the parsed string slice is `"EXIT"`.
///
/// # Arguments
/// * `input` - Input string slice to be parsed
///
/// # Examples
/// ```rust
/// use crate::core::commands::parse_command;
///
/// let cmd = parse_command("GET key").unwrap();
///
/// assert_eq!(format!("{}", cmd), "GET {key}");
/// ```
fn parse_string_command(input: &str) -> Result<Box<dyn Command>> {
  let inputs = input.split(' ').collect::<Vec<&str>>();
  match inputs.get(0) {
    Some(name) => {
      let args = &inputs[1..];
      match *name {
        "GET" => Ok(Box::new(GetCommand::new(args)?)),
        "SET" => Ok(Box::new(SetCommand::new(args)?)),
        "REM" => Ok(Box::new(RemCommand::new(args)?)),
        unknown => Err(format!("Command not found: {}", unknown).into()),
      }
    }
    None => Err("Command not parsable: Input string not space-separated".into()),
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::parse_command;
  use sparrow_resp::Data;

  #[test]
  fn test_parse_command_valid() {
    let get_cmd = parse_command(&Data::BulkString("GET key".to_string())).unwrap();
    assert_eq!(format!("{}", get_cmd), "GET key");

    let set_cmd = parse_command(&Data::BulkString("SET key value".to_string())).unwrap();
    assert_eq!(format!("{}", set_cmd), "SET key value");

    let rem_cmd = parse_command(&Data::BulkString("REM key".to_string())).unwrap();
    assert_eq!(format!("{}", rem_cmd), "REM key");
  }

  #[test]
  #[should_panic(expected = "Command not found: TOTO")]
  fn test_parse_command_unknown() {
    parse_command(&Data::BulkString("TOTO key".to_string())).unwrap();
  }

  #[test]
  #[should_panic(expected = "Command not found:")]
  fn test_parse_command_empty() {
    parse_command(&Data::BulkString("".to_string())).unwrap();
  }

  #[test]
  #[should_panic(expected = "Cannot parse command: data is not a bulk string")]
  fn test_parse_command_null() {
    parse_command(&Data::Null).unwrap();
  }
}
