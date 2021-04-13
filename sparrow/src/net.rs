use crate::core::commands::parse_command;
use crate::core::{EngineInput, EngineOutput};
use crate::logger::BACKSPACE_CHARACTER;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::collections::VecDeque;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

const MAX_CONNECTIONS: usize = 256;

pub async fn run_tcp_server<'a>(
  address: &str,
  sender: mpsc::Sender<EngineInput>,
  bus: &Arc<Mutex<bus::Bus<EngineOutput>>>,
) -> Result<(), Box<dyn Error + 'a>> {
  // Queue used to give an unique id
  let mut available_ids: VecDeque<usize> = VecDeque::with_capacity(MAX_CONNECTIONS);
  for i in 0..MAX_CONNECTIONS {
    available_ids.push_back(i);
  }
  let address: SocketAddr = address.parse()?;

  let service = make_service_fn(move |socket: &AddrStream| {
    let socket_address = socket.remote_addr();
    let sender = sender.clone();
    let bus = bus.clone();

    // TODO: if no id then return error code with max connections
    let socket_id = available_ids.pop_front().unwrap();

    let response = async move {
      Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
        let sender = sender.clone();
        let receiver = bus.lock().unwrap().add_rx();
        handle_request(req, socket_address, socket_id, sender, receiver)
      }))
    };

    available_ids.push_back(socket_id);
    response
  });

  let server = Server::bind(&address).serve(service);

  if let Err(e) = server.await {
    log::error!("{}", e)
  }
  Ok(())
}

async fn handle_request(
  req: Request<Body>,
  socket_address: SocketAddr,
  socket_id: usize,
  sender: mpsc::Sender<EngineInput>,
  mut receiver: bus::BusReader<EngineOutput>,
) -> Result<Response<Body>, hyper::Error> {
  log::trace!(
    "{}[{}] Parsing request body",
    BACKSPACE_CHARACTER,
    socket_address
  );
  let body = hyper::body::to_bytes(req.into_body()).await?;
  // TODO: respond with error code
  let body = std::str::from_utf8(&body).unwrap();
  // TODO: respond with error code
  let command = parse_command(body.trim_end()).unwrap();
  log::trace!(
    "{}[{}] Parsed request body",
    BACKSPACE_CHARACTER,
    socket_address
  );
  // TODO: respond with error code
  sender.send(EngineInput::new(socket_id, command)).unwrap();

  loop {
    for output in receiver.iter() {
      if output.id() == socket_id {
        return Ok(Response::new(Body::from(format!("{:?}", output.content()))));
      }
    }
  }
}
