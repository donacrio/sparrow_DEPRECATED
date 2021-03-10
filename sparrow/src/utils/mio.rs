//! Metal I/O utilities.
//!
//! Provides utilities on top of [`mio`] crate.
//!
//! [`mio`]: https://docs.rs/mio/*/mio/

/// Increment a given mio token and return a new token with this value.
///
/// This function is used in Sparrow to generate unique tokens.
/// The last unique token value is often help by a variable that is passed to this function.
/// The token is incremented then cloned.
///
/// # Arguments
///
/// * `token` - mio unique token
pub fn next_token(token: &mut mio::Token) -> mio::Token {
  let next = token.0;
  token.0 += 1;
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
