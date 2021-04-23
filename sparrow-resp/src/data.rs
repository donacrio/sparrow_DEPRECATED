//! Rust enum representation of data types used by the RESP protocol.

/// Enum representation of RESP data types.
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
