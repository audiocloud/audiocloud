use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post, put};
use axum::{middleware, Extension, Router};

use api::auth::Auth;

use crate::service::Service;
use crate::ws_socket;

use self::auth::*;
use self::instances::*;
use self::tasks::*;
use self::users::*;

pub mod auth;
pub mod instances;
pub mod tasks;
pub mod users;

pub fn rest_api(router: Router<Service>, service: Service) -> Router<Service> {
  let auth_layer = || middleware::from_fn_with_state(service.clone(), auth);

  router.route("/api/v1/instances/:filter/specs",
               get(list_instances_handler).route_layer(auth_layer()))
        .route("/api/v1/instances/:id/spec",
               put(set_instance_spec_handler).route_layer(auth_layer()))
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

async fn web_socket(State(service): State<Service>,
                    ws: WebSocketUpgrade,
                    Extension(auth): Extension<Auth>,
                    ConnectInfo(remote): ConnectInfo<SocketAddr>)
                    -> impl IntoResponse {
  ws.on_upgrade(move |socket| ws_socket::handle_socket(service, socket, remote, auth))
}
