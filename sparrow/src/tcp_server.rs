use crate::core::EngineInput;
use crate::errors::Result;
use crate::logger::BACKSPACE_CHARACTER;
use async_std::channel::{unbounded, Sender};
use async_std::io::{BufReader, BufWriter};
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::prelude::*;
use async_std::task;
use sparrow_resp::{decode, encode, Data};

use std::sync::Arc;

pub fn run_tcp_server(port: u16, engine_sender: Sender<EngineInput>) -> Result<()> {
  task::block_on(accept_loop(format!("127.0.0.1:{}", port), engine_sender))
}

async fn accept_loop(addr: impl ToSocketAddrs, engine_sender: Sender<EngineInput>) -> Result<()> {
  let listener = TcpListener::bind(addr).await?;

  let mut incoming = listener.incoming();
  while let Some(stream) = incoming.next().await {
    let stream = stream?;
    log::info!("Accepting from: {}", stream.peer_addr()?);
    let engine_sender = engine_sender.clone();
    task::spawn(async move {
      if let Err(err) = connection_loop(stream, engine_sender).await {
        log::error!("{}", err);
      }
    });
  }
  Ok(())
}

async fn connection_loop(stream: TcpStream, engine_sender: Sender<EngineInput>) -> Result<()> {
  let id = stream.peer_addr()?.to_string();
  let (sender, receiver) = unbounded();

  let stream = Arc::new(stream);
  let mut reader = BufReader::new(&*stream);
  let mut writer = BufWriter::new(&*stream);
  loop {
    let output = match decode(&mut reader).await {
      Ok(input) => {
        let id = id.clone();
        log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, id, input);
        let sender = sender.clone();
        let input = EngineInput::new(id, input, sender);
        engine_sender.send(input).await?;
        let output = receiver.recv().await?;
        output
      }
      Err(err) => {
        log::error!("{}[{}] {}", BACKSPACE_CHARACTER, id, err);
        Data::Error(format!("{}", err))
      }
    };
    log::info!("{}[{}] {:?}", BACKSPACE_CHARACTER, id, output);
    encode(&output, &mut writer).await?;
    writer.flush().await?;
  }
}
