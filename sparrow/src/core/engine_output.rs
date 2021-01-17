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

pub struct EngineOutput {
  id: usize,
  output: Option<Egg>,
}

impl EngineOutput {
  pub fn new(id: usize, output: Option<Egg>) -> EngineOutput {
    EngineOutput { id, output }
  }
}

impl EngineOutput {
  pub fn id(&self) -> usize {
    self.id
  }
  pub fn output(&self) -> &Option<Egg> {
    &self.output
  }
}
