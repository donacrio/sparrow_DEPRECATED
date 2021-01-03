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

mod commands;

use commands::{Command, GetCommand, InsertCommand, PopCommand};
use std::error;
use std::fmt;
use std::io;

pub struct Cli {}

impl Cli {
  pub fn new() -> Cli {
    Cli {}
  }
}

impl Default for Cli {
  fn default() -> Self {
    Self::new()
  }
}

impl Cli {
  pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
    loop {
      let mut input = String::new();
      io::stdin().read_line(&mut input)?;

      let input: Vec<&str> = input.trim().split(' ').collect();
      if let Some(command_type) = input.get(0) {
        let command_args: Vec<&str> = input[1..].to_vec();
        let result: Box<dyn fmt::Display> = match *command_type {
          "insert" => Box::new(InsertCommand::new(command_args)),
          "get" => Box::new(GetCommand::new(command_args)),
          "pop" => Box::new(PopCommand::new(command_args)),
          _ => Box::new("Not found"),
        };
        println!("Received command: {}", result);
      }
    }
  }
}
