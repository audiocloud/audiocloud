use axum_connect::error::{RpcError, RpcErrorCode};

pub fn auth_error<T>(err: String) -> Result<T, RpcError> {
  Err(RpcError::new(RpcErrorCode::Unauthenticated, err))
}

pub fn internal_error<T>(err: String) -> Result<T, RpcError> {
  Err(RpcError::new(RpcErrorCode::Internal, err))
}

pub fn not_found_error<T>(err: String) -> Result<T, RpcError> {
  Err(RpcError::new(RpcErrorCode::NotFound, err))
}

pub fn invalid_argument_error<T>(err: String) -> Result<T, RpcError> {
  Err(RpcError::new(RpcErrorCode::InvalidArgument, err))
}
