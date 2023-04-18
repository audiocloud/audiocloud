use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use tokio::time::sleep;
use tokio::{select, spawn};
use tracing::warn;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::{APIBuilder, API};
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;

use api::rt::RtRequest;

use crate::rt_socket::run_socket;
use crate::service::Service;

use super::Result;

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

pub enum ToRtcSocket {
  IceCandidate(String),
  Accept(String),
  Detach,
}

pub enum FromRtcSocket {
  Offer(String),
  IceCandidate(String),
  Connected,
}

enum Internal {
  IceCandidate(String),
  ConnectionStateChanged(bool),
  Message(bytes::Bytes),
}

pub fn new_rtc_socket(service: Service) -> (mpsc::Sender<ToRtcSocket>, mpsc::Receiver<FromRtcSocket>) {
  let (tx_to_rtc_socket, rx_to_rtc_socket) = mpsc::channel(0xff);
  let (tx_from_rtc_socket, rx_from_rtc_socket) = mpsc::channel(0xff);

  spawn(rtc_socket(service, rx_to_rtc_socket, tx_from_rtc_socket));

  (tx_to_rtc_socket, rx_from_rtc_socket)
}

async fn rtc_socket(service: Service, mut rx: mpsc::Receiver<ToRtcSocket>, mut tx: mpsc::Sender<FromRtcSocket>) -> Result {
  let (tx_internal, mut rx_internal) = mpsc::channel::<Internal>(0xff);

  let Ok(pc) = WEBRTC_API.new_peer_connection(default_peer_connection_configuration()).await else { return Err(anyhow!("Failed to create peer connection")) };
  let Ok(dc) = pc.create_data_channel("data", default_data_channel_init()).await else { return Err(anyhow!("Failed to create data channel")) };

  pc.on_ice_candidate({
      let tx_internal = tx_internal.clone();
      Box::new(move |candidate| {
        let candidate = serde_json::to_string(&candidate).unwrap();

        let mut tx_internal = tx_internal.clone();
        Box::pin(async move {
          let _ = tx_internal.send(Internal::IceCandidate(candidate)).await;
        })
      })
    });

  pc.on_peer_connection_state_change({
      let tx_internal = tx_internal.clone();
      Box::new(move |state| {
        let connected = matches!(&state, RTCPeerConnectionState::Connected);
        let mut tx_internal = tx_internal.clone();

        Box::pin(async move {
          let _ = tx_internal.send(Internal::ConnectionStateChanged(connected)).await;
        })
      })
    });

  dc.on_message({
      let tx_internal = tx_internal.clone();
      Box::new(move |msg| {
        let mut tx_internal = tx_internal.clone();

        Box::pin(async move {
          let _ = tx_internal.send(Internal::Message(msg.data)).await;
        })
      })
    });

  let mut upstream_informed = false;

  loop {
    select! {
      Some(internal) = rx_internal.next() => {
        match internal {
          | Internal::IceCandidate(candidate) => {
            tx.send(FromRtcSocket::IceCandidate(candidate)).await.map_err(|_| anyhow!("Failed to send ice candidate, web socket down?"))?;
          },
          | Internal::ConnectionStateChanged(connected) => {
            if connected && !upstream_informed {
              upstream_informed = true;
              let _ = tx.send(FromRtcSocket::Connected).await;
            } else if !connected && upstream_informed {
              break;
            }
          },
          | Internal::Message(message) => {
            // until we are detached, rq all messages
            let mut tx_internal = tx_internal.clone();

            spawn(async move {
              let _ = sleep(Duration::from_millis(100));
              let _ = tx_internal.send(Internal::Message(message)).await;
            });
          }
        }
      },
      Some(rx) = rx.next(), if !upstream_informed => {
        match rx {
          | ToRtcSocket::Accept(response) => {
            let response = serde_json::from_str::<RTCSessionDescription>(&response)?;
            pc.set_remote_description(response).await?;
          },
          | ToRtcSocket::IceCandidate(candidate) => {
            let candidate  = serde_json::from_str::<RTCIceCandidateInit>(&candidate)?;
            pc.add_ice_candidate(candidate).await?;
          }
          | ToRtcSocket::Detach => {
            // detach
            spawn(detached_socket(service, pc, dc, rx_internal));

            break;
          }
        }
      }
    }
  }

  Ok(())
}

async fn detached_socket(service: Service, pc: RTCPeerConnection, dc: Arc<RTCDataChannel>, mut rx_internal: mpsc::Receiver<Internal>) {
  let (tx_rt_evt, mut rx_rt_evt) = mpsc::channel(0xff);
  let (mut tx_rt_cmd, rx_rt_cmd) = mpsc::channel(0xff);

  spawn(run_socket(service, rx_rt_cmd, tx_rt_evt));

  while pc.ice_connection_state() == RTCIceConnectionState::Checking {
    select! {
      Some(evt) = rx_rt_evt.next() => {
        let Ok(msg) = rmp_serde::to_vec_named(&evt) else { continue };
        if let Err(err) = dc.send(&bytes::Bytes::from(msg)).await {
          warn!(?err, "Failed to send message to peer: {err}");
          break;
        }
      },
      Some(evt) = rx_internal.next() => {
        match evt {
          | Internal::IceCandidate(_) => {},
          | Internal::ConnectionStateChanged(connected) => {
            if !connected {
              break;
            }
          },
          | Internal::Message(message) => {
            let Ok(decoded) = rmp_serde::decode::from_slice::<RtRequest>(&message) else { continue };
            let _ = tx_rt_cmd.send(decoded).await;
          },
        }
      },
      _ = sleep(Duration::from_secs(1)) => {}
    }
  }
}

fn default_peer_connection_configuration() -> RTCConfiguration {
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
