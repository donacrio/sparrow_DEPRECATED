//! Rust enum representation of data types used by the RESP protocol.

/// Data types used by the RESP protocol.
#[derive(Debug, PartialEq)]
pub enum Data {
  Array(Vec<Data>),
  BulkString(String),
  Error(String),
  Integer(i64),
  Null,
  NullArray,
  SimpleString(String),
}
