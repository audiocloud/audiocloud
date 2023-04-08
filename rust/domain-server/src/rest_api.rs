use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use futures::TryFutureExt;

use crate::service::Service;

pub fn rest_api(router: Router<Service>) -> Router<Service> {
  router.route("/api/v1/instances/:filter/specs", get(list_instances))
}

async fn list_instances(State(service): State<Service>, Path(filter): Path<String>) -> impl IntoResponse {
  service.list_instances(filter)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
