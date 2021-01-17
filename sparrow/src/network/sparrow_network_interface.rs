// Copyright [2020] [Donatien Criaud]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::commands::{parse_command, Command};
use crate::core::{EngineInput, SparrowEngineInputQueue, SparrowEngineOutputQueue};
use crate::errors::{CommandNotParsableError, PoisonedQueueError, Result, SparrowError};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};

// Setup reserved server token to identify which events are for the TCP server socket
const SERVER: Token = Token(0);

pub struct SparrowNetworkInterface {
  address: String,
  engine_input_queue: Arc<Mutex<SparrowEngineInputQueue>>,
  engine_output_queue: Arc<Mutex<SparrowEngineOutputQueue>>,
}

impl SparrowNetworkInterface {
  pub fn new(
    address: &str,
    engine_input_queue: Arc<Mutex<SparrowEngineInputQueue>>,
    engine_output_queue: Arc<Mutex<SparrowEngineOutputQueue>>,
  ) -> SparrowNetworkInterface {
    SparrowNetworkInterface {
      address: address.to_string(),
      engine_input_queue,
      engine_output_queue,
    }
  }
}

impl SparrowNetworkInterface {
  pub fn run_tcp_server(&mut self) -> Result<()> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    let addr = self.address.parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // Register the server with poll so we can receive events for it.
    poll
      .registry()
      .register(&mut server, SERVER, Interest::READABLE)?;

    // Map of `Token` -> `TcpStream`.
    let mut connections = HashMap::<Token, TcpStream>::new();
    // Unique token to identify each incoming connection.
    let mut unique_token = Token(SERVER.0 + 1);

    println!("Server ready to accept connections on at {}", self.address);
    loop {
      poll.poll(&mut events, None)?;

      for event in events.iter() {
        match event.token() {
          SERVER => loop {
            // Received an event for the TCP server socket, which
            // indicates we can accept a connection.
            let (mut connection, address) = match server.accept() {
              Ok((connection, address)) => (connection, address),
              Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // If we get a `WouldBlock` error we know our
                // listener has no more incoming connections queued,
                // so we can return to polling and wait for some
                // more.
                break;
              }
              Err(e) => {
                // If it was any other kind of error, something went
                // wrong and we terminate with an error.
                return Err(SparrowError::IOError(e));
              }
            };

            println!("Accepted connection from: {}", address);

            let token = next(&mut unique_token);
            poll
              .registry()
              .register(&mut connection, token, Interest::READABLE)?;

            connections.insert(token, connection);
          },
          token => {
            // Maybe received an event for a TCP connection.
            let done = if let Some(mut connection) = connections.get_mut(&token) {
              self.handle_connection_event(&token, &mut connection, event)?
            } else {
              // Sporadic events happen, we can safely ignore them.
              false
            };
            if done {
              connections.remove(&token);
            }
          }
        }
      }
    }
  }

  fn handle_connection_event(
    &self,
    token: &Token,
    connection: &mut TcpStream,
    event: &Event,
  ) -> Result<bool> {
    // If the connection exists we handle it

    if event.is_readable() {
      let mut connection_closed = false;
      let mut received_data = vec![0; 4096];
      let mut bytes_read = 0;
      loop {
        match connection.read(&mut received_data[bytes_read..]) {
          Ok(0) => {
            // Read 0 bytes so the connection is closed
            connection_closed = true;
            break;
          }
          Ok(n) => {
            bytes_read += n;
            if bytes_read == received_data.len() {
              received_data.resize(received_data.len() + 1024, 0);
            }
          }
          Err(ref err) if would_block(err) => break,
          Err(ref err) if interrupted(err) => continue,
          Err(err) => return Err(SparrowError::IOError(err)),
        }
      }

      if bytes_read != 0 {
        let received_data = &received_data[..bytes_read];
        if let Err(err) = self.handle_command(&token, received_data) {
          println!("{}", err);
          connection.write(format!("{}", err).as_bytes())?;
        }
      }

      if connection_closed {
        println!("Connection closed");
        return Ok(true);
      }
    }

    Ok(false)
  }

  fn handle_command(&self, token: &Token, received_data: &[u8]) -> Result<()> {
    let str_buf = std::str::from_utf8(received_data)
      .map_err(|err| CommandNotParsableError::new(&format!("{}", err)))?;
    let command = parse_command(str_buf.trim_end())?;
    self
      .engine_input_queue
      .lock()
      .map_err(|err| PoisonedQueueError::new(&format!("{}", err)))?
      .push_back(EngineInput::new(token.0, command));
    Ok(())
  }
}

fn next(current: &mut Token) -> Token {
  let next = current.0;
  current.0 += 1;
  Token(next)
}

fn would_block(err: &io::Error) -> bool {
  err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
  err.kind() == io::ErrorKind::Interrupted
}
