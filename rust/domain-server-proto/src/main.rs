use std::net::SocketAddr;

use argon2::Argon2;
use axum::Router;
use axum_connect::prelude::*;
use hmac::digest::KeyInit;
use hmac::Hmac;
use password_hash::{PasswordHash, SaltString};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::info;
use tracing_subscriber::prelude::*;

use api_proto::{CreateTaskRequest, CreateTaskResponse, DomainSecurityService, DomainTaskService, GlobalPermission, TaskPermission};
use domain_db::security::{DbCreateApiKey, DbCreateApp, DbPrincipal};
use domain_db::Db;

use crate::context::{ServiceContext, ServiceContextFactory};

pub mod context;
pub mod error;
pub mod security;

const LOG_DEFAULTS: &'static str = "info,domain_server_proto=trace,tower_http=debug";
const DEFAULT_PASSWORD: &'static str = "admin";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_DEFAULTS.into()))
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let in_mem_db = Db::new_in_mem().await?;
  in_mem_db.create_app("admin",
                       DbCreateApp { permissions: vec![GlobalPermission::CreateTask, GlobalPermission::DeleteTask], })
           .await?;

  let hashed_password = PasswordHash::generate(Argon2::default(),
                                               DEFAULT_PASSWORD.as_bytes(),
                                               &SaltString::generate(rand::thread_rng()))?.to_string();

  let api_key = in_mem_db.create_api_key(Some("admin".to_owned()),
                                         DbPrincipal::App("admin".to_owned()),
                                         DbCreateApiKey { hash:             hashed_password,
                                                          task:             Some("task123".to_owned()),
                                                          permissions:      vec![GlobalPermission::CreateTask],
                                                          task_permissions: vec![TaskPermission::ReadTask],
                                                          expires_at:       chrono::Utc::now() + chrono::Duration::days(1), })
                         .await?;

  info!("built-in admin API key: {}:{}", &api_key.id.id, DEFAULT_PASSWORD);

  let factory = ServiceContextFactory { db:         in_mem_db,
                                        token_hmac: Hmac::new_from_slice(b"cli1aut1w0000p322bmgj2tos01H165PPBRJH2N308A8XARYTED")?, };

  let app = Router::new().rpc(DomainSecurityService::user_login(security::user_login_handler))
                         .rpc(DomainSecurityService::create_token(security::create_token_handler))
                         .rpc(DomainSecurityService::invalidate_token(security::invalidate_token_handler))
                         .rpc(DomainSecurityService::describe_token(security::describe_token_handler))
                         .rpc(DomainSecurityService::register_user(security::register_user_handler))
                         .rpc(DomainTaskService::create_task(create_task_handler))
                         .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
                         .with_state(factory);

  let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
  println!("listening on http://{}", addr);
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

  Ok(())
}

async fn create_task_handler(context: ServiceContext, request: CreateTaskRequest) -> Result<CreateTaskResponse, RpcError> {
  info!("create_task_handler: {request:?}, {context:?}");

  Err(RpcError::new(RpcErrorCode::Internal, "not implemented".to_string()))
}
