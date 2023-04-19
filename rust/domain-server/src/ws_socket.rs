use std::error::Error;
use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures::channel::mpsc;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::{select, spawn};
use tracing::{instrument, warn};

use api::rt::{RtCommand, RtEvent, RtRequest};

use crate::rt_socket::run_socket;
use crate::rtc_socket::{new_rtc_socket, FromRtcSocket, ToRtcSocket};
use crate::service::Service;

enum Internal {
  Rtc(FromRtcSocket),
  CreatePeerConnection,
  AcceptPeerConnection { offer: String },
  OfferPeerConnectionCandidate { candidate: String },
}

#[instrument(skip(service, web_socket))]
pub async fn handle_socket(service: Service, web_socket: WebSocket, from: SocketAddr) {
  let (mut tx_ws, mut rx_ws) = web_socket.split();
  let (mut tx_int, mut rx_int) = mpsc::channel(0xff);
  let (tx_rt_evt, mut rx_rt_evt) = mpsc::channel(0xff);
  let (mut tx_rt_cmd, rx_rt_cmd) = mpsc::channel(0xff);
  let mut maybe_rtc: Option<mpsc::Sender<ToRtcSocket>> = None;

  spawn(run_socket(service.clone(), rx_rt_cmd, tx_rt_evt));

  loop {
    select! {
      message = rx_ws.next() => match message {
        | Some(message) => if !handle_web_socket_message(&mut tx_rt_cmd, &mut tx_int, message).await { break; },
        | None => break,
      },
      event = rx_rt_evt.next() => match event {
        | Some(event) => handle_rt_event(&mut tx_ws, event).await,
        | None => break,
      },
      Some(internal) = rx_int.next() => {
        handle_int_message(&service, &mut maybe_rtc, &mut tx_ws, &mut tx_int, internal).await;
      },
      else => break,
    }
  }
}

async fn handle_int_message(service: &Service,
                            maybe_rtc: &mut Option<mpsc::Sender<ToRtcSocket>>,
                            tx_ws: &mut SplitSink<WebSocket, Message>,
                            tx_int: &mut mpsc::Sender<Internal>,
                            message: Internal) {
  match message {
    | Internal::Rtc(rtc) => match rtc {
      | FromRtcSocket::Offer(offer) => {
        handle_rt_event(tx_ws, RtEvent::OfferPeerConnection { offer }).await;
      }
      | FromRtcSocket::IceCandidate(candidate) => {
        handle_rt_event(tx_ws, RtEvent::OfferPeerConnectionCandidate { candidate }).await;
      }
      | FromRtcSocket::Connected =>
        if let Some(mut rtc) = maybe_rtc.take() {
          let _ = rtc.send(ToRtcSocket::Detach).await;
        },
    },
    | Internal::CreatePeerConnection => {
      let (tx_rtc, mut rx_rtc) = new_rtc_socket(service.clone());
      let mut tx_int = tx_int.clone();

      spawn(async move {
        while let Some(msg) = rx_rtc.next().await {
          if let Err(_) = tx_int.send(Internal::Rtc(msg)).await {
            break;
          }
        }
      });

      maybe_rtc.replace(tx_rtc);
    }
    | Internal::AcceptPeerConnection { offer } =>
      if let Some(tx_rtc) = maybe_rtc {
        let _ = tx_rtc.send(ToRtcSocket::Accept(offer)).await;
      },
    | Internal::OfferPeerConnectionCandidate { candidate } =>
      if let Some(tx_rtc) = maybe_rtc {
        let _ = tx_rtc.send(ToRtcSocket::IceCandidate(candidate)).await;
      },
  }
}

async fn handle_web_socket_message(tx_rt_cmd: &mut mpsc::Sender<RtRequest>,
                                   tx_int: &mut mpsc::Sender<Internal>,
                                   message: Result<Message, impl Error>)
                                   -> bool {
  use Internal::*;

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

      match &message.command {
        | RtCommand::CreatePeerConnection => {
          let _ = tx_int.send(CreatePeerConnection).await;
        }
        | RtCommand::AcceptPeerConnection { offer } => {
          let _ = tx_int.send(AcceptPeerConnection { offer: offer.clone() }).await;
        }
        | RtCommand::OfferPeerConnectionCandidate { candidate } => {
          let _ = tx_int.send(OfferPeerConnectionCandidate { candidate: candidate.clone(), }).await;
        }
        | _ =>
          if let Err(_) = tx_rt_cmd.send(message).await {
            return false;
          },
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
