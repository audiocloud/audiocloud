/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::time::Instant;

use actix::{fut, Actor, ActorFutureExt, ContextFutureSpawner, MailboxError, WrapFuture};
use futures::TryFutureExt;
use tracing::*;

use audiocloud_api::domain::streaming::{DomainClientMessage, DomainServerMessage};
use audiocloud_api::domain::DomainError;
use audiocloud_api::{Codec, MsgPack};

use crate::sockets::supervisor::SocketContext;
use crate::sockets::{SocketReceived, SocketsSupervisor};
use crate::tasks::{get_tasks_supervisor, messages};
use crate::{to_serializable, DomainSecurity, ResponseMedia};

impl SocketsSupervisor {
    #[instrument(skip_all)]
    pub fn on_socket_message_received(&mut self, message: SocketReceived, ctx: &mut <Self as Actor>::Context) {
        let (request, socket_id, use_json) = match message {
            SocketReceived::Bytes(socket_id, bytes) => match MsgPack.deserialize::<DomainClientMessage>(bytes.as_ref()) {
                Ok(request) => (request, socket_id, false),
                Err(error) => {
                    warn!(%error, %socket_id, "Failed to decode message, dropping socket");
                    self.remove_socket(&socket_id);
                    return;
                }
            },
            SocketReceived::Text(socket_id, text) => match serde_json::from_str(&text) {
                Ok(request) => (request, socket_id, true),
                Err(error) => {
                    warn!(%error, %socket_id, "Failed to decode message, dropping socket");
                    self.remove_socket(&socket_id);
                    return;
                }
            },
        };

        trace!(?request, %socket_id, use_json, "Received");

        let socket = match self.clients.get_mut(&socket_id.client_id) {
            None => {
                warn!(%socket_id, "Received message from unknown client, dropping message");
                return;
            }
            Some(client) => match client.sockets.get_mut(&socket_id.socket_id) {
                None => {
                    warn!(%socket_id, "Received message from unknown socket, dropping message");
                    return;
                }
                Some(socket) => socket,
            },
        };

        let response_media = if use_json { ResponseMedia::Json } else { ResponseMedia::MsgPack };

        match request {
            DomainClientMessage::RequestModifyTaskSpec { request_id,
                                                         task_id,
                                                         modify_spec,
                                                         optional,
                                                         revision, } => {
                // TODO: get security
                let security = DomainSecurity::Cloud;
                let task_fut = get_tasks_supervisor().send(messages::ModifyTask { modify_spec,
                                                                                  security,
                                                                                  task_id,
                                                                                  revision,
                                                                                  optional: false });
                task_fut.map_err(bad_gateway)
                        .and_then(fut::ready)
                        .into_actor(self)
                        .map(move |res, actor, ctx| {
                            let result = to_serializable(res);
                            let result = DomainServerMessage::ModifyTaskSpecResponse { request_id, result };
                            let _ = actor.send_to_socket_by_id(&socket_id, result, response_media, ctx);
                        })
                        .spawn(ctx);
            }
            DomainClientMessage::RequestPeerConnection { request_id } => {
                let request = SocketContext { socket_id,
                                              request_id,
                                              media: ResponseMedia::MsgPack };

                self.request_peer_connection(request, ctx);
            }
            DomainClientMessage::AnswerPeerConnection { socket_id: rtc_socket_id,
                                                        request_id,
                                                        answer, } => {
                let request = SocketContext { socket_id,
                                              request_id,
                                              media: ResponseMedia::MsgPack };

                self.on_peer_connection_remote_answer(request, rtc_socket_id, answer, ctx);
            }
            DomainClientMessage::SubmitPeerConnectionCandidate { request_id,
                                                                 socket_id: rtc_socket_id,
                                                                 candidate, } => {
                let request = SocketContext { socket_id,
                                              request_id,
                                              media: ResponseMedia::Json };

                self.submit_peer_connection_candidate(request, rtc_socket_id, candidate, ctx);
            }
            DomainClientMessage::RequestAttachToTask { request_id,
                                                       task_id,
                                                       secure_key, } => {
                let secure_key_is_valid =
                    matches!(self.security.get(&task_id), Some(track_security) if track_security.security.contains_key(&secure_key));

                let result = if secure_key_is_valid {
                    let socket_id = socket_id.clone();
                    self.clients
                        .entry(socket_id.client_id.clone())
                        .or_default()
                        .memberships
                        .insert(task_id, secure_key);

                    Ok(())
                } else {
                    Err(DomainError::AuthenticationFailed)
                };

                let response = DomainServerMessage::AttachToTaskResponse { request_id,
                                                                           result: to_serializable(result) };

                let _ = self.send_to_socket_by_id(&socket_id, response, response_media, ctx);
            }
            DomainClientMessage::RequestDetachFromTask { request_id, task_id } => {
                let result = match self.clients
                                       .get_mut(&socket_id.client_id)
                                       .and_then(|client| client.memberships.remove(&task_id))
                {
                    Some(_) => Ok(()),
                    None => Err(DomainError::TaskNotFound { task_id }),
                };

                let response = DomainServerMessage::DetachFromTaskResponse { request_id,
                                                                             result: to_serializable(result) };

                let _ = self.send_to_socket_by_id(&socket_id, response, response_media, ctx);
            }
            DomainClientMessage::Pong { challenge, response } => {
                socket.last_pong_at = Instant::now();
            }
        }
    }
}

fn bad_gateway(error: MailboxError) -> DomainError {
    DomainError::BadGateway { error: error.to_string() }
}
