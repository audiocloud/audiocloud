use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail};
use async_nats::jetstream::kv::Operation;
use async_nats::jetstream::{kv, Context};
use async_nats::Client;
use async_stream::stream;
use bytes::Bytes;
use futures::channel::oneshot;
use futures::{pin_mut, FutureExt, Stream, StreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_stream::StreamMap;
use tracing::{debug, trace, warn};
use wildmatch::WildMatch;

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::driver::spec::DriverServiceSpec;
use api::instance::spec::InstanceSpec;
use api::instance::{InstanceConnectionState, InstancePlayState, InstancePowerState};
use api::media::spec::{MediaDownloadSpec, MediaId, MediaUploadSpec};
use api::media::state::{MediaDownloadState, MediaUploadState};
use api::task::spec::TaskSpec;
use api::task::DesiredTaskPlayState;
use api::user::UserSpec;
use api::{instance, media, task, user, BucketKey, BucketName, Events, Request};

pub type WatchStream<K, T> = Pin<Box<dyn Stream<Item = (K, Option<T>)> + Send>>;

pub type WatchStreamMap<K, T> = StreamMap<K, WatchStream<K, T>>;

pub fn watch_bucket_as_json<K, T>(store: kv::Store, control: BucketKey<K, T>) -> WatchStream<K, T>
  where T: DeserializeOwned + Send + 'static,
        K: FromStr + Send + 'static,
        K: ToString
{
  Box::pin(stream! {
    let key = control.key.clone().to_string();

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
                  let Ok(key) = K::from_str(&entry.key) else { continue; };
                  yield (key, Some(value));
                }
                | Err(err) => {
                  warn!(name, key, ?err, "Error deserializing JSON: {err}");
                }
              }
            }
            | kv::Operation::Delete | kv::Operation::Purge => {
              let Ok(key) = K::from_str(&entry.key) else { continue; };
              yield (key, None);
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
  anyhow!("NATS error {err}")
}

pub fn nats_request_err(err: async_nats::RequestError) -> anyhow::Error {
  anyhow!("NATS request error {err}")
}

pub fn nats_publish_err(err: async_nats::PublishError) -> anyhow::Error {
  anyhow!("NATS publish error {err}")
}

pub fn json_err(err: serde_json::Error) -> anyhow::Error {
  anyhow!("JSON error {err}")
}

#[derive(Clone)]
pub struct Nats {
  pub client:                    Client,
  pub jetstream:                 Context,
  pub driver_spec:               Bucket<String, DriverServiceSpec>,
  pub instance_power_state:      Bucket<String, InstancePowerState>,
  pub instance_play_state:       Bucket<String, InstancePlayState>,
  pub instance_connection_state: Bucket<String, InstanceConnectionState>,
  pub instance_spec:             Bucket<String, InstanceSpec>,
  pub instance_power_ctrl:       Bucket<String, InstancePowerControl>,
  pub instance_play_ctrl:        Bucket<String, InstancePlayControl>,
  pub media_download_spec:       Bucket<MediaId, MediaDownloadSpec>,
  pub media_download_state:      Bucket<MediaId, MediaDownloadState>,
  pub media_upload_spec:         Bucket<MediaId, MediaUploadSpec>,
  pub media_upload_state:        Bucket<MediaId, MediaUploadState>,
  pub task_spec:                 Bucket<String, TaskSpec>,
  pub task_state:                Bucket<String, ()>,
  pub user_spec:                 Bucket<String, UserSpec>,
  pub task_ctrl:                 Bucket<String, DesiredTaskPlayState>,
}

impl Nats {
  pub async fn new(client: Client, recreate: bool) -> anyhow::Result<Self> {
    let forever = Duration::default();
    let one_minute = Duration::from_secs(60);
    let three_days = Duration::from_secs(3 * 24 * 60 * 60);

    let jetstream = async_nats::jetstream::new(client.clone());
    let js = &jetstream;

    Ok(Self { client:                    client.clone(),
              jetstream:                 async_nats::jetstream::new(client),
              driver_spec:               Bucket::new(js, &instance::driver::buckets::DRIVER_SPEC, forever, recreate).await?,
              instance_connection_state: Bucket::new(js, &instance::buckets::INSTANCE_CONNECTION_STATE, one_minute, recreate).await?,
              instance_power_state:      Bucket::new(js, &instance::buckets::INSTANCE_POWER_STATE, forever, recreate).await?,
              instance_play_state:       Bucket::new(js, &instance::buckets::INSTANCE_PLAY_STATE, forever, recreate).await?,
              instance_spec:             Bucket::new(js, &instance::buckets::INSTANCE_SPEC, forever, recreate).await?,
              instance_power_ctrl:       Bucket::new(js, &instance::buckets::INSTANCE_POWER_CONTROL, forever, recreate).await?,
              instance_play_ctrl:        Bucket::new(js, &instance::buckets::INSTANCE_PLAY_CONTROL, forever, recreate).await?,
              media_download_spec:       Bucket::new(js, &media::buckets::DOWNLOAD_SPEC, three_days, recreate).await?,
              media_upload_spec:         Bucket::new(js, &media::buckets::UPLOAD_SPEC, three_days, recreate).await?,
              media_download_state:      Bucket::new(js, &media::buckets::DOWNLOAD_STATE, three_days, recreate).await?,
              media_upload_state:        Bucket::new(js, &media::buckets::UPLOAD_STATE, three_days, recreate).await?,
              task_spec:                 Bucket::new(js, &task::buckets::TASK_SPEC, forever, recreate).await?,
              task_state:                Bucket::new(js, &task::buckets::TASK_STATE, forever, recreate).await?,
              user_spec:                 Bucket::new(js, &user::buckets::USER_SPEC, forever, recreate).await?,
              task_ctrl:                 Bucket::new(js, &task::buckets::TASK_CONTROL, forever, recreate).await?, })
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

  pub async fn publish_event<Evt>(&self, subject: Events<Evt>, event: Evt) -> anyhow::Result<()>
    where Evt: Serialize + Debug
  {
    trace!(?event, "Publishing {}", subject.subject);

    let event = serde_json::to_vec(&event).map_err(json_err)?;
    self.client
        .publish(subject.subject, Bytes::from(event))
        .await
        .map_err(nats_publish_err)?;
    Ok(())
  }

  pub async fn publish_event_no_wait<Evt>(&self, subject: Events<Evt>, event: Evt) -> anyhow::Result<JoinHandle<anyhow::Result<()>>>
    where Evt: Serialize
  {
    let subject = subject.subject;
    let event = serde_json::to_vec(&event).map_err(json_err)?;
    let client = self.client.clone();

    Ok(spawn(async move {
         client.publish(subject, Bytes::from(event)).await.map_err(nats_publish_err)?;

         Ok::<_, anyhow::Error>(())
       }))
  }

  pub async fn request<Req, Res>(&self, subject: Request<Req, Res>, request: Req) -> anyhow::Result<Res>
    where Req: Serialize + Send + 'static,
          Res: DeserializeOwned + Send + 'static
  {
    let request = serde_json::to_vec(&request).map_err(json_err)?;
    let response = self.client
                       .request(subject.subject, Bytes::from(request))
                       .await
                       .map_err(nats_request_err)?;
    let response = serde_json::from_slice(&response.payload).map_err(json_err)?;

    Ok(response)
  }

  pub fn request_and_forget<Req, Res, F>(&self, subject: Request<Req, Res>, request: Req, func: F) -> JoinHandle<()>
    where Req: Serialize + Send + 'static,
          Res: DeserializeOwned + Send + 'static,
          F: FnOnce(anyhow::Result<Res>) -> () + Send + 'static
  {
    let client = self.client.clone();
    spawn(async move {
            let request = serde_json::to_vec(&request).map_err(json_err)?;
            let response = client.request(subject.subject, Bytes::from(request))
                                 .await
                                 .map_err(nats_request_err)?;
            let response: Res = serde_json::from_slice(&response.payload).map_err(json_err)?;
            Ok::<_, anyhow::Error>(response)
          }.map(|res| {
             func(res);
           }))
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

pub type EventStream<E> = Pin<Box<dyn Stream<Item = (String, E)> + Send>>;

pub type EventStreamMap<K, E> = StreamMap<K, EventStream<E>>;

pub fn subscribe_to_events_json<Evt>(client: Client, events: Events<Evt>) -> EventStream<Evt>
  where Evt: DeserializeOwned + Send + 'static
{
  Box::pin(stream! {
    let Ok(stream) = client.subscribe(events.subject.clone()).await else { return; };

    pin_mut!(stream);

    while let Some(message) = stream.next().await {
      let Ok(event) = serde_json::from_slice::<Evt>(&message.payload) else { continue; };

      yield (message.subject, event);
    }
  })
}

#[derive(Clone)]
pub struct Bucket<Key, Content> {
  pub store:       Arc<kv::Store>,
  _marker_key:     PhantomData<Key>,
  _marker_content: PhantomData<Content>,
}

impl<Key, Content> Bucket<Key, Content> where Content: DeserializeOwned + Send + 'static
{
  pub async fn new(js: &Context, name: &BucketName<Content>, ttl: Duration, recreate: bool) -> anyhow::Result<Self> {
    let store = if recreate {
      let _ = js.delete_key_value(name.name).await;
      js.create_key_value(default_bucket_config(name.name, ttl)).await.map_err(nats_err)?
    } else {
      js.get_key_value(name.name).await.map_err(nats_err)?
    };

    Ok(Self { store:           Arc::new(store),
              _marker_key:     PhantomData,
              _marker_content: PhantomData, })
  }

  pub async fn scan(&self, filter: &str) -> anyhow::Result<HashMap<String, Content>> {
    let matcher = WildMatch::new(filter);
    let mut rv = HashMap::new();
    let mut keys = self.store.keys().await.map_err(nats_err)?;
    while let Some(key) = keys.next().await {
      let Ok(key) = key else { continue; };
      if matcher.matches(&key) {
        let entry = self.store.entry(&key).await.map_err(nats_err)?;
        if let Some(entry) = entry {
          rv.insert(key, serde_json::from_slice(&entry.value).map_err(json_err)?);
        }
      }
    }

    Ok(rv)
  }

  pub async fn put(&self, key: BucketKey<Key, Content>, value: Content) -> anyhow::Result<u64>
    where Content: Serialize + Debug
  {
    debug!(?value, "Update {}[{}]", self.store.name, key.key);

    let value = serde_json::to_vec(&value).map_err(json_err)?;
    let revision = self.store.put(&key.key, Bytes::from(value)).await.map_err(nats_err)?;

    Ok(revision)
  }

  pub async fn get(&self, key: BucketKey<Key, Content>) -> anyhow::Result<Option<Content>> {
    let entry = self.store.entry(&key.key).await.map_err(nats_err)?;
    if let Some(entry) = entry {
      match entry.operation {
        | Operation::Put => Ok(Some(serde_json::from_slice(&entry.value).map_err(json_err)?)),
        | Operation::Delete | Operation::Purge => Ok(None),
      }
    } else {
      Ok(None)
    }
  }

  pub async fn delete(&self, key: BucketKey<Key, Content>) -> anyhow::Result<()> {
    self.store.delete(&key.key).await.map_err(nats_err)?;

    Ok(())
  }

  pub fn watch(&self, key: BucketKey<Key, Content>) -> WatchStream<Key, Content>
    where Key: FromStr + Send + Display + 'static
  {
    watch_bucket_as_json(self.store.as_ref().clone(), key)
  }

  pub fn watch_all(&self) -> WatchStream<Key, Content>
    where Key: FromStr + Send + ToString + 'static
  {
    watch_bucket_as_json(self.store.as_ref().clone(), BucketKey::all())
  }

  pub async fn modify(&self,
                      key: BucketKey<Key, Content>,
                      max_attempts: usize,
                      modification: impl Fn(&mut Content) -> ())
                      -> anyhow::Result<()>
    where Content: Default,
          Content: DeserializeOwned,
          Content: Serialize
  {
    let store = self.store.as_ref().clone();

    for attempts in 1..=max_attempts {
      let entry = store.entry(&key.key).await.map_err(nats_err)?;

      let (mut value, revision) = match entry {
        | None => (Content::default(), None),
        | Some(entry) => (serde_json::from_slice(&entry.value)?, Some(entry.revision)),
      };

      modification(&mut value);

      let bytes = Bytes::from(serde_json::to_vec(&value)?);

      match revision {
        | None =>
          if let Ok(_) = store.put(&key.key, bytes).await.map_err(nats_err) {
            break;
          },
        | Some(revision) =>
          if let Ok(_) = store.update(&key.key, bytes, revision).await.map_err(nats_err) {
            break;
          },
      }

      if attempts == 10 {
        bail!("Failed to modify bucket entry after 10 attempts");
      }
    }

    Ok(())
  }
}
