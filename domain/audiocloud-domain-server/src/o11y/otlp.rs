use std::any::Any;
use std::time::Duration;

use anyhow::anyhow;
use once_cell::sync::OnceCell;

use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry::sdk::metrics::{controllers, processors, selectors};
use opentelemetry::sdk::trace::Tracer;
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::{runtime, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;

use tracing_subscriber::registry::LookupSpan;

use crate::o11y::O11yOpts;

static PROMETHEUS_EXPORTER: OnceCell<PrometheusExporter> = OnceCell::new();

impl O11yOpts {
    fn generate_tracing_resource(&self) -> Resource {
        Resource::new(vec![KeyValue::new("service.name", "domain"),
                           KeyValue::new("service.namespace", "audiocloud.io"),
                           KeyValue::new("service.instance.name", self.domain_id.as_str().to_owned()),
                           KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),])
    }
}

pub fn otlp_tracing_layer<S>(opts: &O11yOpts) -> anyhow::Result<OpenTelemetryLayer<S, Tracer>>
    where S: Subscriber + for<'span> LookupSpan<'span>
{
    let otlp_exporter = || {
        opentelemetry_otlp::new_exporter().tonic()
                                          .with_endpoint(&opts.otlp_endpoint)
                                          .with_timeout(Duration::from_millis(opts.otlp_timeout_ms))
    };

    let trace_config = || trace::config().with_resource(opts.generate_tracing_resource());

    let tracer = opentelemetry_otlp::new_pipeline().tracing()
                                                   .with_exporter(otlp_exporter())
                                                   .with_trace_config(trace_config())
                                                   .install_batch(runtime::Tokio)?;

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}

pub fn setup_otlp_metrics(opts: &O11yOpts) -> anyhow::Result<Box<dyn Any>> {
    let export_config = || ExportConfig { endpoint: opts.otlp_endpoint.to_string(),
                                          timeout: Duration::from_millis(opts.otlp_timeout_ms),
                                          ..ExportConfig::default() };

    let metrics =
        opentelemetry_otlp::new_pipeline().metrics(selectors::simple::inexpensive(), cumulative_temporality_selector(), runtime::Tokio)
                                          .with_resource(opts.generate_tracing_resource())
                                          .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_export_config(export_config()))
                                          .build();

    Ok(Box::new(metrics))
}

pub fn setup_prometheus(opts: &O11yOpts) -> anyhow::Result<()> {
    let controller = controllers::basic(
        processors::factory(
            selectors::simple::inexpensive(),
            cumulative_temporality_selector(),
        )
        .with_memory(true),
    )
    .with_resource(opts.generate_tracing_resource())
    .build();

    PROMETHEUS_EXPORTER.set(opentelemetry_prometheus::exporter(controller).init())
                       .map_err(|_| anyhow!("Prometheus exporter already initialized"))?;

    Ok(())
}

pub fn generate_prometheus_metrics() -> anyhow::Result<String> {
    let encoder = TextEncoder::new();
    let metric_families = PROMETHEUS_EXPORTER.get()
                                             .ok_or_else(|| anyhow!("Prometheus exporter not initialized"))?
                                             .registry()
                                             .gather();
    let mut result = Vec::new();
    encoder.encode(&metric_families, &mut result)?;

    Ok(String::from_utf8(result)?)
}
