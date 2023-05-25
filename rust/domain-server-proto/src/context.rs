use std::collections::HashSet;

use argon2::Argon2;
use axum::extract::FromRequestParts;
use axum::headers::authorization::{Basic, Bearer};
use axum::headers::Authorization;
use axum::{async_trait, http, TypedHeader};
use axum_connect::error::RpcError;
use axum_connect::prelude::RpcFromRequestParts;
use axum_connect::prost::Message;
use chrono::Utc;
use hmac::Hmac;
use password_hash::PasswordHash;
use serde::{Deserialize, Serialize};
use sha2::Sha384;

use api_proto::{AppInfo, GlobalPermission, TaskPermission, UserInfo};
use domain_db::security::DbApiKeyDataResolveUserApp;
use domain_db::{Db, Timestamp};

use crate::error::{auth_error, internal_error};
use crate::security::{app_info_from, decode_and_fetch_token, user_info_from};

#[derive(Clone)]
pub struct ServiceContextFactory {
  pub db:         Db,
  pub token_hmac: Hmac<Sha384>,
}

#[derive(Debug, Clone)]
pub enum Principal {
  User(UserInfo),
  App(AppInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
  pub task_id:     String,
  pub permissions: HashSet<TaskPermission>,
}

impl TaskContext {
  pub fn from_api_key(api_key: &DbApiKeyDataResolveUserApp) -> Option<Self> {
    if let Some(task_id) = api_key.task.as_ref() {
      Some(Self { task_id:     task_id.id.to_string(),
                  permissions: api_key.task_permissions.iter().cloned().collect(), })
    } else {
      None
    }
  }
}

#[derive(Debug, Clone)]
pub struct ServiceContext {
  pub db:          Db,
  pub principal:   Principal,
  pub permissions: HashSet<GlobalPermission>,
  pub task:        Option<TaskContext>,
  pub token_hmac:  Hmac<Sha384>,
}

impl ServiceContextFactory {
  pub fn new_context(&self, principal: Principal, permissions: HashSet<GlobalPermission>, task: Option<TaskContext>) -> ServiceContext {
    ServiceContext { db: self.db.clone(),
                     principal,
                     permissions,
                     task,
                     token_hmac: self.token_hmac.clone() }
  }
}

type BearerAuth = TypedHeader<Authorization<Bearer>>;
type BasicAuth = TypedHeader<Authorization<Basic>>;

#[async_trait]
impl<M> RpcFromRequestParts<M, ServiceContextFactory> for ServiceContext where M: Message
{
  type Rejection = RpcError;

  async fn rpc_from_request_parts(parts: &mut http::request::Parts, factory: &ServiceContextFactory) -> Result<Self, Self::Rejection> {
    if let Ok(TypedHeader(bearer)) = BearerAuth::from_request_parts(parts, factory).await {
      let (claims, principal, token) = decode_and_fetch_token(&bearer.token(), &factory.token_hmac, &factory.db).await?;
      if token.expires_at > Utc::now() {
        return auth_error(format!("Token cancelled"));
      }

      check_if_principal_disabled(&principal)?;

      return Ok(factory.new_context(principal, claims.permissions, claims.task));
    } else if let Ok(TypedHeader(key)) = BasicAuth::from_request_parts(parts, factory).await {
      let (api_key, hashed_password) = (key.username(), key.password());
      let Ok(Some(api_key)) = factory.db.get_api_key_by_id(api_key).await else { return auth_error(format!("Invalid API key")) };
      let Ok(hash) = PasswordHash::new(&api_key.hash) else { return internal_error(format!("API key hash malformed")) };
      let Ok(_) = hash.verify_password(&[&Argon2::default()], hashed_password) else { return auth_error(format!("API key hash did not match")) };

      let principal = match (&api_key.user, &api_key.app) {
        | (Some(user), None) => Principal::User(user_info_from(user)),
        | (None, Some(app)) => Principal::App(app_info_from(app)),
        | _ => return internal_error(format!("API key references neither user nor app")),
      };

      check_if_principal_disabled(&principal)?;

      return Ok(factory.new_context(principal,
                                    api_key.permissions.iter().cloned().collect(),
                                    TaskContext::from_api_key(&api_key)));
    }

    auth_error(format!("Invalid authorization header"))
  }
}

fn check_if_principal_disabled(p: &Principal) -> Result<(), RpcError> {
  if match p {
    | Principal::User(u) => u.disabled_at
                             .as_ref()
                             .map(|d| {
                               let d: Timestamp = d.clone().try_into().unwrap();
                               d <= Utc::now()
                             })
                             .unwrap_or_default(),
    | Principal::App(a) => a.disabled_at
                            .as_ref()
                            .map(|d| {
                              let d: Timestamp = d.clone().try_into().unwrap();
                              d <= Utc::now()
                            })
                            .unwrap_or_default(),
  } {
    return auth_error(format!("User or app disabled"));
  }

  Ok(())
}
