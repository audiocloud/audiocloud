use std::collections::HashMap;
use std::default::Default;
use std::time::Duration;

use chrono::Utc;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::graph::AudioGraphSpec;
use domain_server::nats_utils::create_buckets;
use domain_server::tasks;
use domain_server::tasks::TaskSpec;

const LOG_DEFAULTS: &'static str = "info,task=trace,domain_server=trace,tower_http=debug";

#[tokio::main]
async fn main() -> tasks::Result {
  tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_DEFAULTS.into()))
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let client = async_nats::connect("127.0.0.1:4222").await?;
  let jetstream = async_nats::jetstream::new(client);

  create_buckets(&jetstream).await?;

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
