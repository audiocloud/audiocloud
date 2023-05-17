use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

use api::auth::Auth;
use api::task::{CreateTaskRequest, CreateTaskResponse};

use crate::service::Service;

pub async fn create_task_handler(State(service): State<Service>,
                                 Extension(auth): Extension<Auth>,
                                 Json(create): Json<CreateTaskRequest>)
                                 -> Result<Json<CreateTaskResponse>, (StatusCode, String)> {
  if !matches!(&auth, Auth::User(_)) {
    return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
  }

  service.create_task(create)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
