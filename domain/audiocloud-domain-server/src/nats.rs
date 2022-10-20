use std::io;
use std::time::Duration;

use anyhow::anyhow;
use futures::{stream, Stream, StreamExt};
use nats_aflowt::{connect, Connection, Message, Subscription};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stream_throttle::{ThrottlePool, ThrottleRate, ThrottledStream};
use tracing::*;

use audiocloud_api::{Codec, Json, MsgPack, Request};

static NATS_CONNECTION: OnceCell<Connection> = OnceCell::new();

#[instrument(skip_all, err)]
pub async fn init(nats_url: &str) -> anyhow::Result<()> {
    let conn = connect(nats_url).await?;
    NATS_CONNECTION
        .set(conn)
        .map_err(|_| anyhow!("NATS_CONNECTION already initialized"))?;

    Ok(())
}

pub fn subscribe<M: DeserializeOwned, C: Codec>(
    subject: String,
    codec: C,
) -> impl Stream<Item = M> {
    let conn = NATS_CONNECTION
        .get()
        .expect("NATS_CONNECTION not initialized");
    let throttle_rate = ThrottleRate::new(5, Duration::new(1, 0));
    let throttle_pool = ThrottlePool::new(throttle_rate);

    stream::repeat_with(move || conn.clone())
        .throttle(throttle_pool)
        .then(move |conn| {
            let subject = subject.clone();
            async move {
                let rv = conn.subscribe(&subject).await;
                rv
            }
        })
        .filter_map(move |res: io::Result<Subscription>| async move { res.ok() })
        .flat_map(move |sub: Subscription| sub.stream())
        .filter_map(move |msg: Message| {
            let codec = codec.clone();
            async move { codec.deserialize(&msg.data).ok() }
        })
}

pub fn subscribe_msgpack<M: DeserializeOwned>(subject: String) -> impl Stream<Item = M> {
    subscribe(subject, MsgPack)
}

pub fn subscribe_json<M: DeserializeOwned>(subject: String) -> impl Stream<Item = M> {
    subscribe(subject, Json)
}

pub async fn publish<M: Serialize, C: Codec>(
    subject: &str,
    codec: C,
    message: M,
) -> anyhow::Result<()> {
    let connection = NATS_CONNECTION
        .get()
        .ok_or_else(|| anyhow!("NATS_CONNECTION initialized"))?;

    let message = codec.serialize(&message)?;
    connection.publish(&subject, &message).await?;

    Ok(())
}

pub async fn request<R, C, S>(
    subject: S,
    codec: C,
    req: R,
) -> anyhow::Result<<R as Request>::Response>
where
    R: Request,
    C: Codec,
    S: ToString,
{
    let subject = subject.to_string();
    let connection = NATS_CONNECTION
        .get()
        .ok_or_else(|| anyhow!("NATS_CONNECTION initialized"))?;

    let req = codec.serialize(&req)?;
    let reply = connection.request(&subject, &req).await?;
    Ok(codec.deserialize(&reply.data)?)
}

pub async fn request_json<R, S>(subject: S, req: R) -> anyhow::Result<<R as Request>::Response>
where
    R: Request,
    S: ToString,
{
    request(subject, Json, req).await
}

pub async fn request_msgpack<R, S>(subject: S, req: R) -> anyhow::Result<<R as Request>::Response>
where
    R: Request,
    S: ToString,
{
    request(subject, MsgPack, req).await
}
