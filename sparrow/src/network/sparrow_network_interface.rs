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

use crate::commands::parse_command;
use crate::core::{EngineInput, SparrowEngineInputs, SparrowEngineOutputs};
use crate::errors::{CommandNotParsableError, PoisonedQueueError, Result, SparrowError};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};

// Setup reserved server token to identify which events are for the TCP server socket
const SERVER: Token = Token(0);

pub fn run_tcp_server(
  address: &str,
  mut engine_inputs: Arc<Mutex<SparrowEngineInputs>>,
  mut engine_outputs: Arc<Mutex<SparrowEngineOutputs>>,
) -> Result<()> {
  // Create a poll instance.
  let poll = Poll::new()?;
  // Create storage for events.
  let events = Events::with_capacity(128);
  // Setup the TCP server socket.
  let addr = address.parse().unwrap();
  let mut server = TcpListener::bind(addr)?;
  // Register the server with poll so we can receive events for it.
  poll
    .registry()
    .register(&mut server, SERVER, Interest::READABLE)?;
  // Map of `Token` -> `TcpStream`.
  let connections = HashMap::<Token, TcpStream>::new();

  println!("Server ready to accept connections on at {}", address);
  handle_connections(
    poll,
    events,
    server,
    connections,
    &mut engine_inputs,
    &mut engine_outputs,
  )
}

fn handle_connections(
  mut poll: Poll,
  mut events: Events,
  server: TcpListener,
  mut connections: HashMap<Token, TcpStream>,
  engine_inputs: &mut Arc<Mutex<SparrowEngineInputs>>,
  engine_outputs: &mut Arc<Mutex<SparrowEngineOutputs>>,
) -> Result<()> {
  // Unique token to identify each incoming connection.
  let mut unique_token = Token(SERVER.0 + 1);
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
          let mut done = false;
          if let Some(mut connection) = connections.get_mut(&token) {
            if event.is_readable() {
              done = handle_readable_connection_event(
                poll.registry(),
                &mut connection,
                event,
                engine_inputs,
              )?;
            }
            if event.is_writable() {
              handle_writable_connection_event(
                poll.registry(),
                &mut connection,
                event,
                engine_outputs,
              )?;
            }
          };
          if done {
            connections.remove(&token);
            engine_outputs
              .lock()
              .map_err(|err| PoisonedQueueError::new(&format!("{}", err)))?
              .remove(&token.0);
          }
        }
      }
    }
  }
}

fn handle_readable_connection_event(
  registry: &Registry,
  connection: &mut TcpStream,
  event: &Event,
  engine_input_queue: &mut Arc<Mutex<SparrowEngineInputs>>,
) -> Result<bool> {
  // If the connection exists we handle it

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
    match handle_command(&event.token(), received_data, engine_input_queue) {
      Ok(_) => registry.reregister(
        connection,
        event.token(),
        Interest::READABLE.add(Interest::WRITABLE),
      )?,
      Err(err) => {
        println!("{}", err);
        match connection.write_all(format!("{}\n", err).as_bytes()) {
          Ok(_) => {}
          Err(ref err) if would_block(err) || interrupted(err) => {}
          // Other errors we'll consider fatal.
          Err(err) => return Err(SparrowError::IOError(err)),
        }
      }
    };
  }

  if connection_closed {
    println!("Connection closed");
    return Ok(true);
  }

  Ok(false)
}

fn handle_command(
  token: &Token,
  received_data: &[u8],
  engine_inputs: &mut Arc<Mutex<SparrowEngineInputs>>,
) -> Result<()> {
  let str_buf = std::str::from_utf8(received_data)
    .map_err(|err| CommandNotParsableError::new(&format!("{}", err)))?;
  let command = parse_command(str_buf.trim_end())?;
  engine_inputs
    .lock()
    .map_err(|err| PoisonedQueueError::new(&format!("{}", err)))?
    .push_back(EngineInput::new(token.0, command));
  Ok(())
}

fn handle_writable_connection_event(
  registry: &Registry,
  connection: &mut TcpStream,
  event: &Event,
  engine_outputs: &mut Arc<Mutex<SparrowEngineOutputs>>,
) -> Result<()> {
  if let Some(output) = engine_outputs
    .lock()
    .map_err(|err| PoisonedQueueError::new(&format!("{}", err)))?
    .remove(&event.token().0)
  {
    let data = format!("{:?}\n", output.output());
    // We can (maybe) write to the connection.
    match connection.write_all(data.as_bytes()) {
      Ok(_) => {
        // After we've written something we'll reregister the connection
        // to only respond to readable events.
        registry.reregister(connection, event.token(), Interest::READABLE)?
      }
      Err(ref err) if would_block(err) || interrupted(err) => {}
      // Other errors we'll consider fatal.
      Err(err) => return Err(SparrowError::IOError(err)),
    }
  } else {
    registry.reregister(connection, event.token(), Interest::WRITABLE)?
  }
  Ok(())
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
