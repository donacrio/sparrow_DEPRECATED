//! Constant values used in this lib.

// Data types first byte
pub const ARRAY_FIRST_BYTE: &[u8] = b"*";
pub const BULK_STRING_FIRST_BYTE: &[u8] = b"$";
pub const ERROR_FIRST_BYTE: &[u8] = b"-";
pub const INTEGER_FIRST_BYTE: &[u8] = b":";
pub const SIMPLE_STRING_FIRST_BYTE: &[u8] = b"+";

// Carriage Return Line Feed
pub const CRLF_BYTES: &[u8] = b"\r\n";
pub const CR_BYTE: u8 = b'\r';
pub const LF_BYTE: u8 = b'\n';

// Null bytes
pub const NULL_BYTES: &[u8] = b"$-1\r\n";
pub const NULL_ARRAY_BYTES: &[u8] = b"*-1\r\n";

// Bulk Strings size
pub const RESPONSE_MAX_SIZE: i64 = 512 * 1024 * 1024;
