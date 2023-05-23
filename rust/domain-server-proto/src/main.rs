use std::net::SocketAddr;

use axum::extract::{Host, State};
use axum::Router;
use axum_connect::prelude::*;

use api_proto::{DomainSecurityService, UserLoginRequest, UserLoginResponse};

struct Service {
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let app = Router::new().rpc(DomainSecurityService::user_login(user_login_handler))
                         .with_state("foo_bar");

  let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
  println!("listening on http://{}", addr);
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

  Ok(())
}

async fn user_login_handler(Host(host): Host,
                            State(shared): State<&'static str>,
                            request: UserLoginRequest)
                            -> Result<UserLoginResponse, RpcError> {
  if request.username == "bojan" {
    Ok(UserLoginResponse { authorization_token: format!("this is a shared token: {shared} for {host}"), })
  } else {
    Err(RpcError::new(RpcErrorCode::PermissionDenied, "Invalid username or password".to_string()))
  }
}
