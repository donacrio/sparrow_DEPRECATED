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

use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};

// Setup reserved server token to identify which events are for the TCP server socket
const SERVER: Token = Token(0);

// Testing Data for demonstrating purposes
// TODO: remove this when fully integrated
const DATA: &[u8] = b"Hello world!\n";

pub struct SparrowNetworkInterface {
  address: String,
}

impl SparrowNetworkInterface {
  pub fn new(address: &str) -> SparrowNetworkInterface {
    SparrowNetworkInterface {
      address: address.to_string(),
    }
  }
}

impl SparrowNetworkInterface {
  pub fn run_tcp_server(&self) -> io::Result<()> {
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
                return Err(e);
              }
            };

            println!("Accepted connection from: {}", address);

            let token = next(&mut unique_token);
            poll.registry().register(
              &mut connection,
              token,
              Interest::READABLE.add(Interest::WRITABLE),
            )?;

            connections.insert(token, connection);
          },
          token => {
            // Maybe received an event for a TCP connection.
            let done = if let Some(connection) = connections.get_mut(&token) {
              handle_connection_event(poll.registry(), connection, event)?
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
}

fn next(current: &mut Token) -> Token {
  let next = current.0;
  current.0 += 1;
  Token(next)
}

/// Returns `true` if the connection is done.
fn handle_connection_event(
  registry: &Registry,
  connection: &mut TcpStream,
  event: &Event,
) -> io::Result<bool> {
  if event.is_writable() {
    // We can (maybe) write to the connection.
    match connection.write(DATA) {
      // We want to write the entire `DATA` buffer in a single go. If we
      // write less we'll return a short write error (same as
      // `io::Write::write_all` does).
      Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
      Ok(_) => {
        // After we've written something we'll reregister the connection
        // to only respond to readable events.
        registry.reregister(connection, event.token(), Interest::READABLE)?
      }
      // Would block "errors" are the OS's way of saying that the
      // connection is not actually ready to perform this I/O operation.
      Err(ref err) if would_block(err) => {}
      // Got interrupted we'll try again.
      Err(ref err) if interrupted(err) => {
        return handle_connection_event(registry, connection, event)
      }
      // Other errors we'll consider fatal.
      Err(err) => return Err(err),
    }
  }

  if event.is_readable() {
    let mut connection_closed = false;
    let mut received_data = vec![0; 4096];
    let mut bytes_read = 0;
    // We can (maybe) read from the connection.
    loop {
      match connection.read(&mut received_data[bytes_read..]) {
        Ok(0) => {
          // Reading 0 bytes means the other side has closed the
          // connection or is done writing, then so are we.
          connection_closed = true;
          break;
        }
        Ok(n) => {
          bytes_read += n;
          if bytes_read == received_data.len() {
            received_data.resize(received_data.len() + 1024, 0);
          }
        }
        // Would block "errors" are the OS's way of saying that the
        // connection is not actually ready to perform this I/O operation.
        Err(ref err) if would_block(err) => break,
        Err(ref err) if interrupted(err) => continue,
        // Other errors we'll consider fatal.
        Err(err) => return Err(err),
      }
    }

    if bytes_read != 0 {
      let received_data = &received_data[..bytes_read];
      if let Ok(str_buf) = std::str::from_utf8(received_data) {
        println!("Received data: {}", str_buf.trim_end());
      } else {
        println!("Received (none UTF-8) data: {:?}", received_data);
      }
    }

    if connection_closed {
      println!("Connection closed");
      return Ok(true);
    }
  }

  Ok(false)
}

fn would_block(err: &io::Error) -> bool {
  err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
  err.kind() == io::ErrorKind::Interrupted
}
