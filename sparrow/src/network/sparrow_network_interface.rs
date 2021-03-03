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
use crate::core::{EngineInput, EngineOutput};
use crate::errors::Result;
use crate::utils;
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// Setup reserved server token to identify which events are for the TCP server socket
const SERVER: Token = Token(0);

pub fn run_tcp_server(
  address: &str,
  sender: mpsc::Sender<EngineInput>,
  receiver: mpsc::Receiver<EngineOutput>,
) -> Result<()> {
  // Create a poll instance.
  let poll = Poll::new()?;
  // Setup the TCP server socket.
  let addr = address.parse()?;
  let mut server = TcpListener::bind(addr)?;
  // Register the server with poll so we can receive events for it.
  poll
    .registry()
    .register(&mut server, SERVER, Interest::READABLE)?;
  // Map of `Token` -> `TcpStream`.
  // TODO: Create struct to use message passing instead
  let poll = Arc::new(Mutex::new(poll));
  let connections = Arc::new(Mutex::new(HashMap::<Token, TcpStream>::new()));

  println!("Server ready to accept connections on at {}", address);

  // take_hook() returns the default hook in case when a custom one is not set
  let orig_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    // invoke the default handler and exit the process
    orig_hook(panic_info);
    std::process::exit(1);
  }));

  let t1_poll = poll.clone();
  let t1_connections = connections.clone();
  let t1 = std::thread::spawn(move || {
    handle_incoming_connections(&t1_poll, server, &t1_connections, &sender).unwrap()
  });

  let t2_poll = poll;
  let t2_connections = connections;
  let t2 = std::thread::spawn(move || {
    handle_engine_outcomes(&t2_poll, &t2_connections, receiver).unwrap()
  });

  t1.join().unwrap();
  t2.join().unwrap();

  Ok(())
}

fn handle_incoming_connections(
  poll: &Arc<Mutex<Poll>>,
  server: TcpListener,
  connections: &Arc<Mutex<HashMap<Token, TcpStream>>>,
  sender: &mpsc::Sender<EngineInput>,
) -> Result<()> {
  // Unique token to identify each incoming connection.
  let mut unique_token = Token(SERVER.0 + 1);
  // Create storage for events.
  let mut events = Events::with_capacity(128);
  loop {
    {
      poll.lock().unwrap().poll(&mut events, None)?;
    }
    for event in events.iter() {
      match event.token() {
        SERVER => loop {
          // Received an event for the TCP server socket, which
          // indicates we can accept a connection.
          let (mut connection, address) = match server.accept() {
            Ok((connection, address)) => (connection, address),
            // If we get a `WouldBlock` error we know our
            // listener has no more incoming connections queued,
            // so we can return to polling and wait for some
            // more.
            Err(err) if utils::errors::would_block(&err) => break,
            // If it was any other kind of error, something went
            // wrong and we terminate with an error.
            Err(err) => return Err(err.into()),
          };

          println!("Accepted connection from: {}", address);

          let token = utils::mio::next_token(&mut unique_token);
          {
            poll
              .lock()
              .unwrap()
              .registry()
              .register(&mut connection, token, Interest::READABLE)?;
          }

          {
            connections.lock().unwrap().insert(token, connection);
          }
        },
        token => {
          // Maybe received an event for a TCP connection.
          let mut done = false;
          if let Some(mut connection) = connections.lock().unwrap().get_mut(&token) {
            if event.is_readable() {
              done = handle_readable_connection_event(poll, &mut connection, event, sender)?;
            }
          };
          if done {
            connections.lock().unwrap().remove(&token);
          }
        }
      }
    }
  }
}

fn handle_readable_connection_event(
  poll: &Arc<Mutex<Poll>>,
  connection: &mut TcpStream,
  event: &Event,
  sender: &mpsc::Sender<EngineInput>,
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
      Err(ref err) if utils::errors::would_block(err) => break,
      Err(ref err) if utils::errors::interrupted(err) => continue,
      Err(err) => return Err(err.into()),
    }
  }

  if bytes_read != 0 {
    let received_data = &received_data[..bytes_read];
    match handle_command(&event.token(), received_data, sender) {
      Ok(_) => poll.lock().unwrap().registry().reregister(
        connection,
        event.token(),
        Interest::READABLE.add(Interest::WRITABLE),
      )?,
      Err(err) => {
        println!("{}", err);
        match connection.write_all(format!("{}\n", err).as_bytes()) {
          Ok(_) => {}
          Err(ref err) if utils::errors::would_block(err) || utils::errors::interrupted(err) => {}
          // Other errors we'll consider fatal.
          Err(err) => return Err(err.into()),
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
  sender: &mpsc::Sender<EngineInput>,
) -> Result<()> {
  let str_buf = std::str::from_utf8(received_data)?;
  let command = parse_command(str_buf.trim_end())?;
  //TODO: handle this error
  sender.send(EngineInput::new(token.0, command))?;
  Ok(())
}

fn handle_engine_outcomes(
  poll: &Arc<Mutex<Poll>>,
  connections: &Arc<Mutex<HashMap<Token, TcpStream>>>,
  receiver: mpsc::Receiver<EngineOutput>,
) -> Result<()> {
  loop {
    let output = receiver.recv()?;
    let token = Token(output.id());
    if let Some(connection) = connections.lock().unwrap().get_mut(&token) {
      let data = format!("{:?}\n", output.content());
      // We can (maybe) write to the connection.
      match connection.write_all(data.as_bytes()) {
        Ok(_) => {
          // After we've written something we'll reregister the connection
          // to only respond to readable events.
          poll
            .lock()
            .unwrap()
            .registry()
            .reregister(connection, token, Interest::READABLE)?
        }
        Err(ref err) if utils::errors::would_block(err) || utils::errors::interrupted(err) => {}
        // Other errors we'll consider fatal.
        Err(err) => return Err(err.into()),
      }
    }
  }
}
