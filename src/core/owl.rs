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

use super::egg::Egg;
use super::errors;
use super::nest::Nest;

pub struct Owl {
  nest: Nest,
}

impl Owl {
  pub fn new() -> Owl {
    Owl { nest: Nest::new() }
  }
}

impl Owl {
  pub fn insert(&mut self, key: &str, value: &str) -> Option<Egg> {
    self.nest.insert(Egg::new(key, value))
  }
  pub fn get(&self, key: &str) -> Result<&Egg, errors::EggNotInNestError> {
    self.nest.get(key)
  }
  pub fn pop(&mut self, key: &str) -> Result<Egg, errors::EggNotInNestError> {
    self.nest.pop(key)
  }
}
