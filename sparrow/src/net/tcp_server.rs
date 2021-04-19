//! Network TCP interface managing incoming requests.

use crate::core::commands::parse_command;
use crate::core::{EngineInput, EngineOutput};
use crate::logger::BACKSPACE_CHARACTER;
use crate::net::Error;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::VecDeque;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// Run the TCP server on the given address.
///
/// [`mpsc::Sender`] is a producer used to send messages to the [`sparrow::Engine`] thread.
/// [`bus::Bus`] is a broadcaster used to retrieve messages from [`sparrow::Engine`] thread.
///
/// [`mpsc::Sender`]: std::sync::mpsc::Sender
/// [`bus::Bus`]: bus::Bus
///
/// # Examples
/// ```rust
/// async {
///   use sparrow::net::run_tcp_server;
///   use sparrow::core::Engine;
///
///   let mut engine = Engine::new();
///   let (sender, bus) = engine.init(256);
///
///   std::thread::spawn(move || engine.run().unwrap());
///   run_tcp_server("127.0.0.1:8080".parse().unwrap(), 256, sender, &bus).await.unwrap();
/// };
/// ```
pub async fn run_tcp_server(
  port: u16,
  max_connections: usize,
  sender: mpsc::Sender<EngineInput>,
  bus: &Arc<Mutex<bus::Bus<EngineOutput>>>,
) -> Result<(), Box<dyn std::error::Error>> {
  // Queue is used to store unique available ids
  let mut available_ids: VecDeque<usize> = VecDeque::with_capacity(max_connections);
  for i in 0..max_connections {
    available_ids.push_back(i);
  }

  // Create the hyper service that will handle HTTP requests
  let service = make_service_fn(move |socket: &AddrStream| {
    // Sender can be cloned and shared across threads
    let sender = sender.clone();
    // Bus reference must be cloned to be passed across threads
    let bus = bus.clone();

    // Get connected socket information
    let socket_address = socket.remote_addr();
    let socket_id = available_ids.pop_front();

    let response = async move {
      Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
        let sender = sender.clone();
        let receiver = bus.lock().unwrap().add_rx();
        handle_request(req, socket_address, socket_id, sender, receiver)
      }))
    };

    // The used socket id is now available
    if let Some(socket_id) = socket_id {
      available_ids.push_back(socket_id);
    }

    response
  });

  let address = SocketAddr::from(([127, 0, 0, 1], port));
  let server = Server::bind(&address).serve(service);

  server.await?;

  Ok(())
}

/// Handle an HTTP request made to the TCP server.
///
/// This function only handles operations related to HTTP requests and responses.
async fn handle_request(
  req: Request<Body>,
  socket_address: SocketAddr,
  socket_id: Option<usize>,
  sender: mpsc::Sender<EngineInput>,
  receiver: bus::BusReader<EngineOutput>,
) -> Result<Response<Body>, hyper::Error> {
  match socket_id {
    // If socket_id is `Some` then we can process the request
    Some(socket_id) => match (req.method(), req.uri().path()) {
      // Only serving this entrypoint
      (&Method::GET, "/") => {
        match process_request(req, socket_address, socket_id, sender, receiver).await {
          Ok(response) => Ok(response),
          Err(err) => {
            log::error!("{:?}", err);
            Ok(
              Response::builder()
                .status(err.status_code())
                .body(Body::from(format!("{}", err.error())))
                .unwrap(),
            )
          }
        }
      }
      // Other requests are unknown
      _ => Ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body(Body::empty())
          .unwrap(),
      ),
    },
    // If socket_id is `None` that means there is too many alive connections
    None => Ok(
      Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .body(Body::from(
          "Too many requests. Please close unused alive connections",
        ))
        .unwrap(),
    ),
  }
}

/// Process a unique request.
async fn process_request(
  req: Request<Body>,
  socket_address: SocketAddr,
  socket_id: usize,
  sender: mpsc::Sender<EngineInput>,
  mut receiver: bus::BusReader<EngineOutput>,
) -> Result<Response<Body>, Error> {
  log::trace!(
    "{}[{}] Parsing request body",
    BACKSPACE_CHARACTER,
    socket_address
  );
  let body = hyper::body::to_bytes(req.into_body())
    .await
    .map_err(|err| Error::new(StatusCode::BAD_REQUEST, Box::new(err)))?;
  let body =
    std::str::from_utf8(&body).map_err(|err| Error::new(StatusCode::BAD_REQUEST, Box::new(err)))?;
  let command = parse_command(body).map_err(|err| Error::new(StatusCode::BAD_REQUEST, err))?;

  log::trace!(
    "{}[{}] Parsed request body",
    BACKSPACE_CHARACTER,
    socket_address
  );
  sender
    .send(EngineInput::new(socket_id, command))
    .map_err(|err| Error::new(StatusCode::INTERNAL_SERVER_ERROR, Box::new(err)))?;

  loop {
    for output in receiver.iter() {
      if output.id() == socket_id {
        return Response::builder()
          .status(StatusCode::OK)
          .body(Body::from(format!("{}", output)))
          .map_err(|err| Error::new(StatusCode::INTERNAL_SERVER_ERROR, Box::new(err)));
      }
    }
  }
}
