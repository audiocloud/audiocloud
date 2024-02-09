use async_nats::Client;
use async_stream::try_stream;
use axum_connect::error::{RpcError, RpcErrorCode};
use axum_connect::prost::Message;
use futures::{Stream, StreamExt};

pub fn nats_subscribe<T: Message + Default>(conn: Client, subject: String) -> impl Stream<Item = Result<T, RpcError>> {
  try_stream! {
    let mut sub = conn.subscribe(subject.clone()).await.map_err(|err| RpcError::new(RpcErrorCode::Aborted, format!("Failed to subscribe to messages: {err}")))?;

    while let Some(msg) = sub.next().await {
      let data = T::decode(msg.payload.as_ref()).map_err(|err| RpcError::new(RpcErrorCode::Internal, format!("Failed to decode message: {err}")))?;

      yield data;
    }
  }
}
