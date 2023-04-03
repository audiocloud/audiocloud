use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::time::Duration;

use anyhow::anyhow;
use async_nats::jetstream::kv::Config;
use chrono::Utc;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

use api::graph::AudioGraphSpec;
use api::{driver, media};
use domain_server::tasks;
use domain_server::tasks::TaskSpec;

#[tokio::main]
async fn main() -> tasks::Result {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info,domain_server=trace");
  }

  tracing_subscriber::fmt::SubscriberBuilder::default().compact()
                                                       .with_thread_ids(true)
                                                       .with_target(false)
                                                       .with_env_filter(EnvFilter::from_default_env())
                                                       .init();

  let client = async_nats::connect("127.0.0.1:4222").await?;
  let jetstream = async_nats::jetstream::new(client);

  for bucket in [driver::buckets::INSTANCE_STATE,
                 driver::buckets::INSTANCE_SPEC,
                 driver::buckets::INSTANCE_CONTROL,
                 media::buckets::MEDIA_SPEC,
                 media::buckets::MEDIA_STATE]
  {
    jetstream.create_key_value(Config { bucket: bucket.to_string(),
                                        ..Config::default() })
             .await
             .map_err(|err| anyhow!("failed to create bucket: {bucket}: {err}"))?;
  }

  let mut le_instnaces = HashMap::new();
  le_instnaces.insert("one".to_owned(), "pultec_1".to_owned());

  let task_id = "task".to_owned();
  let spec = TaskSpec { app_id:     "bintest".to_string(),
                        from:       Utc::now(),
                        to:         Utc::now() + chrono::Duration::hours(4),
                        requests:   Default::default(),
                        instances:  le_instnaces,
                        graph_spec: AudioGraphSpec::default(), };

  let (tx_cmd, rx_cmd) = mpsc::channel(0xff);

  let mut service = tasks::run::TaskService::new(jetstream, task_id, spec, rx_cmd).await?;
  let handle = spawn(async move { service.run().await });

  sleep(Duration::from_secs(60 * 5)).await;
  tx_cmd.send(tasks::run::Command::Terminate).await?;

  handle.await??;

  Ok(())
}
