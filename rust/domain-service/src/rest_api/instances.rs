use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use api::auth::Auth;
use api::instance::spec::InstanceSpec;

use crate::service::Service;

pub async fn list_instances_handler(Extension(auth): Extension<Auth>,
                                    State(service): State<Service>,
                                    Path(filter): Path<String>)
                                    -> impl IntoResponse {
  service.list_instances(auth, filter)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn set_instance_spec_handler(State(service): State<Service>,
                                       Extension(auth): Extension<Auth>,
                                       Path(id): Path<String>,
                                       Json(spec): Json<InstanceSpec>)
                                       -> impl IntoResponse {
  service.set_instance_spec(auth, id, spec)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
