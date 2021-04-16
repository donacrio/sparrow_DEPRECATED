//! Error handling utilities for Sparrow's network interface.

use std::fmt;

/// Struct wrapping an [`std::error::Error`] with it's associated [`StatusCode`].
/// It is used to return a [`Response`] using [`hyper`] http server.
///
/// [`std::error::Error`]: std::error::Error
/// [`StatusCode`]: hyper::StatusCode
/// [`Response`]: hyper::Response
/// [`hyper`]: hyper
pub struct Error {
  /// [`StatusCode`] associated to this error.
  ///
  /// [`StatusCode`]: hyper::StatusCode
  status_code: hyper::StatusCode,
  /// [`std::error::Error`] associated to this error.
  ///
  /// [`std::error::Error`]: std::error::Error
  error: Box<dyn std::error::Error>,
}

impl Error {
  /// Return a new [`Error`].
  ///
  /// [`Error`]: sparrow::net::errors::Error
  pub fn new(status_code: hyper::StatusCode, error: Box<dyn std::error::Error>) -> Error {
    Error { status_code, error }
  }
}

impl Error {
  /// Return the [`StatusCode`] associated to this error.
  ///
  /// [`StatusCode`]: hyper::StatusCode
  pub fn status_code(&self) -> &hyper::StatusCode {
    &self.status_code
  }
  /// Return the [`std::error::Error`] associated to this error.
  ///
  /// [`std::error::Error`]: std::error::Error
  #[allow(clippy::borrowed_box)]
  pub fn error(&self) -> &Box<dyn std::error::Error> {
    &self.error
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "error {}: {}", self.status_code, self.error)
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "error {:?}: {:?}", self.status_code, self.error)
  }
}
