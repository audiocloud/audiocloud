#![allow(unused_variables)]

use std::collections::HashMap;
use std::time::Instant;

use actix::{Actor, ActorFutureExt, Context, ContextFutureSpawner, Handler, WrapFuture};
use tracing::*;

use audiocloud_api::domain::streaming::DomainServerMessage::{AnswerPeerConnectionResponse, PeerConnectionResponse};
use audiocloud_api::domain::streaming::PeerConnectionCreated;
use audiocloud_api::domain::DomainError;
use audiocloud_api::newtypes::AppTaskId;
use audiocloud_api::{ClientId, ClientSocketId, RequestId, SecureKey, SerializableResult, TaskSecurity, Timestamped};
use sockets::{SocketActorAddr, SupervisedSocket};

use crate::sockets::web_rtc::{AddRemoteIceCandidate, SetPeerAnswer, WebRtcActor};
use crate::sockets::{get_next_socket_id, SocketId, SocketsOpts};
use crate::{DomainResult, ResponseMedia};

use super::messages::*;

mod handle_task_events;
mod packets;
mod receive;
mod sockets;
mod timers;

pub struct SocketsSupervisor {
    opts:     SocketsOpts,
    clients:  HashMap<ClientId, SupervisedClient>,
    security: HashMap<AppTaskId, TaskSecurity>,
}

#[derive(Debug, Default)]
pub struct SupervisedClient {
    pub sockets:     HashMap<SocketId, SupervisedSocket>,
    pub memberships: HashMap<AppTaskId, SecureKey>,
}

#[derive(Clone, Debug)]
pub struct SocketContext {
    pub socket_id:  ClientSocketId,
    pub request_id: RequestId,
    pub media:      ResponseMedia,
}

impl Actor for SocketsSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register_timers(ctx);
    }
}

impl SocketsSupervisor {
    pub fn new(opts: SocketsOpts) -> Self {
        Self { opts:     { opts },
               clients:  { Default::default() },
               security: { Default::default() }, }
    }

    fn request_peer_connection(&mut self, request: SocketContext, ctx: &mut Context<SocketsSupervisor>) {
        let socket_id = ClientSocketId::new(request.socket_id.client_id.clone(), get_next_socket_id());
        let initiator_socket_id = request.socket_id.clone();
        let opts = self.opts.clone();

        let result = match WebRtcActor::new(socket_id.clone(), initiator_socket_id.clone(), &opts.web_rtc) {
            Ok((actor, local_description)) => {
                let socket = SupervisedSocket { actor_addr:    { SocketActorAddr::WebRtc(actor) },
                                                init_complete: { Timestamped::new(false) },
                                                last_pong_at:  { Instant::now() }, };

                self.clients
                    .entry(initiator_socket_id.client_id.clone())
                    .or_default()
                    .sockets
                    .insert(socket_id.socket_id.clone(), socket);

                let res = PeerConnectionCreated::Created { socket_id:          { socket_id },
                                                           remote_description: { local_description }, };
                SerializableResult::Ok(res)
            }
            Err(error) => {
                warn!(%error, "Failed to create WebRTC actor");
                SerializableResult::Error(DomainError::WebRTCError { error: error.to_string() })
            }
        };

        let response = PeerConnectionResponse { request_id: { request.request_id },
                                                result:     { result }, };

        let _ = self.send_to_socket_by_id(&request.socket_id, response, request.media, ctx);
    }

    fn on_peer_connection_remote_answer(&mut self,
                                        request: SocketContext,
                                        rtc_socket_id: SocketId,
                                        answer: String,
                                        ctx: &mut Context<SocketsSupervisor>) {
        let req_for_err = request.clone();
        let rtc_socket_id = ClientSocketId::new(request.socket_id.client_id.clone(), rtc_socket_id);

        let error = match self.clients.get(&rtc_socket_id.client_id) {
            None => {
                warn!(%rtc_socket_id, "Client not found, dropping message");
                Some(DomainError::SocketNotFound { socket_id: rtc_socket_id })
            }
            Some(client) => match client.sockets.get(&rtc_socket_id.socket_id) {
                None => {
                    warn!(%rtc_socket_id, "Client not found, dropping message");
                    Some(DomainError::SocketNotFound { socket_id: rtc_socket_id })
                }
                Some(socket) => match socket.actor_addr {
                    SocketActorAddr::WebRtc(ref addr) => {
                        addr.send(SetPeerAnswer { answer })
                            .into_actor(self)
                            .map(move |res, actor, ctx| {
                                if res.is_err() {
                                    for client in actor.clients.get_mut(&rtc_socket_id.client_id) {
                                        client.sockets.remove(&rtc_socket_id.socket_id);
                                    }
                                }

                                let result = match res {
                                    Ok(_) => SerializableResult::Ok(()),
                                    Err(err) => SerializableResult::Error(DomainError::WebRTCError { error: err.to_string() }),
                                };

                                let response = AnswerPeerConnectionResponse { request_id: { request.request_id },
                                                                              result:     { result }, };

                                if let Err(error) = actor.send_to_socket_by_id(&request.socket_id, response, request.media, ctx) {
                                    warn!(%error, socket_id = %request.socket_id, "Could not send peer answer");
                                }
                            })
                            .spawn(ctx);
                        None
                    }
                    SocketActorAddr::WebSocket(_) => {
                        warn!(%rtc_socket_id, "Socket is not a WebRTC socket, dropping message");
                        Some(DomainError::SocketNotFound { socket_id: rtc_socket_id })
                    }
                },
            },
        };

        if let Some(error) = error {
            let result = SerializableResult::Error(error);
            let _ = self.send_to_socket_by_id(&req_for_err.socket_id,
                                              AnswerPeerConnectionResponse { request_id: { req_for_err.request_id },
                                                                             result:     { result }, },
                                              request.media,
                                              ctx);
        }
    }

    fn submit_peer_connection_candidate(&mut self,
                                        request: SocketContext,
                                        rtc_socket_id: SocketId,
                                        candidate: Option<String>,
                                        ctx: &mut Context<SocketsSupervisor>) {
        let rtc_socket_id = ClientSocketId::new(request.socket_id.client_id.clone(), rtc_socket_id);

        let result = match self.clients.get(&request.socket_id.client_id) {
            None => {
                warn!(%rtc_socket_id, "Socket not found, dropping message");
                Some(SerializableResult::Error(DomainError::SocketNotFound { socket_id: rtc_socket_id }))
            }
            Some(client) => match client.sockets.get(&rtc_socket_id.socket_id) {
                None => {
                    warn!(%rtc_socket_id, "Socket not found, dropping message");
                    Some(SerializableResult::Error(DomainError::SocketNotFound { socket_id: rtc_socket_id }))
                }
                Some(socket) => match socket.actor_addr {
                    SocketActorAddr::WebRtc(ref addr) => {
                        addr.send(AddRemoteIceCandidate { candidate })
                            .into_actor(self)
                            .map(|res, actor, ctx| {})
                            .spawn(ctx);
                        None
                    }
                    SocketActorAddr::WebSocket(_) => {
                        warn!(%rtc_socket_id, "Socket is not a WebRTC socket, dropping message");
                        Some(SerializableResult::Error(DomainError::SocketNotFound { socket_id: rtc_socket_id }))
                    }
                },
            },
        };

        if let Some(result) = result {
            let _ = self.send_to_socket_by_id(&request.socket_id,
                                              PeerConnectionResponse { request_id: request.request_id,
                                                                       result },
                                              request.media,
                                              ctx);
        }
    }
}

impl Handler<RegisterWebSocket> for SocketsSupervisor {
    type Result = DomainResult;

    fn handle(&mut self, msg: RegisterWebSocket, ctx: &mut Self::Context) -> Self::Result {
        let client = self.clients.entry(msg.socket_id.client_id.clone()).or_default();
        if client.sockets.contains_key(&msg.socket_id.socket_id) {
            return Err(DomainError::SocketExists { socket_id: msg.socket_id.clone(), });
        }

        client.sockets.insert(msg.socket_id.socket_id,
                              SupervisedSocket { actor_addr:    { SocketActorAddr::WebSocket(msg.address) },
                                                 init_complete: { Timestamped::new(true) },
                                                 last_pong_at:  { Instant::now() }, });

        Ok(())
    }
}

impl Handler<SocketConnected> for SocketsSupervisor {
    type Result = ();

    fn handle(&mut self, msg: SocketConnected, ctx: &mut Self::Context) -> Self::Result {
        if let Some(socket) = self.clients
                                  .get_mut(&msg.socket_id.client_id)
                                  .and_then(|client| client.sockets.get_mut(&msg.socket_id.socket_id))
        {
            debug!(id = %msg.socket_id, "Connected");
            socket.init_complete = Timestamped::new(true);
            socket.last_pong_at = Instant::now();
        }
    }
}
