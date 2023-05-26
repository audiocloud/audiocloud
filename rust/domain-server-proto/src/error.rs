use axum_connect::error::{RpcError, RpcErrorCode};
use axum_connect::pbjson_types::Empty;

pub type RpcResult<T = Empty> = Result<T, RpcError>;

pub fn auth_error<T>(err: String) -> RpcResult<T> {
  Err(RpcError::new(RpcErrorCode::Unauthenticated, err))
}

pub fn internal_error<T>(err: String) -> RpcResult<T> {
  Err(RpcError::new(RpcErrorCode::Internal, err))
}

pub fn not_found_error<T>(err: String) -> RpcResult<T> {
  Err(RpcError::new(RpcErrorCode::NotFound, err))
}

pub fn invalid_argument_error<T>(err: String) -> RpcResult<T> {
  Err(RpcError::new(RpcErrorCode::InvalidArgument, err))
}

pub fn not_implemented_error<T>(method: &str) -> RpcResult<T> {
  internal_error(format!("{method} not implemented"))
}
