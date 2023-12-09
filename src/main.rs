use futures_util::{FutureExt, StreamExt};
use warp::{Filter, Reply, Rejection};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;

pub mod math {
  pub mod vector3;
}

pub mod graph {
  pub mod graph;
  pub mod parsers;
  pub mod routing;
}

pub mod transit {
  pub mod transit_service;
  pub mod simulation_model;
  pub mod entities {
    pub mod entity;
    pub mod drone;
    pub mod robot;
    pub mod human;
    pub mod helicopter;
    pub mod package;
  }
  pub mod factory;
  pub mod strategy;
}

use transit::transit_service::TransitServer;

// https://tms-dev-blog.com/build-basic-rust-websocket-server/

#[derive(Debug, Clone)]
pub struct Client {
  pub client_id: String,
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<warp::ws::Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 3 {
    print!("Usage: cargo run <port> web");
    return;
  }
  let port = match args[1].parse::<u16>() {
    Ok(v) => v,
    Err(e) => {
      println!("{} is not a valid port: {}", args[1], e);
      return;
    }
  };
  let web_dir = args[2].clone();
  let websocket_con = warp::ws()
    .and(warp::any())
    .and_then(handle_connection)
    .with(warp::cors().allow_any_origin());
  warp::serve(websocket_con.or(warp::fs::dir(web_dir)))
    .run(([127, 0, 0, 1], port))
    .await;
}

async fn handle_connection(ws: warp::ws::Ws) -> std::result::Result<impl Reply, Rejection> {
  let mut resp = ws
    .on_upgrade(serve)
    .into_response();
  resp.headers_mut().append("Sec-WebSocket-Protocol", "web_server".parse().unwrap());
  Ok(resp)
}

async fn serve(websocket: WebSocket) {
  println!("establishing client connection...");
  let (ws_sink, mut ws_stream) = websocket.split();
  let (sink, stream) = mpsc::unbounded_channel();
  let stream = UnboundedReceiverStream::new(stream);
  tokio::task::spawn(stream.forward(ws_sink).map(|result| {
    if let Err(e) = result {
      println!("error sending websocket msg: {}", e);
    }
  }));
  let uuid = Uuid::new_v4().simple().to_string();
  let new_client = Client {
    client_id: uuid.clone(),
    sender: Some(sink),
  };
  let mut server: TransitServer = TransitServer::new(&new_client);
  while let Some(result) = ws_stream.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        println!("error receiving message for id {}): {}", uuid.clone(), e);
        break;
      }
    };
    if let Ok(message) = msg.to_str() {
      server.recieve_message(message);
    }
  }
  println!("client disconnected");
}
