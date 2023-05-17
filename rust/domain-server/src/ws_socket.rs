use std::error::Error;
use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures::channel::mpsc;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::{select, spawn};
use tracing::{instrument, warn};

use api::auth::Auth;
use api::rt::{RtEvent, RtRequest};

use crate::rt_socket::run_socket;
use crate::service::Service;

#[instrument(skip(service, web_socket))]
pub async fn handle_socket(service: Service, web_socket: WebSocket, from: SocketAddr, auth: Auth) {
  let (mut tx_ws, mut rx_ws) = web_socket.split();
  let (tx_rt_evt, mut rx_rt_evt) = mpsc::channel(0xff);
  let (mut tx_rt_cmd, rx_rt_cmd) = mpsc::channel(0xff);

  spawn(run_socket(service.clone(), rx_rt_cmd, tx_rt_evt));

  loop {
    select! {
      message = rx_ws.next() => match message {
        | Some(message) => if !handle_web_socket_message(&mut tx_rt_cmd, message).await { break; },
        | None => break,
      },
      event = rx_rt_evt.next() => match event {
        | Some(event) => handle_rt_event(&mut tx_ws, event).await,
        | None => break,
      },
      else => break,
    }
  }
}

async fn handle_web_socket_message(tx_rt_cmd: &mut mpsc::Sender<RtRequest>, message: Result<Message, impl Error>) -> bool {
  let message = match message {
    | Ok(message) => message,
    | Err(err) => {
      warn!("Error while receiving message: {}", err);
      return false;
    }
  };

  match message {
    | Message::Text(message) => {
      let Ok(message) = serde_json::from_str::<RtRequest>(&message) else { return true; };
      if let Err(_) = tx_rt_cmd.send(message).await {
        return false;
      }
    }
    | Message::Binary(_) | Message::Ping(_) | Message::Pong(_) => {}
    | Message::Close(_) => {
      return false;
    }
  }

  true
}

async fn handle_rt_event(tx_ws: &mut SplitSink<WebSocket, Message>, event: RtEvent) {
  let Ok(encoded) = serde_json::to_string(&event) else { return; };
  let message = Message::Text(encoded);
  let _ = tx_ws.send(message).await;
}
