/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use axum::extract::ws::WebSocket;
use bytes::Bytes;
use coerce::actor::message::Message;
use coerce::actor::ActorRef;

use audiocloud_api::domain::streaming::DomainServerMessage;
use audiocloud_api::{ClientId, ClientSocketId, SocketId};

use crate::sockets::web_rtc::WebRtcActor;
use crate::{DomainResult, ResponseMedia};

pub enum SocketSend {
    Bytes(Bytes),
    Text(String),
}

impl Message for SocketSend {
    type Result = ();
}

pub enum SocketReceived {
    Bytes(ClientSocketId, Bytes),
    Text(ClientSocketId, String),
}

impl Message for SocketReceived {
    type Result = ();
}

pub struct RegisterWebSocket {
    pub socket_id: ClientSocketId,
    pub socket:    WebSocket,
}

impl Message for RegisterWebSocket {
    type Result = Result<(), String>;
}

pub struct SendToClient {
    pub client_id: ClientId,
    pub message:   DomainServerMessage,
    pub media:     ResponseMedia,
}
