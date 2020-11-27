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

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct EggNotInNestError {
  key: String,
}

impl EggNotInNestError {
  pub fn new(key: &str) -> EggNotInNestError {
    EggNotInNestError {
      key: key.to_string(),
    }
  }
}

impl fmt::Display for EggNotInNestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "No egg with key \"{}\" was found in the nest", self.key)
  }
}