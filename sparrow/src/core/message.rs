//! Generic DTO to pass message.

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
