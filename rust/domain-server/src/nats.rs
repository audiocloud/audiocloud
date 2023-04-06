use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use anyhow::bail;
use async_nats::jetstream::{kv, Context};
use async_nats::Client;
use async_stream::stream;
use bytes::Bytes;
use futures::{pin_mut, Stream, StreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::spawn;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tracing::warn;

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::driver::spec::DriverServiceSpec;
use api::instance::spec::InstanceSpec;
use api::instance::state::InstanceState;
use api::media::spec::{MediaDownloadSpec, MediaUploadSpec};
use api::media::state::{MediaDownloadState, MediaUploadState};
use api::task::spec::TaskSpec;
use api::{instance, instance_driver, media, task, BucketKey, BucketName, Events, Request};

pub type WatchStream<T> = Pin<Box<dyn Stream<Item = (String, Option<T>)> + Send>>;

pub fn watch_bucket_as_json<T>(store: kv::Store, control: BucketKey<T>) -> WatchStream<T>
  where T: DeserializeOwned + Send + 'static
{
  Box::pin(stream! {
    let key = control.key.clone();

    let Ok(stream) = store.watch_with_history(&key).await else { return; };

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
pub struct Nats {
  pub client:               Client,
  pub jetstream:            Context,
  pub driver_spec:          Bucket<DriverServiceSpec>,
  pub instance_state:       Bucket<InstanceState>,
  pub instance_spec:        Bucket<InstanceSpec>,
  pub instance_power_ctrl:  Bucket<InstancePowerControl>,
  pub instance_play_ctrl:   Bucket<InstancePlayControl>,
  pub media_download_spec:  Bucket<MediaDownloadSpec>,
  pub media_download_state: Bucket<MediaDownloadState>,
  pub media_upload_spec:    Bucket<MediaUploadSpec>,
  pub media_upload_state:   Bucket<MediaUploadState>,
  pub task_spec:            Bucket<TaskSpec>,
  pub task_state:           Bucket<()>,
  pub task_ctrl:            Bucket<()>,
}

impl Nats {
  pub async fn new(client: Client) -> anyhow::Result<Self> {
    let forever = Duration::default();
    let three_days = Duration::from_secs(3 * 24 * 60 * 60);

    let jetstream = async_nats::jetstream::new(client.clone());
    let js = &jetstream;

    Ok(Self { client:               client.clone(),
              jetstream:            async_nats::jetstream::new(client),
              driver_spec:          Bucket::new(js, &instance::driver::buckets::DRIVER_SPEC, forever).await?,
              instance_state:       Bucket::new(js, &instance::buckets::INSTANCE_STATE, forever).await?,
              instance_spec:        Bucket::new(js, &instance::buckets::INSTANCE_SPEC, forever).await?,
              instance_power_ctrl:  Bucket::new(js, &instance::buckets::INSTANCE_POWER_CONTROL, forever).await?,
              instance_play_ctrl:   Bucket::new(js, &instance::buckets::INSTANCE_PLAY_CONTROL, forever).await?,
              media_download_spec:  Bucket::new(js, &media::buckets::DOWNLOAD_SPEC, three_days).await?,
              media_upload_spec:    Bucket::new(js, &media::buckets::UPLOAD_SPEC, three_days).await?,
              media_download_state: Bucket::new(js, &media::buckets::DOWNLOAD_STATE, three_days).await?,
              media_upload_state:   Bucket::new(js, &media::buckets::UPLOAD_STATE, three_days).await?,
              task_spec:            Bucket::new(js, &task::buckets::TASK_SPEC, forever).await?,
              task_state:           Bucket::new(js, &task::buckets::TASK_STATE, forever).await?,
              task_ctrl:            Bucket::new(js, &task::buckets::TASK_CONTROL, forever).await?, })
  }

  pub fn subscribe_to_events<Evt>(&self, events: Events<Evt>) -> EventStream<Evt>
    where Evt: DeserializeOwned + Send + 'static
  {
    subscribe_to_events_json(self.client.clone(), events)
  }

  pub fn serve_requests<Req, Res>(&self, request: Request<Req, Res>) -> RequestStream<Req, Res>
    where Req: DeserializeOwned + Send + 'static,
          Res: Serialize + Send + 'static
  {
    serve_request_json(self.client.clone(), request)
  }

  pub async fn publish_event<Evt>(&self, subject: Events<Evt>, event: Evt) -> anyhow::Result<()> {
    let event = serde_json::to_vec(&event).map_err(json_err)?;
    self.client.publish(subject.subject, Bytes::from(event)).await.map_err(nats_err)?;
    Ok(())
  }

  pub async fn publish_event_no_wait<Evt>(&self, subject: Events<Evt>, event: Evt) -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
    let subject = subject.subject;
    let event = serde_json::to_vec(&event).map_err(json_err)?;
    let client = self.client.clone();

    Ok(spawn(async move {
         client.publish(subject, Bytes::from(event)).await.map_err(nats_err)?;

         Ok::<_, anyhow::Error>(())
       }))
  }

  pub async fn request<Req, Res>(&self, subject: Request<Req, Res>, request: Req) -> anyhow::Result<Res>
    where Req: Serialize + Send + 'static,
          Res: DeserializeOwned + Send + 'static
  {
    let request = serde_json::to_vec(&request).map_err(json_err)?;
    let response = self.client.request(subject.subject, Bytes::from(request)).await.map_err(nats_err)?;
    let response = serde_json::from_slice(&response.data).map_err(json_err)?;

    Ok(response)
  }
}

fn default_bucket_config(name: impl ToString, ttl: Duration) -> kv::Config {
  kv::Config { bucket: name.to_string(),
               max_age: ttl,
               ..kv::Config::default() }
}

pub type RequestStream<Req, Res> = Pin<Box<dyn Stream<Item = (String, Req, oneshot::Sender<Res>)> + Send>>;

pub fn serve_request_json<Req, Res>(client: Client, request: Request<Req, Res>) -> RequestStream<Req, Res>
  where Req: DeserializeOwned + Send + 'static,
        Res: Serialize + Send + 'static
{
  Box::pin(stream! {
    let Ok(stream) = client.subscribe(request.subject.clone()).await else { return; };

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

pub type EventStream<T> = Pin<Box<dyn Stream<Item = (String, T)> + Send>>;

pub fn subscribe_to_events_json<Evt>(client: Client, events: Events<Evt>) -> EventStream<Evt>
  where Evt: DeserializeOwned + Send + 'static
{
  Box::pin(stream! {
    let Ok(stream) = client.subscribe(events.subject.clone()).await else { return; };

    while let Some(message) = stream.next().await {
      let Ok(event) = serde_json::from_slice::<Evt>(&message.payload) else { continue; };

      yield (message.subject, event);
    }
  })
}

#[derive(Clone)]
pub struct Bucket<T> {
  pub store: Arc<kv::Store>,
  _marker:   PhantomData<T>,
}

impl<T> Bucket<T> where T: DeserializeOwned + Send + 'static
{
  pub async fn new(js: &Context, name: &BucketName<T>, ttl: Duration) -> anyhow::Result<Self> {
    let store = js.create_key_value(default_bucket_config(name.name, ttl)).await.map_err(nats_err)?;

    Ok(Self { store:   Arc::new(store),
              _marker: PhantomData, })
  }

  pub fn watch(&self, key: BucketKey<T>) -> WatchStream<T> {
    watch_bucket_as_json(self.store.as_ref().clone(), key)
  }

  pub fn watch_all(&self) -> WatchStream<T> {
    watch_bucket_as_json(self.store.as_ref().clone(), BucketKey::all())
  }

  pub async fn modify(&self, key: BucketKey<T>, modification: impl Fn(&mut T) -> ()) -> anyhow::Result<()>
    where T: Default
  {
    let mut store = self.store.as_ref().clone();

    for attempts in 1..=10 {
      let entry = store.entry(&key.key).await.map_err(nats_err)?;

      let (mut value, revision) = match entry {
        | None => (T::default(), -1),
        | Some(entry) => (serde_json::from_slice(&entry.value)?, entry.revision),
      };

      modification(&mut value);

      let bytes = Bytes::from(serde_json::to_vec(&value)?);

      if revision < 0 {
        if let Ok(_) = store.put(&key.key, bytes).await.map_err(nats_err) {
          break;
        }
      } else {
        if let Ok(_) = store.update(&key.key, bytes, revision).await.map_err(nats_err) {
          break;
        }
      }

      if attempts == 10 {
        bail!("Failed to modify bucket entry after 10 attempts");
      }
    }

    Ok(())
  }
}
