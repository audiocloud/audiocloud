use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use tokio::select;
use futures::channel::mpsc;
use tokio_stream::StreamMap;
use tracing::{error, info, instrument, warn};

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;
use api::instance::spec::InstanceSpec;
use api::rt::{RtCommand, RtEvent, RtRequest};

use crate::nats::{EventStream, WatchStreamMap};
use crate::rtc_socket::{FromRtcSocket, ToRtcSocket};
use crate::service::Service;

enum Internal {
  Event(RtEvent),
  Rtc(FromRtcSocket),
}

#[instrument(skip(service, web_socket))]
pub async fn handle_socket(service: Service, web_socket: WebSocket, from: SocketAddr) {
  use Internal::*;

  let (mut tx, mut rx) = web_socket.split();

  let mut instance_events = StreamMap::<String, EventStream<InstanceDriverEvent>>::new();
  let mut instance_specs = WatchStreamMap::<String, InstanceSpec>::new();

  // TODO: we can make this faster by spawning request handlers and piping response to tx_internal.
  let (mut tx_internal, mut rx_internal) = mpsc::channel::<Internal>(0x100);

  let mut tx_maybe_rtc: Option<mpsc::Sender<ToRtcSocket>> = None;

  loop {
    select! {
      Some((instance_id, (_, event))) = instance_events.next(), if !instance_events.is_empty() => {
        let _ = tx_internal.send(Event(RtEvent::InstanceDriverEvent { instance_id, event })).await;
      },
      Some((instance_id, (_, spec))) = instance_specs.next(), if !instance_specs.is_empty() => {
        let _ = tx_internal.send(Event(RtEvent::SetInstanceSpec { instance_id, spec })).await;
      },
      Some(internal) = rx_internal.next() => {
        match internal {
          | Event(event) => {
              let event = match serde_json::to_string(&event) {
                Ok(event) => event,
                Err(err) => {
                  error!(?err, ?event, "failed to serialize web socket event: {err}");
                  continue;
                }
              };

              if let Err(err) = tx.send(Message::Text(event)).await {
                error!(?err, "failed to send message, bailing: {err}");
                break;
              }
            }
          | Rtc(event) => {
            // TODO: ...
          }
        }
      },
      message = rx.next() => {
        let Some(message) = message else { break; };
        let message = match message {
          Ok(message) => message,
          Err(err) => {
            warn!(?err, "failed to receive message, bailing: {err}");
            break;
          }
        };

        let message = match message {
          | Message::Close(_) => {
            info!("closed by remote, bailing");
            break;
          }
          | Message::Text(message) => { message }
          | _ => {
            continue;
          }
        };

        let Ok(RtRequest{ request_id, command }) = serde_json::from_str::<RtRequest>(&message) else { continue; };

        match command {
          | RtCommand::SetInstancePowerControl { instance_id, power } => {
            let success = if let Err(err) = service.set_instance_power_control(&instance_id, power).await {
              error!(?err, "failed to set instance power control: {err}");
              false
            } else {
              true
            };

            let _ = tx_internal.send(Event(RtEvent::SetInstancePowerControl { request_id, success }))
                       .await;
          },
          | RtCommand::SetInstancePlayControl {instance_id, play} => {
            let success = if let Err(err) = service.set_instance_play_control(&instance_id, play).await {
              error!(?err, "failed to set instance play control: {err}");
              false
            } else {
              true
            };

            let _ = tx_internal.send(Event(RtEvent::SetInstancePlayControl { request_id, success }))
                       .await;
          }
          | RtCommand::SetInstanceParameters(request) => {
            let response = match service.set_instance_parameters(&request.instance_id, request.changes).await {
              | Err(err) => {
                error!(?err, "failed to set instance command: {err}");
                SetInstanceParameterResponse::RpcFailure
              }
              | Ok(response) => response
            };

            let _ = tx_internal.send(Event(RtEvent::SetInstanceParameters { request_id, response }))
                       .await;
          },
          | RtCommand::SubscribeToInstanceEvents { instance_id } => {
            let success = if !instance_events.contains_key(&instance_id) {
              let stream = service.subscribe_to_instance_events(&instance_id);
              instance_events.insert(instance_id.clone(), stream);

              let stream = service.watch_instance_specs(&instance_id);
              instance_specs.insert(instance_id.clone(), stream);

              true
            } else {
              false
            };

            let _ = tx_internal.send(Event(RtEvent::SubscribeToInstanceEvents { request_id, success }))
                       .await;
          },
          | RtCommand::UnsubscribeFromInstanceEvents {instance_id} => {
            let success = true && instance_events.remove(&instance_id).is_some();
            let success = success && instance_specs.remove(&instance_id).is_some();

            let _ = tx_internal.send(Event(RtEvent::UnsubscribeFromInstanceEvents { request_id, success }))
                       .await;
          },
          | RtCommand::CreatePeerConnection => {
            // TODO: ...
          },
          | RtCommand::OfferPeerConnectionCandidate { candidate } => {
          }
          | RtCommand::AcceptPeerConnection { offer } => {
          }
        }
      }
    }
  }
}
