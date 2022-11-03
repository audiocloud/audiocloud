/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Addr, Message};
use bytes::Bytes;

use audiocloud_api::domain::streaming::DomainServerMessage;
use audiocloud_api::{ClientId, ClientSocketId, SocketId};

use crate::sockets::web_rtc::WebRtcActor;
use crate::sockets::web_sockets::WebSocketActor;
use crate::{DomainResult, ResponseMedia};

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub enum SocketSend {
    Bytes(Bytes),
    Text(String),
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub enum SocketReceived {
    Bytes(ClientSocketId, Bytes),
    Text(ClientSocketId, String),
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult")]
pub struct RegisterWebSocket {
    pub address:   Addr<WebSocketActor>,
    pub socket_id: ClientSocketId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct SocketConnected {
    pub socket_id: ClientSocketId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct RegisterWebRtcSocket {
    pub address: Addr<WebRtcActor>,
    pub id:      ClientSocketId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct Disconnect;

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct SocketReady(SocketId);

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct SendToClient {
    pub client_id: ClientId,
    pub message:   DomainServerMessage,
    pub media:     ResponseMedia,
}
