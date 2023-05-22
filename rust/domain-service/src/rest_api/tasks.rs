use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use api::auth::Auth;
use api::task::graph::AudioGraphSpec;
use api::task::{
  CreateTaskRequest, DesiredTaskPlayState, ModifyTaskGraphRequest, SetTaskInstancesRequest, SetTaskSettingsRequest, SetTaskTimeRequest,
};

use crate::service::Service;

pub async fn create_task_handler(State(service): State<Service>,
                                 Extension(auth): Extension<Auth>,
                                 Path(id): Path<String>,
                                 Json(create): Json<CreateTaskRequest>)
                                 -> impl IntoResponse {
  if !matches!(&auth, Auth::User(_)) {
    return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
  }

  service.create_task(auth, id, create)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn get_task_summary_handler(State(service): State<Service>,
                                      Extension(auth): Extension<Auth>,
                                      Path(id): Path<String>)
                                      -> impl IntoResponse {
  service.get_task_summary(auth, id)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn set_task_graph_handler(State(service): State<Service>,
                                    Extension(auth): Extension<Auth>,
                                    Path(id): Path<String>,
                                    Json(graph): Json<AudioGraphSpec>)
                                    -> impl IntoResponse {
  service.set_task_graph(auth, id, graph)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn set_task_time_handler(State(service): State<Service>,
                                   Extension(auth): Extension<Auth>,
                                   Path(id): Path<String>,
                                   Json(set_time): Json<SetTaskTimeRequest>)
                                   -> impl IntoResponse {
  service.set_task_time(auth, id, set_time)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn set_task_instances_handler(State(service): State<Service>,
                                        Extension(auth): Extension<Auth>,
                                        Path(id): Path<String>,
                                        Json(instances): Json<SetTaskInstancesRequest>)
                                        -> impl IntoResponse {
  service.set_task_instances(auth, id, instances)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn modify_task_graph_handler(State(service): State<Service>,
                                       Extension(auth): Extension<Auth>,
                                       Path(id): Path<String>,
                                       Json(modify): Json<ModifyTaskGraphRequest>)
                                       -> impl IntoResponse {
  service.modify_task_graph(auth, id, modify)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn set_task_control_handler(State(service): State<Service>,
                                      Extension(auth): Extension<Auth>,
                                      Path(id): Path<String>,
                                      Json(control): Json<DesiredTaskPlayState>)
                                      -> impl IntoResponse {
  service.set_task_control(auth, id, control)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn set_task_settings_handler(State(service): State<Service>,
                                       Extension(auth): Extension<Auth>,
                                       Path(id): Path<String>,
                                       Json(settings): Json<SetTaskSettingsRequest>)
                                       -> impl IntoResponse {
  service.set_task_settings(auth, id, settings)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn delete_task_handler(State(service): State<Service>,
                                 Extension(auth): Extension<Auth>,
                                 Path(id): Path<String>)
                                 -> impl IntoResponse {
  service.delete_task(auth, id)
         .await
         .map(Json)
         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
