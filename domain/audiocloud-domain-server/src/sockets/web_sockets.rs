/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

#![allow(unused_variables)]

use std::time::Duration;

use axum::extract::ws::WebSocket;
use axum::extract::{Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures::{FutureExt, SinkExt, StreamExt};
use serde::Deserialize;
use tracing::*;

use audiocloud_api::newtypes::SecureKey;
use audiocloud_api::{ClientId, ClientSocketId, SocketId};

use crate::sockets::messages::{RegisterWebSocket, SocketReceived, SocketSend};
use crate::sockets::{get_sockets_supervisor, Disconnect, SocketConnectedMsg};
use crate::DomainContext;

pub fn configure(router: Router<DomainContext>) -> Router<DomainContext> {
    router.route("/ws/{client_id}/{socket_id}", get(ws_handler))
}

#[derive(Deserialize)]
struct AuthParams {
    secure_key: SecureKey,
}

async fn ws_handler(Path((client_id, socket_id)): Path<(ClientId, SocketId)>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(ClientSocketId { client_id, socket_id }, socket))
}

async fn handle_socket(id: ClientSocketId, mut socket: WebSocket) {
    let (socket_tx, socket_rx) = socket.split();
    let (cmd_rx, cmd_tx) = tokio::sync::mpsc::channel(256);

    // could also just send the stream to the supervisor and let it handle the rest

    get_sockets_supervisor().send(SocketConnectedMsg {}).await;

    loop {
        tokio::select! {
            msg = socket_rx.next() => match msg {
                None => break,
                Some(msg) => {
                }
            },
            cmd  = cmd_rx.next() => match cmd {
                None => break,
                Some(event) => {
                    // todo; ...
                }
            }
        }
    }

    // Send cleanup
    get_sockets_supervisor().send(SocketDisconnected {}).await;
}
