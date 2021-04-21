//! Rust enum representation of data types used by the RESP protocol.
use crate::serialize::encode;

// Data types used by the RESP protocol
pub enum Data {
  Array(Vec<Data>),
  BulkString(String),
  Error(Box<dyn std::error::Error>),
  Integer(i64),
  Null,
  NullArray,
  SimpleString(String),
}

impl Data {
  pub fn encode(&self) -> Vec<u8> {
    encode(self)
  }
}
