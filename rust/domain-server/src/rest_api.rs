use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use futures::{SinkExt, StreamExt};
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::StreamMap;
use tracing::{error, info, instrument, warn};

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;
use api::ws::{WsCommand, WsEvent, WsRequest};

use crate::nats::EventStream;
use crate::service::Service;

pub fn rest_api(router: Router<Service>) -> Router<Service> {
  router.route("/api/v1/instances/:filter/specs", get(list_instances))
        .route("/ws", get(web_socket))
}

async fn list_instances(State(service): State<Service>, Path(filter): Path<String>) -> impl IntoResponse {
  service.list_instances(filter)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

async fn web_socket(State(service): State<Service>,
                    ws: WebSocketUpgrade,
                    ConnectInfo(remote): ConnectInfo<SocketAddr>)
                    -> impl IntoResponse {
  ws.on_upgrade(move |socket| handle_socket(service, socket, remote))
}

#[instrument(skip(service, web_socket))]
async fn handle_socket(service: Service, web_socket: WebSocket, from: SocketAddr) {
  let (mut tx, mut rx) = web_socket.split();

  let mut instance_events = StreamMap::<String, EventStream<InstanceDriverEvent>>::new();
  let (tx_internal, mut rx_internal) = mpsc::channel::<WsEvent>(0x100);

  // TODO: we can make this faster by spawning request handlers and piping response to tx_internal.

  loop {
    select! {
      Some((instance_id, (_, event))) = instance_events.next(), if !instance_events.is_empty() => {
        let _ = tx_internal.send(WsEvent::InstanceDriverEvent { instance_id, event }).await;
      },
      Some(event) = rx_internal.recv() => {
        let Ok(event) = serde_json::to_string(&event) else { continue; };
        if let Err(err) = tx.send(Message::Text(event)).await {
          error!(?err, "failed to send message, bailing: {err}");
          break;
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

        let Ok(WsRequest{ request_id, command }) = serde_json::from_str::<WsRequest>(&message) else { continue; };

        match command {
          | WsCommand::SetInstancePowerControl { instance_id, power } => {
            let success = if let Err(err) = service.set_instance_power_control(&instance_id, power).await {
              error!(?err, "failed to set instance power control: {err}");
              false
            } else {
              true
            };

            let _ = tx_internal.send(WsEvent::SetInstancePowerControl { request_id, success })
                       .await;
          },
          | WsCommand::SetInstancePlayControl {instance_id, play} => {
            let success = if let Err(err) = service.set_instance_play_control(&instance_id, play).await {
              error!(?err, "failed to set instance play control: {err}");
              false
            } else {
              true
            };

            let _ = tx_internal.send(WsEvent::SetInstancePlayControl { request_id, success })
                       .await;
          }
          | WsCommand::SetInstanceParameters(request) => {
            let response = match service.set_instance_parameters(request).await {
              | Err(err) => {
                error!(?err, "failed to set instance command: {err}");
                SetInstanceParameterResponse::RpcFailure
              }
              | Ok(response) => response
            };

            let _ = tx_internal.send(WsEvent::SetInstanceParameters { request_id, response })
                       .await;
          },
          | WsCommand::SubscribeToInstanceEvents { instance_id } => {
            let success = if !instance_events.contains_key(&instance_id) {
              let stream = service.subscribe_to_instance_events(&instance_id);
              instance_events.insert(instance_id, stream);
              true
            } else {
              false
            };

            let _ = tx_internal.send(WsEvent::SubscribeToInstanceEvents { request_id, success })
                       .await;
          },
          | WsCommand::UnsubscribeFromInstanceEvents {instance_id} => {
            let success = instance_events.remove(&instance_id).is_some();

            let _ = tx_internal.send(WsEvent::UnsubscribeFromInstanceEvents { request_id, success })
                       .await;
          }
        }
      }
    }
  }
}
