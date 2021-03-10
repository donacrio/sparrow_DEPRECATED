//! Error utilities.

/// Return whether or not an error has kind [`WouldBlock`].
///
/// [`WouldBlock`]: std::io::ErrorKind::WouldBlock
pub fn would_block(err: &std::io::Error) -> bool {
  err.kind() == std::io::ErrorKind::WouldBlock
}

/// Return whether or not an error has kind [`Interrupted`].
///
/// [`Interrupted`]: std::io::ErrorKind::Interrupted
pub fn interrupted(err: &std::io::Error) -> bool {
  err.kind() == std::io::ErrorKind::Interrupted
}
