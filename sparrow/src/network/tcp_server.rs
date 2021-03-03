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
use std::net::Shutdown;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// TODO; change this to avoid in-script logging config
// Backspace character used to format logging
const BACKSPACE_CHARACTER: &str = "\x08";

// Setup reserved server token to identify which events are for the TCP server socket
const SERVER: Token = Token(0);

pub fn run_tcp_server(
  address: &str,
  sender: mpsc::Sender<EngineInput>,
  receiver: mpsc::Receiver<EngineOutput>,
) -> Result<()> {
  // Create a new poll instance.
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
  let connections = Arc::new(Mutex::new(HashMap::<Token, (TcpStream, SocketAddr)>::new()));

  log::info!("Server ready to accept connections on at {}", address);

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
    if let Err(err) = handle_incoming_connections(&t1_poll, server, &t1_connections, &sender) {
      println!("{}", err);
    }
  });

  let t2_poll = poll;
  let t2_connections = connections;
  let t2 = std::thread::spawn(move || {
    handle_engine_outcomes(&t2_poll, &t2_connections, &receiver).unwrap()
  });

  t1.join().unwrap();
  t2.join().unwrap();

  Ok(())
}

fn handle_incoming_connections(
  poll: &Arc<Mutex<Poll>>,
  server: TcpListener,
  connections: &Arc<Mutex<HashMap<Token, (TcpStream, SocketAddr)>>>,
  sender: &mpsc::Sender<EngineInput>,
) -> Result<()> {
  // Unique token to identify each incoming connection
  let mut unique_token = Token(SERVER.0 + 1);
  // Create storage for polling new events
  let mut events = Events::with_capacity(128);

  loop {
    {
      // Polling new events
      poll.lock().unwrap().poll(&mut events, None)?;
    }
    for event in events.iter() {
      match event.token() {
        SERVER => handle_server_event(&server, &poll, &mut unique_token, &connections)?,
        _ => handle_client_event(&event, &poll, &connections, &sender)?,
      }
    }
  }
}

fn handle_server_event(
  server: &TcpListener,
  poll: &Arc<Mutex<Poll>>,
  unique_token: &mut Token,
  connections: &Arc<Mutex<HashMap<Token, (TcpStream, SocketAddr)>>>,
) -> Result<()> {
  loop {
    // An event is received for the TCP server socket, which indicates we can accept a connection.
    let (mut connection, address) = match server.accept() {
      Ok((connection, address)) => (connection, address),
      // A `WouldBlock` error means the listener has no more incoming connections
      Err(err) if utils::errors::would_block(&err) => return Ok(()),
      Err(err) => return Err(err.into()),
    };

    log::info!("{}[{}] Connection accepted", BACKSPACE_CHARACTER, address);
    // Create a new unique token
    let token = utils::mio::next_token(unique_token);
    {
      // Register the connection into the polling instance with the unique token and a READABLE interest
      poll
        .lock()
        .unwrap()
        .registry()
        .register(&mut connection, token, Interest::READABLE)?;
    }
    {
      // Insert the new connection into the connections map
      connections
        .lock()
        .unwrap()
        .insert(token, (connection, address));
    }
  }
}

fn handle_client_event(
  event: &Event,
  poll: &Arc<Mutex<Poll>>,
  connections: &Arc<Mutex<HashMap<Token, (TcpStream, SocketAddr)>>>,
  sender: &mpsc::Sender<EngineInput>,
) -> Result<()> {
  let mut connection_alive = true;
  //Event is received for an accepted TCP connection
  if let Some((connection, address)) = connections.lock().unwrap().get_mut(&event.token()) {
    if event.is_readable() {
      // Read data from connection
      connection_alive = match read_connection(connection) {
        // Process information from connection read
        Ok((mut connection_alive, data)) => {
          // Data has been read, a command is sent to the engine
          if let Some(string_command) = data {
            match send_command(&event.token(), string_command, sender) {
              // Command sent to the engine, reregister the connection to be WRITABLE
              Ok(keep_connection_alive) => {
                poll.lock().unwrap().registry().reregister(
                  connection,
                  event.token(),
                  Interest::READABLE.add(Interest::WRITABLE),
                )?;
                connection_alive = keep_connection_alive;
              }
              // Error while sending command, write the error to the client
              Err(err) => {
                log::error!("{}[{}] {}", BACKSPACE_CHARACTER, address, err);
                match connection.write_all(format!("{}", err).as_bytes()) {
                  Ok(_) => {}
                  Err(ref err)
                    if utils::errors::would_block(err) || utils::errors::interrupted(err) => {}
                  Err(err) => return Err(err.into()),
                }
              }
            };
          }
          // Return wether or not the connection is alive
          connection_alive
        }
        // Error occurred while reading, connection is kept alive
        Err(err) => {
          log::error!("{}[{}] {}", BACKSPACE_CHARACTER, address, err);
          true
        }
      };
    }
  }
  // Connection not alive
  if !connection_alive {
    if let Some((mut connection, address)) = connections.lock().unwrap().remove(&event.token()) {
      log::info!(
        "{}[{}] Closing client connection",
        BACKSPACE_CHARACTER,
        address
      );
      connection.shutdown(Shutdown::Both)?;
      poll
        .lock()
        .unwrap()
        .registry()
        .deregister(&mut connection)?;
      log::info!("{}[{}] Connection closed", BACKSPACE_CHARACTER, address);
    }
  };
  Ok(())
}

fn read_connection(connection: &mut TcpStream) -> Result<(bool, Option<Vec<u8>>)> {
  let mut connection_alive = true;
  let mut received_data = vec![0; 4096];
  let mut bytes_read = 0;
  loop {
    match connection.read(&mut received_data[bytes_read..]) {
      Ok(0) => {
        // 0 bytes read means the connection is close
        connection_alive = false;
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
    let received_data = received_data[..bytes_read].to_vec();
    return Ok((true, Some(received_data)));
  }

  if !connection_alive {
    return Ok((false, None));
  }

  Ok((true, None))
}

fn send_command(token: &Token, data: Vec<u8>, sender: &mpsc::Sender<EngineInput>) -> Result<bool> {
  let data = std::str::from_utf8(&data)?;
  match parse_command(data.trim_end())? {
    Some(command) => {
      sender.send(EngineInput::new(token.0, command))?;
      Ok(true)
    }
    None => Ok(false),
  }
}

fn handle_engine_outcomes(
  poll: &Arc<Mutex<Poll>>,
  connections: &Arc<Mutex<HashMap<Token, (TcpStream, SocketAddr)>>>,
  receiver: &mpsc::Receiver<EngineOutput>,
) -> Result<()> {
  loop {
    let output = receiver.recv()?;
    let token = Token(output.id());
    if let Some((connection, _)) = connections.lock().unwrap().get_mut(&token) {
      let data = format!("{:?}\n", output.content());
      match connection.write_all(data.as_bytes()) {
        Ok(_) => {
          poll
            .lock()
            .unwrap()
            .registry()
            .reregister(connection, token, Interest::READABLE)?
        }
        Err(ref err) if utils::errors::would_block(err) || utils::errors::interrupted(err) => {}
        Err(err) => return Err(err.into()),
      }
    }
  }
}
