use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use async_nats::jetstream::{kv, Context};
use async_nats::Client;
use async_stream::stream;
use bytes::Bytes;
use futures::{pin_mut, Stream, StreamExt, TryFutureExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::spawn;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing::warn;

use api::{driver, media, task};

pub type WatchStream<T> = Pin<Box<dyn Stream<Item = (String, Option<T>)> + Send>>;

pub fn watch_bucket_as_json<T: DeserializeOwned + Send + 'static>(store: kv::Store, key: String) -> WatchStream<T> {
  Box::pin(stream! {
    let Ok(stream) = store.watch_with_history(key.clone()).await else { return; };

    pin_mut!(stream);

    let name = &store.name;

    while let Some(entry) = stream.next().await {
      match entry {
        | Ok(entry) => {
          match entry.operation {
            | kv::Operation::Put => {
              match serde_json::from_slice(&entry.value) {
                | Ok(value) => {
                  yield (entry.key, Some(value));
                }
                | Err(err) => {
                  warn!(name, key, ?err, "Error deserializing JSON: {err}");
                }
              }
            }
            | kv::Operation::Delete | kv::Operation::Purge => {
              yield (entry.key, None);
            }
          }
        }
        | Err(err) => {
          warn!(name, key, ?err, "Error watching bucket: {err}");
        }
      }
    }
  })
}

pub fn nats_err(err: async_nats::Error) -> anyhow::Error {
  anyhow::anyhow!("NATS error {err}")
}

pub fn json_err(err: serde_json::Error) -> anyhow::Error {
  anyhow::anyhow!("JSON error {err}")
}

#[derive(Clone)]
pub struct Buckets {
  pub driver_spec:          Arc<kv::Store>,
  pub instance_state:       Arc<kv::Store>,
  pub instance_spec:        Arc<kv::Store>,
  pub instance_ctrl:        Arc<kv::Store>,
  pub media_download_spec:  Arc<kv::Store>,
  pub media_download_state: Arc<kv::Store>,
  pub media_upload_spec:    Arc<kv::Store>,
  pub media_upload_state:   Arc<kv::Store>,
  pub task_spec:            Arc<kv::Store>,
  pub task_state:           Arc<kv::Store>,
  pub task_ctrl:            Arc<kv::Store>,
}

impl Buckets {
  pub async fn new(js: &Context) -> anyhow::Result<Self> {
    let create = |name, ttl| {
      js.create_key_value(default_bucket_config(name, ttl))
        .map_err(nats_err)
        .map_ok(Arc::new)
    };

    let forever = Duration::default();
    let three_days = Duration::from_secs(3 * 24 * 60 * 60);

    Ok(Self { driver_spec:          create(driver::buckets::DRIVER_SPEC, forever).await?,
              instance_state:       create(driver::buckets::INSTANCE_STATE, forever).await?,
              instance_spec:        create(driver::buckets::INSTANCE_SPEC, forever).await?,
              instance_ctrl:        create(driver::buckets::INSTANCE_CONTROL, forever).await?,
              media_download_spec:  create(media::buckets::DOWNLOAD_SPEC, three_days).await?,
              media_upload_spec:    create(media::buckets::UPLOAD_SPEC, three_days).await?,
              media_download_state: create(media::buckets::DOWNLOAD_STATE, three_days).await?,
              media_upload_state:   create(media::buckets::UPLOAD_STATE, three_days).await?,
              task_spec:            create(task::buckets::TASK_SPEC, forever).await?,
              task_state:           create(task::buckets::TASK_STATE, forever).await?,
              task_ctrl:            create(task::buckets::TASK_CONTROL, forever).await?, })
  }
}

fn default_bucket_config(name: impl ToString, ttl: Duration) -> kv::Config {
  kv::Config { bucket: name.to_string(),
               max_age: ttl,
               ..kv::Config::default() }
}

pub type RequestStream<Req, Res> = Pin<Box<dyn Stream<Item = (String, Req, oneshot::Sender<Res>)> + Send>>;

pub fn serve_request_json<Req: DeserializeOwned + Send + 'static, Res: Serialize + Send + 'static>(client: Client,
                                                                                                   name: String)
                                                                                                   -> RequestStream<Req, Res> {
  Box::pin(stream! {
    let Ok(stream) = client.subscribe(name).await else { return; };

    pin_mut!(stream);

    while let Some(message) = stream.next().await {
      let Some(reply) = message.reply.clone() else { continue; };
      let Ok(req) = serde_json::from_slice::<Req>(&message.payload) else { continue; };

      let (tx, rx) = oneshot::channel();
      let client = client.clone();

      spawn(async move {
        let Ok(Ok(res)) = timeout(Duration::from_secs(10), rx).await else { warn!("Request timed out"); return; };
        let Ok(res) = serde_json::to_string(&res) else { warn!("Response failed to serialize"); return; };
        let Ok(_) = client.publish(reply, Bytes::from(res)).await else { warn!("Failed to publish response"); return;};
      });

      yield (message.subject, req, tx);
    }
  })
}
