use std::collections::HashMap;
use std::default::Default;

use chrono::Utc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::task::spec::{AudioGraphSpec, TaskSpec};
use domain_server::nats::Nats;
use domain_server::tasks;

const LOG_DEFAULTS: &'static str = "info,task=trace,domain_server=trace,tower_http=debug";

#[tokio::main]
async fn main() -> tasks::Result {
  tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_DEFAULTS.into()))
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let buckets = Nats::new(async_nats::connect("127.0.0.1:4222").await?).await
                                                                       .expect("failed to create buckets");

  let mut le_instances = HashMap::new();
  le_instances.insert("one".to_owned(), "pultec_1".to_owned());

  let task_id = "task".to_owned();
  let spec = TaskSpec { app_id:     "bintest".to_string(),
                        host_id:    "host".to_string(),
                        from:       Utc::now(),
                        to:         Utc::now() + chrono::Duration::hours(4),
                        requests:   Default::default(),
                        instances:  le_instances,
                        graph_spec: AudioGraphSpec::default(), };

  Ok(())
}
