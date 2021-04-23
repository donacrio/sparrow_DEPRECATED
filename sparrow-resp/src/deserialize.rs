//! Deserialization utilities for the RESP protocol.

use crate::constants::{
  ARRAY_FIRST_BYTE, BULK_STRING_FIRST_BYTE, CR_BYTE, ERROR_FIRST_BYTE, INTEGER_FIRST_BYTE, LF_BYTE,
  RESPONSE_MAX_SIZE, SIMPLE_STRING_FIRST_BYTE,
};
use crate::data::Data;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Result};

/// Decode a given string in the RESP format.
pub fn decode_string(content: String) -> Result<Data> {
  decode(&mut BufReader::new(content.as_bytes()))
}

/// Decode a given bytes buffer in the RESP format into an [Data] enum member.
///
/// [Data]: crate::Data
pub fn decode<R: Read>(reader: &mut BufReader<R>) -> Result<Data> {
  let mut buff = Vec::<u8>::new();
  reader.read_until(LF_BYTE, &mut buff)?;

  if buff.len() < 3 {
    return Err(Error::new(
      ErrorKind::InvalidInput,
      format!("Input is too short: {}", buff.len()),
    ));
  }

  if !is_crlf(buff[buff.len() - 2], buff[buff.len() - 1]) {
    return Err(Error::new(
      ErrorKind::InvalidInput,
      format!(
        "Invalid CRLF: {}{}",
        buff[buff.len() - 2],
        buff[buff.len() - 1]
      ),
    ));
  }

  let bytes = &buff[1..buff.len() - 2];
  match buff[0] {
    ARRAY_FIRST_BYTE => {
      let n_bytes = parse_integer(bytes)?;

      if n_bytes == -1 {
        return Ok(Data::NullArray);
      }

      if n_bytes < -1 || n_bytes > RESPONSE_MAX_SIZE {
        return Err(Error::new(
          ErrorKind::InvalidData,
          format!("Data is too large: {} > {}", n_bytes, RESPONSE_MAX_SIZE),
        ));
      }

      let mut array = Vec::<Data>::with_capacity(n_bytes as usize);
      for _ in 0..n_bytes {
        let data = decode(reader)?;
        array.push(data);
      }

      Ok(Data::Array(array))
    }
    BULK_STRING_FIRST_BYTE => {
      let n_bytes = parse_integer(bytes)?;

      if n_bytes == -1 {
        return Ok(Data::Null);
      }

      if n_bytes < -1 || n_bytes > RESPONSE_MAX_SIZE {
        return Err(Error::new(
          ErrorKind::InvalidData,
          format!("Data is too large: {} > {}", n_bytes, RESPONSE_MAX_SIZE),
        ));
      }

      let mut string_buff: Vec<u8> = vec![0; n_bytes as usize + 2];
      reader.read_exact(string_buff.as_mut_slice())?;
      if !is_crlf(
        string_buff[string_buff.len() - 2],
        string_buff[string_buff.len() - 1],
      ) {
        return Err(Error::new(
          ErrorKind::InvalidInput,
          format!(
            "Invalid CRLF: {}{}",
            buff[buff.len() - 2],
            buff[buff.len() - 1]
          ),
        ));
      }
      parse_string(&string_buff[..string_buff.len() - 2]).map(Data::BulkString)
    }
    ERROR_FIRST_BYTE => parse_string(bytes).map(Data::Error),
    INTEGER_FIRST_BYTE => parse_integer(bytes).map(Data::Integer),
    SIMPLE_STRING_FIRST_BYTE => parse_string(bytes).map(Data::SimpleString),
    unknown => Err(Error::new(
      ErrorKind::InvalidInput,
      format!("Unknown head character: {}", unknown),
    )),
  }
}

fn is_crlf(x: u8, y: u8) -> bool {
  x == CR_BYTE && y == LF_BYTE
}

fn parse_integer(bytes: &[u8]) -> Result<i64> {
  parse_string(bytes)?.parse::<i64>().map_err(|err| {
    Error::new(
      ErrorKind::InvalidData,
      format!("Cannot parse data: {}", err),
    )
  })
}

fn parse_string(bytes: &[u8]) -> Result<String> {
  String::from_utf8(bytes.to_vec()).map_err(|err| {
    Error::new(
      ErrorKind::InvalidData,
      format!("Cannot parse data: {}", err),
    )
  })
}

#[cfg(test)]
mod tests {
  use crate::data::Data;
  use crate::deserialize::decode_string;
  #[test]
  fn test_decode_array() {
    assert_eq!(
      decode_string(
        "*6\r\n\
    +OK\r\n\
    $24\r\nHi sparrow, how are you?\r\n\
    *3\r\n\
    +OK\r\n\
    $-1\r\n\
    :23\r\n\
    $-1\r\n\
    -An error occurred\r\n\
    *-1\r\n"
          .to_string()
      )
      .unwrap(),
      Data::Array(vec![
        Data::SimpleString("OK".to_string()),
        Data::BulkString("Hi sparrow, how are you?".to_string()),
        Data::Array(vec![
          Data::SimpleString("OK".to_string()),
          Data::Null,
          Data::Integer(23),
        ]),
        Data::Null,
        Data::Error("An error occurred".into()),
        Data::NullArray,
      ])
    );
  }

  #[test]
  fn test_decode_error() {
    assert_eq!(
      decode_string("-An error occurred\r\n".to_string()).unwrap(),
      Data::Error("An error occurred".into())
    );
  }

  #[test]
  fn test_decode_bulk_string() {
    assert_eq!(
      decode_string("$2\r\nOK\r\n".to_string()).unwrap(),
      Data::BulkString("OK".to_string())
    );
  }

  #[test]
  fn test_decode_bulk_string_large() {
    assert_eq!(
      decode_string("$24\r\nHi sparrow, how are you?\r\n".to_string()).unwrap(),
      Data::BulkString("Hi sparrow, how are you?".to_string())
    );
  }

  #[test]
  fn test_decode_integer() {
    assert_eq!(
      decode_string(":23\r\n".to_string()).unwrap(),
      Data::Integer(23)
    );
  }

  #[test]
  fn test_decode_null() {
    assert_eq!(decode_string("$-1\r\n".to_string()).unwrap(), Data::Null);
  }

  #[test]
  fn test_decode_null_array() {
    assert_eq!(
      decode_string("*-1\r\n".to_string()).unwrap(),
      Data::NullArray
    );
  }

  #[test]
  fn test_decode_simple_string() {
    assert_eq!(
      decode_string("+OK\r\n".to_string()).unwrap(),
      Data::SimpleString("OK".to_string())
    );
  }
}
