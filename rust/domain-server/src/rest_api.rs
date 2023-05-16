use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Json, Router};

use crate::service::Service;
use crate::ws_socket;

use self::auth::{auth, login_user_handler, logout_user_handler};

pub mod auth;

pub fn rest_api(router: Router<Service>, service: Service) -> Router<Service> {
  let auth_layer = || middleware::from_fn_with_state(service.clone(), auth);

  router.route("/api/v1/instances/:filter/specs", get(list_instances).route_layer(auth_layer()))
        .route("/api/v1/users/login", post(login_user_handler).route_layer(auth_layer()))
        .route("/api/v1/users/logout", get(logout_user_handler).route_layer(auth_layer()))
        .route("/ws", get(web_socket).route_layer(auth_layer()))
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
