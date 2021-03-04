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

use std::io::Write;

// Backspace character used to format logging
pub const BACKSPACE_CHARACTER: &str = "\x08";

pub fn init() {
  let env = env_logger::Env::default()
    .filter_or("LOG_LEVEL", "debug")
    .write_style_or("LOG_STYLE", "always");

  env_logger::Builder::from_env(env)
    .format(|buf, record| {
      let timestamp = buf.timestamp_millis();
      writeln!(
        buf,
        "[{}][{}][{}] {}",
        timestamp,
        record.target(),
        record.level(),
        record.args()
      )
    })
    .init()
}
