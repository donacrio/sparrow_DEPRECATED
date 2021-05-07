//! Serialization utilities for the RESP protocol.

use crate::constants::{
  ARRAY_FIRST_BYTE, BULK_STRING_FIRST_BYTE, CRLF_BYTES, ERROR_FIRST_BYTE, INTEGER_FIRST_BYTE,
  NULL_ARRAY_BYTES, NULL_BYTES, SIMPLE_STRING_FIRST_BYTE,
};
use crate::data::Data;
use async_std::io::{BufWriter, Write};
use async_std::prelude::*;
use futures::future::BoxFuture;
use std::io::Result;

/// Encode a given [String] by writing it to a [BufWriter].
///
/// # Example
/// ```rust
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io::{BufWriter, Cursor};
/// use async_std::prelude::*;
/// use sparrow_resp::encode_string;
///
/// let input = String::from("Hello Sparrow!");
/// let buffer = Cursor::new(Vec::new());
/// let mut writer = BufWriter::new(buffer);
///
/// encode_string(input, &mut writer).await?;
/// writer.flush().await?;
///
/// #
/// # Ok(()) }) }
/// ```
/// This function is mostly used to encode commands made to the Sparrow engine.
///
/// [BufWriter]: async_std::io::BufWriter
pub async fn encode_string<W>(content: String, writer: &mut BufWriter<W>) -> Result<()>
where
  W: Write + Unpin + Send,
{
  encode(&Data::BulkString(content), writer).await
}

/// Encode a given [Data] enum member by writing it to a [BufWriter].
///
/// # Example
/// ```rust
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io::{BufWriter, Cursor};
/// use async_std::prelude::*;
/// use sparrow_resp::{Data, encode};
///
/// let input = Data::SimpleString(String::from("Hello Sparrow!"));
/// let buffer = Cursor::new(Vec::new());
/// let mut writer = BufWriter::new(buffer);
///
/// encode(&input, &mut writer).await?;
/// writer.flush().await?;
///
/// #
/// # Ok(()) }) }
/// ```
/// This function is mostly used to encode commands made to the Sparrow engine.
///
/// [BufWriter]: async_std::io::BufWriter
pub async fn encode<W>(data: &Data, writer: &mut BufWriter<W>) -> Result<()>
where
  W: Write + Unpin + Send,
{
  encode_inner(data, writer).await
}

/// Encode a given [Data] enum member by writing it to a [BufWriter].
///
/// This function is similar to [decode] and is used to encode the given [Data] recursively.
///
/// [Data]: crate::Data
/// [encode]: crate::serialize::encode
fn encode_inner<'a, W>(data: &'a Data, writer: &'a mut BufWriter<W>) -> BoxFuture<'a, Result<()>>
where
  W: Write + Unpin + Send,
{
  Box::pin(async move {
    match data {
      Data::Array(array) => {
        writer.write(ARRAY_FIRST_BYTE).await?;
        writer.write(array.len().to_string().as_bytes()).await?;
        writer.write(CRLF_BYTES).await?;
        for data in array.iter() {
          encode_inner(data, writer).await?;
        }
      }
      Data::BulkString(data) => {
        writer.write(BULK_STRING_FIRST_BYTE).await?;
        writer
          .write(data.as_bytes().len().to_string().as_bytes())
          .await?;
        writer.write(CRLF_BYTES).await?;
        writer.write(data.as_bytes()).await?;
        writer.write(CRLF_BYTES).await?;
      }
      Data::Error(err) => {
        writer.write(ERROR_FIRST_BYTE).await?;
        writer.write(err.to_string().as_bytes()).await?;
        writer.write(CRLF_BYTES).await?;
      }
      Data::Integer(data) => {
        writer.write(INTEGER_FIRST_BYTE).await?;
        writer.write(data.to_string().as_bytes()).await?;
        writer.write(CRLF_BYTES).await?;
      }
      Data::Null => {
        writer.write(NULL_BYTES).await?;
      }
      Data::NullArray => {
        writer.write(NULL_ARRAY_BYTES).await?;
      }
      Data::SimpleString(data) => {
        writer.write(SIMPLE_STRING_FIRST_BYTE).await?;
        writer.write(data.to_string().as_bytes()).await?;
        writer.write(CRLF_BYTES).await?;
      }
    };
    Ok(())
  })
}

#[cfg(test)]
mod tests {
  use crate::data::Data;
  use crate::serialize::encode;
  use async_std::io::BufWriter;

  #[async_std::test]
  async fn test_encode_array() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(
      &Data::Array(vec![
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
      ]),
      &mut writer,
    )
    .await
    .unwrap();

    assert_eq!(
      writer.buffer(),
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
        .as_bytes()
        .to_vec()
    );
  }

  #[async_std::test]
  async fn test_encode_error() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(&Data::Error("An error occurred".into()), &mut writer)
      .await
      .unwrap();
    assert_eq!(writer.buffer(), "-An error occurred\r\n".as_bytes());
  }

  #[async_std::test]
  async fn test_encode_bulk_string() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(&Data::BulkString("OK".to_string()), &mut writer)
      .await
      .unwrap();
    assert_eq!(writer.buffer(), "$2\r\nOK\r\n".as_bytes());
  }

  #[async_std::test]
  async fn test_encode_bulk_string_large() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(
      &Data::BulkString("Hi sparrow, how are you?".to_string()),
      &mut writer,
    )
    .await
    .unwrap();
    assert_eq!(
      writer.buffer(),
      "$24\r\nHi sparrow, how are you?\r\n".as_bytes()
    );
  }

  #[async_std::test]
  async fn test_encode_integer() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(&Data::Integer(23), &mut writer).await.unwrap();
    assert_eq!(writer.buffer(), ":23\r\n".as_bytes());
  }

  #[async_std::test]
  async fn test_encode_null() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(&Data::Null, &mut writer).await.unwrap();
    assert_eq!(writer.buffer(), "$-1\r\n".as_bytes());
  }

  #[async_std::test]
  async fn test_encode_null_array() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(&Data::NullArray, &mut writer).await.unwrap();
    assert_eq!(writer.buffer(), "*-1\r\n".as_bytes());
  }

  #[async_std::test]
  async fn test_encode_simple_string() {
    let mut writer = BufWriter::new(Vec::<u8>::new());
    encode(&Data::SimpleString("OK".to_string()), &mut writer)
      .await
      .unwrap();
    assert_eq!(writer.buffer(), "+OK\r\n".as_bytes());
  }
}
