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

pub fn next_token(current: &mut mio::Token) -> mio::Token {
  let next = current.0;
  current.0 += 1;
  mio::Token(next)
}

#[cfg(test)]
mod tests {
  use crate::utils::mio::next_token;

  #[test]
  fn test_next_token() {
    // Ref token is the reference unique token
    let mut ref_token = mio::Token(0);
    assert_eq!(ref_token.0, 0);

    // token has the previous ref_token value
    // ref_token value is incremented
    let token = next_token(&mut ref_token);
    assert_eq!(token.0, 0);
    assert_eq!(ref_token.0, 1);

    // token value has not changed
    // new_token has the previous ref_token value
    // ref_token value is incremented
    let new_token = next_token(&mut ref_token);
    assert_eq!(token.0, 0);
    assert_eq!(new_token.0, 1);
    assert_eq!(ref_token.0, 2);
  }
}
