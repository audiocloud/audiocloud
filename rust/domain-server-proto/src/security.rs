use std::collections::HashSet;

use argon2::Argon2;
use axum::extract::State;
use axum_connect::pbjson_types;
use axum_connect::pbjson_types::Empty;
use axum_connect::prelude::*;
use chrono::{LocalResult, TimeZone, Utc};
use jwt::{Header, SignWithKey, Token, VerifyWithKey, VerifyingAlgorithm};
use password_hash::{PasswordHash, SaltString};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

use api_proto::{
  authorization_token_info, AppInfo, AuthorizationTokenInfo, CreateApiKeyRequest, CreateApiKeyResponse, CreateTokenResponse,
  DescribeTokenRequest, GlobalPermission, InvalidateApiKeyRequest, InvalidateApiKeyResponse, InvalidateTokenRequest,
  InvalidateTokenResponse, ListApiKeysRequest, ListApiKeysResponse, ListAppsRequest, ListAppsResponse, ListUsersRequest, ListUsersResponse,
  RegisterAppRequest, RegisterUserRequest, TaskPermission, UpdateAppRequest, UpdateUserRequest, UserInfo, UserLoginRequest,
  UserLoginResponse,
};
use domain_db::security::{
  DbAppData, DbCreateApiKey, DbCreateApp, DbCreateUser, DbPrincipal, DbTokenResolvedData, DbUpdateApiKey, DbUpdateApp, DbUpdateUser,
  DbUserData,
};
use domain_db::{Db, Timestamp};

use crate::context::{Principal, ServiceContext, ServiceContextFactory, TaskContext};
use crate::error::{auth_error, internal_error, invalid_argument_error, not_found_error};

pub async fn user_login_handler(State(context): State<ServiceContextFactory>,
                                request: UserLoginRequest)
                                -> Result<UserLoginResponse, RpcError> {
  let Ok(Some(user)) = context.db.get_user_by_id(&request.username).await else { return auth_error(format!("Wrong or missing user")) };
  let Ok(hashed_password) = PasswordHash::new(&user.password) else { return internal_error(format!("Malformed password hash in database")) };
  let Ok(()) = hashed_password.verify_password(&[&Argon2::default()], &request.password) else { return auth_error(format!("Password validation failed")) };

  let exp = expiration(1200);

  let Ok(token) = context.db.create_token(DbPrincipal::User(request.username.clone()), exp).await else { return internal_error(format!("Failed to create token")) };

  let claims = TokenClaims { permissions: user.permissions.iter().cloned().collect(),
                             task:        None,
                             user_id:     Some(request.username.clone()),
                             app_id:      None,
                             exp:         exp.timestamp() as u64,
                             jti:         token.id.id.to_string(), };

  let Ok(authorization_token) = claims.sign_with_key(&context.token_hmac) else { return internal_error(format!("Failed to sign token")) };

  Ok(UserLoginResponse { authorization_token,
                         email: user.email,
                         permissions: user.permissions.into_iter().map(|p| p as i32).collect() })
}

pub async fn create_token_handler(context: ServiceContext, _request: Empty) -> Result<CreateTokenResponse, RpcError> {
  let (principal_user_id, principal_app_id) = match context.principal {
    | Principal::User(ref user) => (Some(user.id.clone()), None),
    | Principal::App(ref app) => (None, Some(app.id.clone())),
  };

  let exp = expiration(1200);

  let Ok(token) = context.db.create_token(match &context.principal {
                          | Principal::User(info) => DbPrincipal::User(info.id.clone()),
                          | Principal::App(info) => DbPrincipal::App(info.id.clone()),
                        }, exp).await else { return internal_error(format!("Failed to create token")) };

  let claims = TokenClaims { permissions: context.permissions,
                             task:        context.task,
                             user_id:     principal_user_id,
                             app_id:      principal_app_id,
                             exp:         exp.timestamp() as u64,
                             jti:         token.id.id.to_string(), };

  let Ok(authorization_token) = claims.sign_with_key(&context.token_hmac) else { return internal_error(format!("Failed to sign token")) };

  Ok(CreateTokenResponse { authorization_token })
}

pub async fn invalidate_token_handler(context: ServiceContext,
                                      request: InvalidateTokenRequest)
                                      -> Result<InvalidateTokenResponse, RpcError> {
  let Ok(Some(token)) = context.db.get_token_by_id(&request.token_id).await else { return not_found_error(format!("Failed to find token")) };
  let is_valid = token.expires_at > Utc::now();

  if !is_valid {
    let Ok(_) = context.db.update_token(&request.token_id, Some(Utc::now())).await else { return internal_error(format!("Failed to update token")) };
  }

  Ok(InvalidateTokenResponse { invalidated: is_valid })
}

pub async fn describe_token_handler(context: ServiceContext, request: DescribeTokenRequest) -> Result<AuthorizationTokenInfo, RpcError> {
  let (claims, principal, token) = decode_and_fetch_token(&request.authorization_token, &context.token_hmac, &context.db).await?;

  Ok(AuthorizationTokenInfo { token_id:    claims.jti,
                              expires_at:  Some(pbjson_types::Timestamp::from(token.expires_at)),
                              permissions: claims.permissions.into_iter().map(|p| p as i32).collect(),
                              principal:   match principal {
                                | Principal::User(user) => Some(authorization_token_info::Principal::UserId(user.id)),
                                | Principal::App(app) => Some(authorization_token_info::Principal::AppId(app.id)),
                              }, })
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
  pub permissions: HashSet<GlobalPermission>,
  pub task:        Option<TaskContext>,
  pub user_id:     Option<String>,
  pub app_id:      Option<String>,
  pub exp:         u64,
  pub jti:         String,
}

pub async fn register_user_handler(context: ServiceContext, request: RegisterUserRequest) -> Result<Empty, RpcError> {
  if request.username.len() < 3 {
    return invalid_argument_error(format!("Username must be at least 3 characters long"));
  }
  if request.password.len() < 8 {
    return invalid_argument_error(format!("Password must be at least 8 characters long"));
  }
  for p in &request.permissions {
    let Some(p) = GlobalPermission::from_i32(*p) else { return invalid_argument_error(format!("Invalid permission")) };
    if !context.permissions.contains(&p) {
      return auth_error(format!("Permission {p:?} cannot be given to a user if you don't have it"));
    }
  }

  let salt = SaltString::generate(rand::thread_rng());
  let Ok(hashed_password) = PasswordHash::generate(Argon2::default(), request.password.as_bytes(), &salt) else { return internal_error(format!("Failed to hash password")) };

  let Ok(_) = context.db.create_user(&request.username, DbCreateUser {
    email: request.email,
    password: hashed_password.to_string(),
    permissions: request.permissions.into_iter().filter_map(|p| GlobalPermission::from_i32(p)).collect(),
  }).await else { return internal_error(format!("Failed to create user")) };

  Ok(Empty {})
}

pub async fn update_user_handler(context: ServiceContext, request: UpdateUserRequest) -> Result<Empty, RpcError> {
  let set_password = match request.set_password {
    | None => None,
    | Some(new_password) => {
      let salt = SaltString::generate(rand::thread_rng());
      let Ok(hashed_password) = PasswordHash::generate(Argon2::default(), new_password.as_bytes(), &salt) else { return internal_error(format!("Failed to hash new password")) };
      Some(hashed_password.to_string())
    }
  };

  let Ok(_) = context.db.update_user(&request.user_id, DbUpdateUser {
    set_password,
    set_email: if request.use_set_email { None } else { Some(request.set_email) },
    set_permissions: if request.use_set_permissions { Some(request.set_permissions.into_iter().filter_map(|p| GlobalPermission::from_i32(p)).collect()) } else { None },
    set_disabled_at: if request.use_set_disabled_at { Some(request.set_disabled_at.map(|t| t.try_into().unwrap())) } else { None },
  }).await else { return internal_error(format!("Failed to update user")) };

  Ok(Empty {})
}

pub async fn register_app_handler(context: ServiceContext, request: RegisterAppRequest) -> Result<Empty, RpcError> {
  if request.id.len() < 3 {
    return invalid_argument_error(format!("App id must be at least 3 characters long"));
  }

  for p in &request.permissions {
    let Some(p) = GlobalPermission::from_i32(*p) else { return invalid_argument_error(format!("Invalid permission")) };
    if !context.permissions.contains(&p) {
      return auth_error(format!("Permission {p:?} cannot be given to an app if you don't have it"));
    }
  }

  let Ok(_) = context.db.create_app(&request.id, DbCreateApp {
    permissions: request.permissions.into_iter().filter_map(|p| GlobalPermission::from_i32(p)).collect(),
  }).await else { return internal_error(format!("Failed to create app")) };

  Ok(Empty {})
}

pub async fn update_app_handler(context: ServiceContext, request: UpdateAppRequest) -> Result<Empty, RpcError> {
  let Ok(_) = context.db.update_app(&request.app_id, DbUpdateApp {
    set_permissions: if request.use_set_permissions { Some(request.set_permissions.into_iter().filter_map(|p| GlobalPermission::from_i32(p)).collect()) } else { None },
    set_disabled_at: if request.use_set_disabled_at { Some(request.set_disabled_at.map(|t| t.try_into().unwrap())) } else { None },
  }).await else { return internal_error(format!("Failed to update app")) };

  Ok(Empty {})
}

pub async fn create_api_key_handler(context: ServiceContext, request: CreateApiKeyRequest) -> Result<CreateApiKeyResponse, RpcError> {
  let secret_value = rand::thread_rng().sample_iter(&Alphanumeric)
                                       .take(32)
                                       .map(char::from)
                                       .collect::<String>();

  let salt = SaltString::generate(rand::thread_rng());

  let Ok(generated_hash) = PasswordHash::generate(Argon2::default(), secret_value.as_bytes(), &salt) else { return internal_error(format!("Failed to hash secret value")) };

  let Ok(duration) = request.duration
                            .map(|d| d.try_into().ok())
                            .flatten()
                            .map(chrono::Duration::from_std)
                            .unwrap_or_else(|| Ok(chrono::Duration::days(31))) else { return invalid_argument_error(format!("Invalid duration")) };

  let principal = match &context.principal {
    | Principal::User(user) => DbPrincipal::User(user.id.clone()),
    | Principal::App(app) => DbPrincipal::App(app.id.clone()),
  };

  let Ok(api_key) = context.db.create_api_key(None, principal, DbCreateApiKey {
    name: request.name.unwrap_or_else(|| format!("API key created at {}", chrono::Utc::now().to_rfc3339())),
    hash: generated_hash.to_string(),
    task: request.task_id,
    permissions: request.global_permissions.into_iter().filter_map(|p| GlobalPermission::from_i32(p)).collect(), // TODO: reject unassignable
    task_permissions: request.task_permissions.into_iter().filter_map(|p| TaskPermission::from_i32(p)).collect(),
    expires_at: Utc::now() + duration
  }).await else { return internal_error(format!("Failed to create api key")) };

  Ok(CreateApiKeyResponse { api_key_id: api_key.id.to_string(),
                            secret_value })
}

pub async fn invalidate_api_key_handler(context: ServiceContext,
                                        request: InvalidateApiKeyRequest)
                                        -> Result<InvalidateApiKeyResponse, RpcError> {
  let Ok(Some(existing)) = context.db.update_api_key(&request.key_id, DbUpdateApiKey { set_expires_at: Some(Utc::now()) }).await else { return internal_error(format!("Failed to invalidate api key")) };

  Ok(InvalidateApiKeyResponse { invalidated: existing.expires_at <= Utc::now(), })
}

pub async fn list_users_handler(context: ServiceContext, request: ListUsersRequest) -> Result<ListUsersResponse, RpcError> {
  let Ok(users) = context.db
         .list_users(request.filter_id.as_ref().map(|s| s.as_str()),
                     request.filter_email.as_ref().map(|s| s.as_str()),
                     request.offset.unwrap_or_default(),
                     request.limit.unwrap_or(100))
         .await else { return internal_error(format!("Failed to list users")) };

  Ok(ListUsersResponse { users: users.iter().map(user_info_from).collect(), })
}

pub async fn list_apps_handler(context: ServiceContext, request: ListAppsRequest) -> Result<ListAppsResponse, RpcError> {
  let Ok(apps) = context.db.list_apps(request.filter_id.as_ref().map(|s| s.as_str()),
                                       request.offset.unwrap_or_default(),
                                       request.limit.unwrap_or(100))
                            .await else { return internal_error(format!("Failed to list apps")) };

  Ok(ListAppsResponse { apps: apps.iter().map(app_info_from).collect(), })
}

pub async fn list_api_keys_handler(context: ServiceContext, request: ListApiKeysRequest) -> Result<ListApiKeysResponse, RpcError> {
  todo!()
}

pub fn user_info_from(user: &DbUserData) -> UserInfo {
  UserInfo { id:          user.id.id.to_string(),
             email:       user.email.clone(),
             permissions: user.permissions.iter().map(|id| *id as i32).collect(),
             disabled_at: user.disabled_at.clone().map(pbjson_types::Timestamp::from), }
}

pub fn app_info_from(app: &DbAppData) -> AppInfo {
  AppInfo { id:          app.id.id.to_string(),
            permissions: app.permissions.iter().map(|id| *id as i32).collect(),
            disabled_at: app.disabled_at.clone().map(pbjson_types::Timestamp::from), }
}

fn expiration(seconds: u64) -> Timestamp {
  chrono::Utc::now().checked_add_signed(chrono::Duration::seconds(seconds as i64))
                    .expect("valid timestamp")
}

pub async fn decode_and_fetch_token(token: &str,
                                    key: &impl VerifyingAlgorithm,
                                    db: &Db)
                                    -> Result<(TokenClaims, Principal, DbTokenResolvedData), RpcError> {
  let token: Token<Header, TokenClaims, _> = token.verify_with_key(key)
                                                  .or_else(|err| auth_error(format!("Invalid token: {err}")))?;

  let (_header, claims): (Header, TokenClaims) = token.into();

  let LocalResult::Single(local_exp) = Utc.timestamp_millis_opt(claims.exp as i64) else { return auth_error(format!("Invalid expiration")) };

  if local_exp > Utc::now() {
    return auth_error(format!("Token expired"));
  }

  let Ok(Some(token)) = db.get_token_by_id(&claims.jti).await else { return auth_error(format!("Token not found")) };

  let principal = match (&token.user, &token.app) {
    | (Some(user), None) => Principal::User(user_info_from(&user)),
    | (None, Some(app)) => Principal::App(app_info_from(&app)),
    | _ => return internal_error(format!("Invalid token: no principal")),
  };

  Ok((claims, principal, token))
}
