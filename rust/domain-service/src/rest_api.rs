use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post, put};
use axum::{middleware, Extension, Json, Router};

use api::auth::Auth;

use crate::rest_api::tasks::{
  create_task_handler, delete_task_handler, get_task_summary_handler, modify_task_graph_handler, set_task_control_handler,
  set_task_graph_handler, set_task_instances_handler, set_task_settings_handler, set_task_time_handler,
};
use crate::rest_api::users::{register_user_handler, update_user_handler, users_summary_handler, whoami_handler};
use crate::service::Service;
use crate::ws_socket;

use self::auth::{auth, login_user_handler, logout_user_handler};

pub mod auth;
pub mod tasks;
pub mod users;

pub fn rest_api(router: Router<Service>, service: Service) -> Router<Service> {
  let auth_layer = || middleware::from_fn_with_state(service.clone(), auth);

  router.route("/api/v1/instances/:filter/specs", get(list_instances).route_layer(auth_layer()))
        .route("/api/v1/users/login", post(login_user_handler))
        .route("/api/v1/users/whoami", get(whoami_handler).route_layer(auth_layer()))
        .route("/api/v1/users", get(users_summary_handler).route_layer(auth_layer()))
        .route("/api/v1/users", post(register_user_handler).route_layer(auth_layer()))
        .route("/api/v1/users/:id", patch(update_user_handler).route_layer(auth_layer()))
        .route("/api/v1/users/logout", get(logout_user_handler))
        .route("/api/v1/tasks/:id", post(create_task_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id", delete(delete_task_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/summary", get(get_task_summary_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/graph", put(set_task_graph_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/time", put(set_task_time_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/instances",
               put(set_task_instances_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/graph/modify",
               post(modify_task_graph_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/graph/control",
               put(set_task_control_handler).route_layer(auth_layer()))
        .route("/api/v1/tasks/:id/graph/settings",
               put(set_task_settings_handler).route_layer(auth_layer()))
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
                    Extension(auth): Extension<Auth>,
                    ConnectInfo(remote): ConnectInfo<SocketAddr>)
                    -> impl IntoResponse {
  ws.on_upgrade(move |socket| ws_socket::handle_socket(service, socket, remote, auth))
}
