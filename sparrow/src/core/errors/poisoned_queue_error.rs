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

#[derive(Debug, PartialEq, Eq)]
pub struct PoisonedInputQueueError {
  message: String,
}

impl PoisonedInputQueueError {
  pub fn new(message: &str) -> PoisonedInputQueueError {
    PoisonedInputQueueError {
      message: message.to_string(),
    }
  }
}

impl std::fmt::Display for PoisonedInputQueueError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Engine input queue was poisoned: {}", self.message)
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PoisonedOutputQueueError {
  message: String,
}

impl PoisonedOutputQueueError {
  pub fn new(message: &str) -> PoisonedOutputQueueError {
    PoisonedOutputQueueError {
      message: message.to_string(),
    }
  }
}

impl std::fmt::Display for PoisonedOutputQueueError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Engine output queue was poisoned: {}", self.message)
  }
}
