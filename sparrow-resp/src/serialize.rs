use crate::constants::{
  ARRAY_FIRST_BYTE, BULK_STRING_FIRST_BYTE, CRLF_BYTES, ERROR_FIRST_BYTE, INTEGER_FIRST_BYTE,
  NULL_ARRAY_BYTES, NULL_BYTES, SIMPLE_STRING_FIRST_BYTE,
};
use crate::data::Data;

pub fn encode(data: &Data) -> Vec<u8> {
  let mut buff = Vec::<u8>::new();
  buff_encode(data, &mut buff);
  buff
}

fn buff_encode(data: &Data, buff: &mut Vec<u8>) {
  match data {
    Data::Array(array) => {
      buff.extend_from_slice(ARRAY_FIRST_BYTE);
      buff.extend_from_slice(array.len().to_string().as_bytes());
      buff.extend_from_slice(CRLF_BYTES);
      for data in array.iter() {
        buff_encode(data, buff);
      }
    }
    Data::BulkString(data) => {
      buff.extend_from_slice(BULK_STRING_FIRST_BYTE);
      buff.extend_from_slice(data.as_bytes().len().to_string().as_bytes());
      buff.extend_from_slice(CRLF_BYTES);
      buff.extend_from_slice(data.as_bytes());
      buff.extend_from_slice(CRLF_BYTES);
    }
    Data::Error(err) => {
      buff.extend_from_slice(ERROR_FIRST_BYTE);
      buff.extend_from_slice(err.to_string().as_bytes());
      buff.extend_from_slice(CRLF_BYTES);
    }
    Data::Integer(data) => {
      buff.extend_from_slice(INTEGER_FIRST_BYTE);
      buff.extend_from_slice(data.to_string().as_bytes());
      buff.extend_from_slice(CRLF_BYTES);
    }
    Data::Null => {
      buff.extend_from_slice(NULL_BYTES);
    }
    Data::NullArray => {
      buff.extend_from_slice(NULL_ARRAY_BYTES);
    }
    Data::SimpleString(data) => {
      buff.extend_from_slice(SIMPLE_STRING_FIRST_BYTE);
      buff.extend_from_slice(data.to_string().as_bytes());
      buff.extend_from_slice(CRLF_BYTES);
    }
  };
}

#[cfg(test)]
mod tests {
  use crate::data::Data;
  use crate::serialize::encode;
  use std::str::from_utf8;

  #[test]
  fn test_encode_array() {
    let encoded = encode(&Data::Array(vec![
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
    ]));
    assert_eq!(
      from_utf8(&encoded).unwrap(),
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
    );
  }

  #[test]
  fn test_encode_error() {
    assert_eq!(
      encode(&Data::Error("An error occurred".into())),
      "-An error occurred\r\n".as_bytes().to_vec()
    );
  }

  #[test]
  fn test_encode_bulk_string() {
    assert_eq!(
      encode(&Data::BulkString("Hi sparrow, how are you?".to_string())),
      "$24\r\nHi sparrow, how are you?\r\n".as_bytes().to_vec()
    );
  }

  #[test]
  fn test_encode_bulk_string_large() {
    assert_eq!(
      encode(&Data::BulkString("Hi sparrow, how are you?".to_string())),
      "$24\r\nHi sparrow, how are you?\r\n".as_bytes().to_vec()
    );
  }

  #[test]
  fn test_encode_integer() {
    assert_eq!(encode(&Data::Integer(23)), ":23\r\n".as_bytes().to_vec());
  }

  #[test]
  fn test_encode_null() {
    assert_eq!(encode(&Data::Null), "$-1\r\n".as_bytes().to_vec());
  }

  #[test]
  fn test_encode_null_array() {
    assert_eq!(encode(&Data::NullArray), "*-1\r\n".as_bytes().to_vec());
  }

  #[test]
  fn test_encode_simple_string() {
    assert_eq!(
      encode(&Data::SimpleString("OK".to_string())),
      "+OK\r\n".as_bytes().to_vec()
    );
  }
}
