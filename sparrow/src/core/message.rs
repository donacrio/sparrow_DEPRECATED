//! Generic DTO to pass message.

use std::fmt;

/// Data Transfer Object with an `id` and some `content`.
#[derive(Clone, Default)]
pub struct Message<T> {
  id: usize,
  content: T,
}

impl<T> Message<T> {
  /// Return a new [`Message`]
  ///
  /// [`Message`]: sparrow::core::message::Message
  pub fn new(id: usize, content: T) -> Message<T> {
    Message { id, content }
  }
}

impl<T> Message<T> {
  /// Return private field `id`
  pub fn id(&self) -> usize {
    self.id
  }
  /// Return private field `content`
  pub fn content(&self) -> &T {
    &self.content
  }
}

impl<T> fmt::Display for Message<Option<T>>
where
  T: fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.content {
      Some(content) => write!(f, "{}", content),
      None => write!(f, "No content"),
    }
  }
}
