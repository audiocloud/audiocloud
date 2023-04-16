use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::StreamMap;
use tracing::{error, info, instrument, warn};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::{APIBuilder, API};
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;
use api::instance::spec::InstanceSpec;
use api::rt::{RtCommand, RtEvent, RtRequest};

use crate::nats::{EventStream, WatchStream};
use crate::service::Service;

lazy_static! {
  static ref WEBRTC_API: API = {
    // Create a MediaEngine object to configure the supported codec
    let mut m = MediaEngine::default();

    // Register default codecs
    m.register_default_codecs().expect("Registering default codecs");

    // Create a InterceptorRegistry. This is the user configurable RTP/RTCP Pipeline.
    // This provides NACKs, RTCP Reports and other features. If you use `webrtc.NewPeerConnection`
    // this is enabled by default. If you are manually managing You MUST create a InterceptorRegistry
    // for each PeerConnection.
    let mut registry = Registry::new();

    // Use the default set of Interceptors
    registry = register_default_interceptors(registry, &mut m).expect("Registering default interceptors");

    // Create the API object with the MediaEngine
    APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build()
  };
}

enum Internal {
  Event(RtEvent),
  DetachPeerConnection,
}

#[instrument(skip(service, web_socket))]
pub async fn handle_socket(service: Service, web_socket: WebSocket, from: SocketAddr) {
  use Internal::*;

  let (mut tx, mut rx) = web_socket.split();

  let mut instance_events = StreamMap::<String, EventStream<InstanceDriverEvent>>::new();
  let mut instance_specs = StreamMap::<String, WatchStream<InstanceSpec>>::new();

  // TODO: we can make this faster by spawning request handlers and piping response to tx_internal.
  let (tx_internal, mut rx_internal) = mpsc::channel::<Internal>(0x100);

  let mut rtc_pc: Option<RTCPeerConnection> = None;
  let mut rtc_dc: Option<Arc<RTCDataChannel>> = None;

  loop {
    select! {
      Some((instance_id, (_, event))) = instance_events.next(), if !instance_events.is_empty() => {
        let _ = tx_internal.send(Event(RtEvent::InstanceDriverEvent { instance_id, event })).await;
      },
      Some((instance_id, (_, spec))) = instance_specs.next(), if !instance_specs.is_empty() => {
        let _ = tx_internal.send(Event(RtEvent::SetInstanceSpec { instance_id, spec })).await;
      },
      Some(internal) = rx_internal.recv() => {
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
          | DetachPeerConnection => {
            if let (Some(pc), Some(dc)) = (rtc_pc.take(), rtc_dc.take()) {
              // TODO: spawn a task to detach peer connection.
            }
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
            let response = match service.set_instance_parameters(request).await {
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

              let stream = service.subscribe_to_instance_specs(&instance_id);
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
            rtc_pc = match WEBRTC_API.new_peer_connection(default_rtc_configuration()).await {
              | Ok(pc) => {
                let tx_detach = tx_internal.clone();
                let mut once = false;

                pc.on_peer_connection_state_change(Box::new(move |state| {
                  if state == RTCPeerConnectionState::Connected && !once {
                    let _ = tx_detach.try_send(DetachPeerConnection);
                    once = true;
                  }

                  Box::pin(async {})
                }));

                match pc.create_data_channel("data", default_data_channel_init()).await {
                  | Ok(dc) => {
                    let mut gather_complete = pc.gathering_complete_promise().await;
                    let _ = gather_complete.recv().await;

                    let Ok(offer) = pc.create_offer(None).await else { continue; };
                    let Ok(_) = pc.set_local_description(offer.clone()).await else { continue; };

                    let _ = tx_internal.send(Event(RtEvent::OfferPeerConnection { offer: serde_json::to_string(&offer).unwrap() })).await;

                    rtc_dc = Some(dc)
                  },
                  | Err(err) => {
                    warn!(?err, "failed to create data channel: {err}");
                    continue;
                  }
                }

                Some(pc)
              },
              | Err(err) => {
                warn!(?err, "failed to create peer connection: {err}");
                None
              }
            }
          },
          | RtCommand::OfferPeerConnectionCandidate { candidate } => {
            if let Some(pc) = &rtc_pc {
              if let Ok(parsed) = serde_json::from_str::<RTCIceCandidateInit>(&candidate) {
                let _ = pc.add_ice_candidate(parsed).await;
              }
            }
          }
          | RtCommand::AcceptPeerConnection { offer } => {
            if let Ok(parsed) = serde_json::from_str::<RTCSessionDescription>(&offer) {
              if let Some(pc) = &rtc_pc {
                let _ = pc.set_remote_description(parsed).await;
              }
            }
          }
        }
      }
    }
  }
}

fn default_rtc_configuration() -> RTCConfiguration {
  let ice_servers = vec![RTCIceServer { urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                                        ..Default::default() }];

  RTCConfiguration { ice_servers,
                     ..Default::default() }
}

fn default_data_channel_init() -> Option<RTCDataChannelInit> {
  Some(RTCDataChannelInit { ordered: Some(true),
                            max_packet_life_time: Some(0),
                            max_retransmits: Some(5),
                            ..Default::default() })
}
