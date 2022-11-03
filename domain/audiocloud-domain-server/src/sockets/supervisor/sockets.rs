/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::time::{Duration, Instant};

use actix::{Actor, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use anyhow::anyhow;
use derive_more::IsVariant;
use futures::FutureExt;
use itertools::Itertools;

use tracing::*;

use audiocloud_api::domain::streaming::DomainServerMessage;
use audiocloud_api::{ClientId, ClientSocketId, Codec, MsgPack, Timestamped};

use crate::sockets::web_rtc::WebRtcActor;
use crate::sockets::web_sockets::WebSocketActor;
use crate::sockets::{Disconnect, SendToClient, SocketReceived, SocketSend, SocketsSupervisor};
use crate::ResponseMedia;

#[derive(Debug)]
pub struct SupervisedSocket {
    pub actor_addr:    SocketActorAddr,
    pub last_pong_at:  Instant,
    pub init_complete: Timestamped<bool>,
}

impl SupervisedSocket {
    pub(crate) fn score(&self) -> usize {
        match self.actor_addr {
            SocketActorAddr::WebRtc(_) => 10,
            SocketActorAddr::WebSocket(_) => 1,
        }
    }
}

impl Drop for SupervisedSocket {
    fn drop(&mut self) {
        match &self.actor_addr {
            SocketActorAddr::WebRtc(socket) => socket.do_send(Disconnect),
            SocketActorAddr::WebSocket(socket) => socket.do_send(Disconnect),
        };
    }
}

#[derive(Clone, Debug, IsVariant)]
pub enum SocketActorAddr {
    WebRtc(Addr<WebRtcActor>),
    WebSocket(Addr<WebSocketActor>),
}

impl SupervisedSocket {
    #[instrument(skip(self))]
    pub fn is_valid(&self, socket_drop_timeout: u64) -> bool {
        let since_last_pong = self.last_pong_at.elapsed();
        let last_pong_too_old = since_last_pong >= Duration::from_millis(socket_drop_timeout);

        if last_pong_too_old {
            debug!(?since_last_pong, "Last pong too old");
            false
        } else {
            let connected = match &self.actor_addr {
                SocketActorAddr::WebRtc(addr) => addr.connected(),
                SocketActorAddr::WebSocket(addr) => addr.connected(),
            };

            if !connected {
                debug!("Host actor is disconnected");
            }

            connected
        }
    }

    #[instrument(skip(self))]
    pub fn is_init_timed_out(&self, max_init_wait_time: u64) -> bool {
        if *self.init_complete.get_ref() {
            false
        } else {
            let since_init_started = self.init_complete.elapsed();
            let init_started_too_old = since_init_started > chrono::Duration::milliseconds(max_init_wait_time as i64);

            if init_started_too_old {
                debug!(%since_init_started, "Init timed out");
            }

            init_started_too_old
        }
    }
}

impl SocketsSupervisor {
    #[instrument(skip(self, ctx), err)]
    pub(crate) fn send_to_socket_by_id(&mut self,
                                       id: &ClientSocketId,
                                       message: DomainServerMessage,
                                       media: ResponseMedia,
                                       ctx: &mut <Self as Actor>::Context)
                                       -> anyhow::Result<()> {
        debug!(?message, %id, "send");

        match self.clients.get(&id.client_id) {
            None => {}
            Some(client) => match client.sockets.get(&id.socket_id) {
                None => warn!(%id, ?message, "Socket not found, dropping message"),
                Some(socket) => self.send_to_socket(socket, message, media, ctx)?,
            },
        }

        Ok(())
    }

    #[instrument(skip_all, err)]
    pub(crate) fn send_to_socket(&self,
                                 socket: &SupervisedSocket,
                                 message: DomainServerMessage,
                                 media: ResponseMedia,
                                 ctx: &mut Context<SocketsSupervisor>)
                                 -> anyhow::Result<()> {
        let cmd = match media {
            ResponseMedia::MsgPack => SocketSend::Bytes(MsgPack.serialize(&message)?.into()),
            ResponseMedia::Json => SocketSend::Text(serde_json::to_string(&message)?),
        };

        match &socket.actor_addr {
            SocketActorAddr::WebRtc(web_rtc) => {
                debug!(?cmd, "sending to WebRTC socket");
                web_rtc.send(cmd).map(drop).into_actor(self).spawn(ctx);
            }
            SocketActorAddr::WebSocket(web_socket) => {
                debug!(?cmd, "sending to WebSocket socket");
                web_socket.send(cmd).map(drop).into_actor(self).spawn(ctx);
            }
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub(crate) fn remove_socket(&mut self, id: &ClientSocketId) {
        if let Some(_) = self.clients
                             .get_mut(&id.client_id)
                             .and_then(|client| client.sockets.remove(&id.socket_id))
        {
            debug!("removed");
        } else {
            warn!("did not exist");
        }
    }

    #[instrument(skip(self, ctx, msg), err)]
    pub(crate) fn send_to_client(&self, client_id: &ClientId, msg: DomainServerMessage, ctx: &mut Context<Self>) -> anyhow::Result<()> {
        if let Some(client) = self.clients.get(client_id) {
            let best_socket = client.sockets
                                    .values()
                                    .filter(|socket| *socket.init_complete.get_ref())
                                    .filter(|socket| socket.is_valid(self.opts.socket_drop_timeout))
                                    .sorted_by_key(|socket| socket.score())
                                    .next();

            if let Some(socket) = best_socket {
                if let Err(error) = self.send_to_socket(socket, msg, ResponseMedia::MsgPack, ctx) {
                    warn!(%error, "Failed to send to client's best socket");
                }

                Ok(())
            } else {
                Err(anyhow!("No valid socket for client {client_id} found"))
            }
        } else {
            Err(anyhow!("Client {client_id} not found"))
        }
    }
}

impl Handler<SocketReceived> for SocketsSupervisor {
    type Result = ();

    fn handle(&mut self, msg: SocketReceived, ctx: &mut Self::Context) -> Self::Result {
        self.on_socket_message_received(msg, ctx);
    }
}

impl Handler<SendToClient> for SocketsSupervisor {
    type Result = ();

    fn handle(&mut self, msg: SendToClient, ctx: &mut Self::Context) -> Self::Result {
        let SendToClient { client_id, message, media } = msg;

        if let Err(error) = self.send_to_client(&client_id, message, ctx) {
            warn!(%error, "Failed");
        }
    }
}
