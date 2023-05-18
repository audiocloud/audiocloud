use axum::extract::Query;
use axum::{
  extract::State,
  http::{header, Request, StatusCode},
  middleware::Next,
  response::IntoResponse,
  Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use api::auth::Auth;
use api::user::{LoginUserRequest, LoginUserResponse, LogoutUserResponse, UserSummary};

use crate::service::Service;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
  pub status:  &'static str,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
  sub:   String,
  email: String,
  exp:   usize,
  iat:   usize,
}

#[derive(Deserialize)]
pub struct TokenQuery {
  #[serde(default)]
  token: Option<String>,
}

pub async fn auth<B>(cookie_jar: CookieJar,
                     State(service): State<Service>,
                     Query(token_query): Query<TokenQuery>,
                     mut req: Request<B>,
                     next: Next<B>)
                     -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
  let token = cookie_jar.get("token")
                        .map(|cookie| cookie.value().to_string())
                        .or_else(|| {
                          req.headers()
                             .get(header::AUTHORIZATION)
                             .and_then(|auth_header| auth_header.to_str().ok())
                             .and_then(|auth_value| {
                               if auth_value.starts_with("Bearer ") {
                                 Some(auth_value[7..].to_owned())
                               } else {
                                 None
                               }
                             })
                        })
                        .or_else(|| token_query.token);

  let token = token.ok_or_else(|| {
                     let json_error = ErrorResponse { status:  "fail",
                                                      message: "You are not logged in, please provide token".to_string(), };
                     (StatusCode::UNAUTHORIZED, Json(json_error))
                   })?;

  let claims = decode::<TokenClaims>(&token,
                                     &DecodingKey::from_secret(service.config.jwt_secret.as_ref()),
                                     &Validation::default()).map_err(|_| {
                                                              let json_error = ErrorResponse { status:  "fail",
                                                                                               message: "Invalid token".to_string(), };
                                                              (StatusCode::UNAUTHORIZED, Json(json_error))
                                                            })?
                                                            .claims;

  let user_id = claims.sub;

  let user = service.get_user(&user_id).await.map_err(|err| {
                                                let json_error = ErrorResponse { status:  "fail",
                                                                                 message: err.to_string(), };
                                                (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
                                              })?;
  let user = user.ok_or_else(|| {
                   let json_error = ErrorResponse { status:  "fail",
                                                    message: "The user belonging to this token no longer exists".to_string(), };
                   (StatusCode::UNAUTHORIZED, Json(json_error))
                 })?;

  req.extensions_mut().insert(Auth::User(UserSummary { id:    user.id,
                                                       email: user.email, }));

  Ok(next.run(req).await)
}

pub async fn login_user_handler(State(service): State<Service>,
                                Json(body): Json<LoginUserRequest>)
                                -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let user = service.check_user_login(&body.id, &body.password).await.map_err(|err| {
                                                                        let error_response = serde_json::json!({
                                                                            "status": "fail",
                                                                            "message": err.to_string()
                                                                        });
                                                                        (StatusCode::UNAUTHORIZED, Json(error_response))
                                                                      })?;

  let now = chrono::Utc::now();
  let iat = now.timestamp() as usize;
  let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
  let sub = user.id.to_string();
  let email = user.email;

  let claims: TokenClaims = TokenClaims { sub, exp, iat, email };

  let token = encode(&Header::default(),
                     &claims,
                     &EncodingKey::from_secret(service.config.jwt_secret.as_ref())).unwrap();

  let cookie = Cookie::build("token", token.to_owned()).path("/")
                                                       .max_age(time::Duration::hours(1))
                                                       .same_site(SameSite::None)
                                                       .http_only(true)
                                                       .finish();

  let mut response = Json(LoginUserResponse { token }).into_response();

  response.headers_mut()
          .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

  Ok(response)
}

pub async fn logout_user_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let cookie = Cookie::build("token", "").path("/")
                                         .max_age(time::Duration::hours(-1))
                                         .same_site(SameSite::None)
                                         .http_only(true)
                                         .finish();

  let mut response = Json(LogoutUserResponse).into_response();

  response.headers_mut()
          .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

  Ok(response)
}
