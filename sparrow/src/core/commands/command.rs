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

use super::get_command::GetCommand;
use super::insert_command::InsertCommand;
use super::pop_command::PopCommand;

#[derive(Clone)]
pub enum Command {
  Insert(InsertCommand),
  Get(GetCommand),
  Pop(PopCommand),
}

impl From<InsertCommand> for Command {
  fn from(cmd: InsertCommand) -> Command {
    Command::Insert(cmd)
  }
}

impl From<GetCommand> for Command {
  fn from(cmd: GetCommand) -> Command {
    Command::Get(cmd)
  }
}

impl From<PopCommand> for Command {
  fn from(cmd: PopCommand) -> Command {
    Command::Pop(cmd)
  }
}
