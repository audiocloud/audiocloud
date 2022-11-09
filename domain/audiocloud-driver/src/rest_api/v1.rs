/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use reqwest::StatusCode;

use audiocloud_api::instance_driver::{InstanceDriverError};
use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_api::DesiredInstancePlayState;

use crate::rest_api::DriverState;

pub fn configure(state: DriverState) -> Router<DriverState> {
    Router::with_state(state).route("/instances", get(get_instances))
                             .route("/:manufacturer/:name/:instance", get(get_instance))
                             .route("/:manufacturer/:name/:instance/parameters", post(set_parameters))
                             .route("/:manufacturer/:name/:instance/parameters/:parameter_id", post(set_parameter))
                             .route("/:manufacturer/:name/:instance/play_state", put(set_desired_instance_state))
}

async fn get_instances(State(state): State<DriverState>) -> impl IntoResponse {
    state.instances.get_instances().await.map_err(to_http_error).map(Json)
}

async fn get_instance(State(state): State<DriverState>, Path(path): Path<(String, String, String)>) -> impl IntoResponse {
    let instance_id = get_instance_id(path);

    state.instances.get_instance(&instance_id).await.map_err(to_http_error).map(Json)
}

async fn set_parameters(State(state): State<DriverState>,
                        Path(path): Path<(String, String, String)>,
                        Json(params): Json<serde_json::Value>)
                        -> impl IntoResponse {
    let instance_id = get_instance_id(path);

    state.instances
         .set_parameters(&instance_id, params)
         .await
         .map_err(to_http_error)
         .map(Json)
}

async fn set_parameter(State(state): State<DriverState>,
                       Path(path): Path<(String, String, String, String)>,
                       Json(value): Json<serde_json::Value>)
                       -> impl IntoResponse {
    let (manufacturer, name, instance, parameter_id) = path;
    let instance_id = get_instance_id((manufacturer, name, instance));

    let mut values = serde_json::Map::new();
    values.insert(parameter_id, value);

    state.instances
         .set_parameters(&instance_id, serde_json::Value::Object(values))
         .await
         .map_err(to_http_error)
         .map(Json)
}

async fn set_desired_instance_state(State(state): State<DriverState>,
                                    Path(path): Path<(String, String, String)>,
                                    Json(desired_state): Json<DesiredInstancePlayState>)
                                    -> impl IntoResponse {
    let instance_id = get_instance_id(path);

    state.instances
         .set_desired_play_state(&instance_id, desired_state)
         .await
         .map_err(to_http_error)
         .map(Json)
}

fn get_instance_id((manufacturer, name, instance): (String, String, String)) -> FixedInstanceId {
    FixedInstanceId::new(manufacturer, name, instance)
}

fn to_http_error(error: InstanceDriverError) -> (StatusCode, Json<InstanceDriverError>) {
    use InstanceDriverError::*;

    let status = match &error {
        InstanceNotFound { .. } | ParameterDoesNotExist { .. } => StatusCode::NOT_FOUND,
        MediaNotPresent | DriverNotSupported { .. } => StatusCode::NOT_IMPLEMENTED,
        ParametersMalformed { .. } | ReportsMalformed { .. } | ConfigMalformed { .. } => StatusCode::BAD_REQUEST,
        RPC { .. } => StatusCode::BAD_GATEWAY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (status, Json(error))
}
