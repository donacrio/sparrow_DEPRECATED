//! Implementation of the RESP protocol used by Sparrow project.
//!
//! You can find the specifications of the RESP protocol at: https://redis.io/topics/protocol

mod constants;
mod data;
mod deserialize;
mod serialize;

pub use data::Data;
pub use deserialize::{decode, decode_string};
pub use serialize::{encode, encode_string};
