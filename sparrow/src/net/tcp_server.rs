use crate::core::{EngineInput, EngineOutput};
use async_std::channel::{unbounded, Receiver, Sender};
use async_std::{
  io::BufReader,
  net::{TcpListener, TcpStream, ToSocketAddrs},
  prelude::*,
  task,
};
use std::io::Result;
use std::sync::Arc;

pub fn run_tcp_server(port: u16, engine_sender: Sender<EngineInput>) -> Result<()> {
  task::block_on(accept_loop(format!("127.0.0.1:{}", port), engine_sender))
}

async fn accept_loop(addr: impl ToSocketAddrs, engine_sender: Sender<EngineInput>) -> Result<()> {
  let listener = TcpListener::bind(addr).await?;

  let mut incoming = listener.incoming();
  while let Some(stream) = incoming.next().await {
    let stream = stream?;
    println!("Accepting from: {}", stream.peer_addr()?);
    let engine_sender = engine_sender.clone();
    task::spawn(async move {
      if let Err(e) = connection_loop(stream, engine_sender).await {
        log::error!("{}", e)
      }
    });
  }
  Ok(())
}

async fn connection_loop(stream: TcpStream, engine_sender: Sender<EngineInput>) -> Result<()> {
  let peer_id = stream.peer_addr()?.to_string();
  let stream = Arc::new(stream);
  let reader = BufReader::new(&*stream);

  Ok(())
}
