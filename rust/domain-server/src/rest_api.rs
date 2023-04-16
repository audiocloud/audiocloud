use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use futures::StreamExt;

use crate::service::Service;
use crate::ws_socket;

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
  ws.on_upgrade(move |socket| ws_socket::handle_socket(service, socket, remote))
}
