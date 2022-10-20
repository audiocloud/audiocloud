use std::convert::Infallible;
use std::future::Future;

use actix::fut::Ready;
use actix::{fut, MailboxError};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::Payload;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{get, web, FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use anyhow::anyhow;
use clap::{Args, ValueEnum};
use derive_more::IsVariant;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tracing::*;

use audiocloud_api::domain::DomainError;
use audiocloud_api::{AppId, AppTaskId, Codec, Json, MsgPack, SecureKey, TaskId};

use crate::o11y::generate_prometheus_metrics;
use crate::{DomainSecurity, ResponseMedia};

mod v1;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(healthz)
       .service(metrics)
       .service(web::scope("/v1").configure(v1::configure));
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    let res = json!({
      "healthy": true
    });

    web::Json(res)
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    match generate_prometheus_metrics() {
        Ok(metrics) => HttpResponse::Ok().content_type("text/plain; version=0.0.4").body(metrics),
        Err(e) => {
            error!("Failed to generate metrics: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub struct ApiResponder(ResponseMedia);

impl ApiResponder {
    pub async fn respond<T, F>(self, fut: F) -> ApiResponse<T>
        where T: Serialize,
              F: Future<Output = Result<T, DomainError>>
    {
        let rv = fut.await;
        ApiResponse(self.0, rv)
    }
}

impl FromRequest for ApiResponder {
    type Error = Infallible;
    type Future = Ready<Result<Self, Infallible>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let rv = Self(req.headers()
                         .get("Accept")
                         .and_then(|v| {
                             if let Ok(v) = v.to_str() {
                                 if v == mime::APPLICATION_MSGPACK.essence_str() {
                                     Some(ResponseMedia::MsgPack)
                                 } else if v == mime::APPLICATION_JSON.essence_str() {
                                     Some(ResponseMedia::Json)
                                 } else {
                                     None
                                 }
                             } else {
                                 None
                             }
                         })
                         .unwrap_or(ResponseMedia::Json));

        fut::ready(Ok(rv))
    }
}

pub struct ApiResponse<T>(ResponseMedia, Result<T, DomainError>);

impl<T> Responder for ApiResponse<T> where T: Serialize
{
    type Body = EitherBody<BoxBody>;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let err_resp = |err: DomainError| {
            let (content, content_type) = match self.0 {
                ResponseMedia::Json => (Json.serialize(&err).unwrap(), mime::APPLICATION_JSON.as_ref()),
                ResponseMedia::MsgPack => (MsgPack.serialize(&err).unwrap(), mime::APPLICATION_MSGPACK.as_ref()),
            };

            let status = StatusCode::from_u16(err.status_code()).unwrap();
            HttpResponseBuilder::new(status).content_type(content_type)
                                            .body(content)
                                            .map_into_right_body()
        };

        match self.1 {
            Ok(ok) => {
                let (content, content_type) = match self.0 {
                    ResponseMedia::Json => (Json.serialize(&ok).map_err(|e| DomainError::Serialization { error: e.to_string() }),
                                            mime::APPLICATION_JSON.as_ref()),
                    ResponseMedia::MsgPack => (MsgPack.serialize(&ok)
                                                      .map_err(|e| DomainError::Serialization { error: e.to_string() }),
                                               mime::APPLICATION_MSGPACK.as_ref()),
                };

                let content = match content {
                    Err(err) => return err_resp(err),
                    Ok(content) => content,
                };

                HttpResponseBuilder::new(StatusCode::OK).content_type(content_type)
                                                        .body(content)
                                                        .map_into_left_body()
            }
            Err(err) => err_resp(err),
        }
    }
}

const HEADER_AUTH_PREFIX: &'static str = "Bearer ";

impl FromRequest for DomainSecurity {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let func = move || -> Result<Self, actix_web::Error> {
            let auth_opts = req.app_data::<web::Data<RestOpts>>()
                               .ok_or_else(|| ErrorInternalServerError(anyhow!("Missing authentication configuration")))?;

            match req.headers().get(AUTHORIZATION) {
                None => match auth_opts.rest_auth_strategy {
                    AuthStrategy::Development => Ok(DomainSecurity::Cloud),
                    AuthStrategy::Production => Err(ErrorUnauthorized(anyhow!("Authentication missing"))),
                },
                Some(authorization) => {
                    let authorization = authorization.to_str()
                                                     .map_err(|err| ErrorBadRequest(anyhow!("Error parsing authorization header: {err}")))?;

                    if !authorization.starts_with(HEADER_AUTH_PREFIX) {
                        Err(ErrorUnauthorized(anyhow!("Authentication missing")))
                    } else {
                        Ok(DomainSecurity::SecureKey(SecureKey::new(authorization[HEADER_AUTH_PREFIX.len()..].to_string())))
                    }
                }
            }
        };

        fut::ready(func())
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

pub fn bad_gateway(err: MailboxError) -> DomainError {
    DomainError::BadGateway { error: err.to_string() }
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
