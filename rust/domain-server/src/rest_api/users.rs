use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

use api::auth::Auth;
use api::user::{RegisterUserRequest, RegisterUserResponse, UpdateUserRequest, UpdateUserResponse, UserSummary};

use crate::service::Service;

pub async fn users_summary_handler(State(service): State<Service>,
                                   Extension(auth): Extension<Auth>)
                                   -> Result<Json<Vec<UserSummary>>, (StatusCode, String)> {
  if !matches!(&auth, Auth::User(_)) {
    return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
  }

  service.list_users()
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn whoami_handler(Extension(user): Extension<Auth>) -> Json<Auth> {
  Json(user)
}

pub async fn register_user_handler(Extension(auth): Extension<Auth>,
                                   State(service): State<Service>,
                                   Json(register): Json<RegisterUserRequest>)
                                   -> Result<Json<RegisterUserResponse>, (StatusCode, String)> {
  if !matches!(&auth, Auth::User(_)) {
    return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
  }

  service.register_user(register)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn update_user_handler(Extension(auth): Extension<Auth>,
                                 State(service): State<Service>,
                                 Path(id): Path<String>,
                                 Json(update): Json<UpdateUserRequest>)
                                 -> Result<Json<UpdateUserResponse>, (StatusCode, String)> {
  if !matches!(&auth, Auth::User(_)) {
    return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
  }

  service.update_user(id, update)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
