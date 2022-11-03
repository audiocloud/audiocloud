/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::default::Default;
use std::time::Duration;

use actix::{Actor, ActorContext, Addr, AsyncContext, Context, ContextFutureSpawner, Handler, Message, WrapFuture};
use clap::Args;
use datachannel::{
    ConnectionState, DataChannelHandler, DataChannelInfo, DataChannelInit, GatheringState, IceCandidate, PeerConnectionHandler,
    Reliability, RtcConfig, RtcDataChannel, RtcPeerConnection, SessionDescription,
};
use futures::FutureExt;
use tracing::*;

use audiocloud_api::domain::streaming::DomainServerMessage;
use audiocloud_api::ClientSocketId;

use crate::sockets::messages::{SocketReceived, SocketSend};
use crate::sockets::{get_sockets_supervisor, Disconnect, SendToClient, SocketConnected};
use crate::ResponseMedia;

#[derive(Args, Clone, Debug)]
pub struct WebRtcOpts {
    /// Enable WebRTC transport support (used only if the app supports it as well)
    #[clap(long, env)]
    enable_web_rtc: bool,

    /// List of ICE servers to use for WebRTC connections
    #[clap(long, env, default_value = "stun:stun.l.google.com:19302")]
    ice_servers: Vec<String>,

    /// Beginning of UDP port range to use for WebRTC (inclusive)
    #[clap(long, env, default_value = "30000")]
    web_rtc_port_min: u16,

    /// End of UDP port range to use for WebRTC (inclusivec)
    #[clap(long, env, default_value = "40000")]
    web_rtc_port_max: u16,

    /// Maximum number of retransmits before dropping data
    #[clap(long, env, default_value = "16")]
    web_rtc_max_retransmits: u16,

    /// Use native WebRTC ordering of packets instead of reordering in the clients
    #[clap(long, env)]
    web_rtc_use_native_ordering: bool,
}

struct ActorConnectionHandler {
    actor: Addr<WebRtcActor>,
}

impl PeerConnectionHandler for ActorConnectionHandler {
    type DCH = ActorDataChannelHandler;

    fn data_channel_handler(&mut self, _info: DataChannelInfo) -> Self::DCH {
        ActorDataChannelHandler { actor: self.actor.clone() }
    }

    #[instrument(skip_all)]
    fn on_candidate(&mut self, cand: IceCandidate) {
        if let Ok(encoded) = serde_json::to_string(&cand) {
            self.actor.do_send(OnLocalIceCandidate(Some(encoded)))
        }
    }

    #[instrument(skip_all)]
    fn on_gathering_state_change(&mut self, state: GatheringState) {
        match state {
            GatheringState::Complete => {
                debug!("Tracing complete");
                self.actor.do_send(OnLocalIceCandidate(None));
            }
            GatheringState::New => {
                debug!("Tracing started");
            }
            GatheringState::InProgress => {
                debug!("Tracing in progress");
            }
        }
    }

    fn on_connection_state_change(&mut self, state: ConnectionState) {
        match state {
            ConnectionState::Disconnected | ConnectionState::Failed | ConnectionState::Closed => {
                self.actor.do_send(Closed);
            }
            _ => {}
        }
    }
}

struct ActorDataChannelHandler {
    actor: Addr<WebRtcActor>,
}

impl DataChannelHandler for ActorDataChannelHandler {
    fn on_open(&mut self) {
        debug!("DataChannel opened");
        self.actor.do_send(Opened);
    }

    fn on_message(&mut self, msg: &[u8]) {
        debug!("DataChannel received message");
        self.actor.do_send(OnDataChannelMessage(bytes::Bytes::copy_from_slice(msg)));
    }

    fn on_closed(&mut self) {
        debug!("DataChannel closed");
        self.actor.do_send(Closed);
    }
}

pub struct WebRtcActor {
    id:                  ClientSocketId,
    initiator_socket_id: ClientSocketId,
    peer_connection:     Box<RtcPeerConnection<ActorConnectionHandler>>,
    data_channel:        Box<RtcDataChannel<ActorDataChannelHandler>>,
    connected:           bool,
}

impl WebRtcActor {
    pub fn new(id: ClientSocketId, initiator_socket_id: ClientSocketId, opts: &WebRtcOpts) -> anyhow::Result<(Addr<Self>, String)> {
        let config = RtcConfig::new(&opts.ice_servers).enable_ice_tcp()
                                                      .enable_ice_udp_mux()
                                                      .port_range_begin(opts.web_rtc_port_min)
                                                      .port_range_end(opts.web_rtc_port_max);

        let mut reliability = Reliability::default().max_retransmits(opts.web_rtc_max_retransmits);

        if !opts.web_rtc_use_native_ordering {
            reliability = reliability.unordered()
        }

        let data_channel_init = DataChannelInit::default().reliability(reliability);

        let mut local_description = String::new();

        let actor = Self::create({
            let local_description = &mut local_description;
            move |ctx| {
                let mut peer_connection =
                    RtcPeerConnection::new(&config, ActorConnectionHandler { actor: ctx.address() }).expect("Create peer connection");

                let data_channel =
                    peer_connection.create_data_channel_ex("data", ActorDataChannelHandler { actor: ctx.address() }, &data_channel_init)
                                   .expect("Create data channel");

                *local_description = serde_json::to_string(
                    &peer_connection
                        .local_description()
                        .expect("Create local description"),
                )
                .expect("Local description to JSON");

                let connected = false;

                Self { peer_connection,
                       data_channel,
                       id,
                       initiator_socket_id,
                       connected }
            }
        });

        Ok((actor, local_description))
    }
}

impl Actor for WebRtcActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<Closed> for WebRtcActor {
    type Result = ();

    fn handle(&mut self, _: Closed, ctx: &mut Self::Context) {
        debug!("Closing WebRTC connection");
        ctx.stop();
    }
}

impl Handler<OnDataChannelMessage> for WebRtcActor {
    type Result = ();

    fn handle(&mut self, msg: OnDataChannelMessage, ctx: &mut Self::Context) -> Self::Result {
        get_sockets_supervisor().send(SocketReceived::Bytes(self.id.clone(), msg.0))
                                .map(drop)
                                .into_actor(self)
                                .spawn(ctx);
    }
}

impl Handler<OnLocalIceCandidate> for WebRtcActor {
    type Result = ();

    fn handle(&mut self, msg: OnLocalIceCandidate, _ctx: &mut Self::Context) -> Self::Result {
        get_sockets_supervisor().do_send(SendToClient { client_id: self.id.client_id.clone(),
                                                        message:   DomainServerMessage::SubmitPeerConnectionCandidate { socket_id: {
                                                                                                                            self.id
                                                                                                                                .socket_id
                                                                                                                                .clone()
                                                                                                                        },
                                                                                                                        candidate: {
                                                                                                                            msg.0
                                                                                                                        }, },
                                                        media:     ResponseMedia::MsgPack, });
    }
}

impl Handler<SocketSend> for WebRtcActor {
    type Result = ();

    fn handle(&mut self, msg: SocketSend, _ctx: &mut Self::Context) -> Self::Result {
        match (msg, self.connected) {
            (SocketSend::Bytes(bytes), true) => {
                if let Err(error) = self.data_channel.send(&bytes[..]) {
                    warn!(%error, "Failed to send");
                }
            }
            _ => {}
        }
    }
}

impl Handler<SetPeerAnswer> for WebRtcActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: SetPeerAnswer, _ctx: &mut Self::Context) -> Self::Result {
        let answer: SessionDescription = serde_json::from_str(&msg.answer)?;
        debug!(id = %self.id, answer = ?answer.sdp, "Received Peer Answer");

        self.peer_connection.set_remote_description(&answer)?;

        Ok(())
    }
}

impl Handler<Opened> for WebRtcActor {
    type Result = ();

    fn handle(&mut self, _msg: Opened, _ctx: &mut Self::Context) -> Self::Result {
        self.connected = true;
        get_sockets_supervisor().do_send(SocketConnected { socket_id: self.id.clone(), });
    }
}

impl Handler<AddRemoteIceCandidate> for WebRtcActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: AddRemoteIceCandidate, _ctx: &mut Self::Context) -> Self::Result {
        match msg.candidate {
            Some(candidate) => match serde_json::from_str::<IceCandidate>(&candidate) {
                Ok(candidate) => {
                    debug!(id = %self.id, ?candidate, "Add ICE candidate");
                    self.peer_connection.add_remote_candidate(&candidate)?;

                    Ok(())
                }
                Err(error) => {
                    warn!(id = %self.id, %error, "Failed to parse ICE candidate");
                    Err(error.into())
                }
            },
            None => {
                // end of gathering, but libdatachannel can't be informed of that
                Ok(())
            }
        }
    }
}

impl Handler<Disconnect> for WebRtcActor {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        debug!(id = %self.id, "Asked to disconnect");
        ctx.run_later(Duration::default(), |_, ctx| ctx.stop());
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnDataChannelMessage(bytes::Bytes);

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnLocalIceCandidate(Option<String>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Closed;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Opened;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct AddRemoteIceCandidate {
    pub candidate: Option<String>,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct SetPeerAnswer {
    pub answer: String,
}

pub fn init(_opts: &WebRtcOpts) -> anyhow::Result<()> {
    // datachannel::configure_logging();

    Ok(())
}
