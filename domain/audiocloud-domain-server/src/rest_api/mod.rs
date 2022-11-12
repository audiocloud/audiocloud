/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::convert::Infallible;
use std::future::Future;

use anyhow::anyhow;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse, Json};
use axum::routing::get;
use axum::Router;
use clap::{Args, ValueEnum};
use derive_more::IsVariant;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tracing::*;

use audiocloud_api::domain::DomainError;
use audiocloud_api::{AppId, AppTaskId, SecureKey, TaskId};

use crate::DomainContext;
use crate::{DomainSecurity, ResponseMedia};

mod v1;

pub fn configure(router: Router<DomainContext>) -> Router<DomainContext> {
    router.route("/healthz", get(healthz))
          .route("/metrics", get(metrics))
          .nest("/v1", Router::inherit_state())
}

async fn healthz() -> impl IntoResponse {
    Json(json!({
           "healthy": true
         }))
}

async fn metrics() -> impl IntoResponse {
    match audiocloud_tracing::generate_prometheus_metrics() {
        Ok(metrics) => Ok((AppendHeaders([(CONTENT_TYPE, "text/plain; version=0.0.4")]), metrics)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Args, Clone, Copy)]
pub struct RestOpts {
    /// Authentication strategy to use for incoming REST requests
    #[clap(long, env, default_value = "production")]
    pub rest_auth_strategy: AuthStrategy,
}

#[derive(ValueEnum, Copy, Clone, IsVariant)]
pub enum AuthStrategy {
    /// Every unauthenticated request is considered to be coming from a superuser (**dangerous!**)
    Development,

    /// Secure keys must be provided for all requests
    Production,
}

#[derive(Deserialize)]
pub struct AppTaskIdPath {
    app_id:  AppId,
    task_id: TaskId,
}

impl Into<AppTaskId> for AppTaskIdPath {
    fn into(self) -> AppTaskId {
        let Self { app_id, task_id } = self;
        AppTaskId { app_id, task_id }
    }
}
