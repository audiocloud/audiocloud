use std::any::Any;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use clap::{Args, ValueEnum};
use maplit::hashmap;
use opentelemetry::Context;
use reqwest::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

use audiocloud_api::DomainId;

pub use self::otlp::generate_prometheus_metrics;

mod otlp;
mod sentry;

#[derive(Args, Clone, Debug)]
pub struct O11yOpts {
    /// How to export tracing data
    #[clap(long, env, default_value = "off")]
    tracing: TracingMode,

    /// How to export metrics data
    #[clap(long, env, default_value = "off")]
    metrics: MetricsMode,

    /// The OTLP collector or Grafana agent OTLP endpoint
    ///
    /// GRPC and HTTP endpoints should be supported.
    #[clap(long, env = "OTEL_EXPORTER_OTLP_METRICS_ENDPOINT", default_value = "grpc://localhost:4317")]
    otlp_endpoint: String,

    /// Timeout to write metrics and traces to OTLP, in milliseconds
    #[clap(long, env, default_value = "5000")]
    otlp_timeout_ms: u64,

    /// Sentry DSN, if tracing is set to 'sentry'
    ///
    /// You can get this from the Sentry project settings page, as of time of this writing it is an URL pointing to ingest.sentry.io
    #[clap(long, env)]
    sentry_dsn: Option<String>,

    /// Domain ID override
    #[clap(long, env, default_value = "")]
    pub domain_id: DomainId,

    /// Logs can be shipped to a Grafana Loki instance, You can provide a base URL to Loki instance here.
    ///
    /// This is used to ship only logs to Loki, and is not used for tracing or metrics.
    #[clap(long, env)]
    loki_url: Option<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum TracingMode {
    /// Do not export tracing data (default)
    Off,

    /// OTLP compatible exporter (GRPC)
    #[clap(name = "otlp")]
    OpenTracing,

    /// Sentry compatible exporter
    Sentry,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum MetricsMode {
    /// Do not export metrics data (default)
    Off,

    /// Export a /metrics REST endpoint that can be scraped by prometheus compatible metrics scrapers
    Prometheus,

    /// OTLP compatible exporter (GRPC)
    #[clap(name = "otlp")]
    OpenTracing,
}

pub fn init_tracing(opts: &O11yOpts) -> anyhow::Result<Box<dyn Any>> {
    set_log_env_defaults();

    let filter = || EnvFilter::from_default_env();

    let registry = Registry::default().with(tracing_subscriber::fmt::layer().with_filter(filter()));

    let registry =
        registry.with(match &opts.loki_url {
                          None => {
                              eprintln!("Loki logs NOT enabled");

                              None
                          }
                          Some(loki_url) => {
                              eprintln!("Loki logs shipping to: {}", loki_url);
                              let (layer, bg_task) = tracing_loki::layer(Url::from_str(&loki_url)?,
                                                                         hashmap! {
                                                                             "source".to_owned() => "audiocloud-io-domain".to_owned(),
                                                                             "domain_id".to_owned() => opts.domain_id.as_str().to_owned(),
                                                                         },
                                                                         HashMap::new())?;

                              tokio::spawn(bg_task);

                              Some(layer.with_filter(filter()))
                          }
                      });

    let mut guard: Box<dyn Any> = Box::new(());

    let registry = registry.with(if let TracingMode::Sentry = opts.tracing {
                                     let (layer, g) = sentry::sentry_tracing_layer(opts)?;
                                     guard = g;
                                     Some(layer)
                                 } else {
                                     None
                                 });

    let registry = registry.with(if let TracingMode::OpenTracing = opts.tracing {
                                     Some(otlp::otlp_tracing_layer(opts)?)
                                 } else {
                                     None
                                 });

    registry.init();

    Ok(guard)
}

pub fn init_metrics(opts: &O11yOpts) -> anyhow::Result<Box<dyn Any>> {
    match opts.metrics {
        MetricsMode::Off => {
            /* metrics exporting is disabled */
            Ok(Box::new(()))
        }
        MetricsMode::OpenTracing => {
            let metrics = otlp::setup_otlp_metrics(opts)?;
            Ok(metrics)
        }
        MetricsMode::Prometheus => {
            otlp::setup_prometheus(opts)?;
            Ok(Box::new(()))
        }
    }
}

fn set_log_env_defaults() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG",
                     "info,audiocloud_domain_server=debug,audiocloud_api=debug,actix_server=warn,rdkafka=debug");
    }
}

pub fn in_context<R>(f: impl FnOnce(&Context) -> R) -> R {
    let ctx = Context::current();
    f(&ctx)
}
