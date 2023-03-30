use fs::read;
use std::env::args;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{sse, IntoResponse};
use axum::routing::get;
use axum::{routing::put, Json, Router};
use futures::StreamExt;
use serde_json::json;
use tokio::spawn;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::BroadcastStream;
use tracing_subscriber::fmt::format::FmtSpan;

use api::driver::{InstanceDriverConfig, InstanceDriverEvent, SetInstanceParameterRequest};
use domain_server::instance_driver::run::{run_driver_server, InstanceDriverCommand};
use domain_server::instance_driver::usb_hid::UsbHidDriver;

#[derive(Clone)]
struct ServerState {
  tx_cmd: mpsc::Sender<InstanceDriverCommand>,
  tx_brd: Arc<broadcast::Sender<InstanceDriverEvent>>,
  config: Arc<InstanceDriverConfig>,
}

#[tokio::main]
async fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info,domain_server=trace");
  }

  tracing_subscriber::fmt::SubscriberBuilder::default().compact()
                                                       .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
                                                       .with_line_number(true)
                                                       .with_file(true)
                                                       .with_thread_ids(true)
                                                       .with_thread_names(true)
                                                       .init();

  let (tx_cmd, rx_cmd) = mpsc::channel(0xff);
  let (tx_evt, mut rx_evt) = mpsc::channel(0xff);

  let cfg = serde_yaml::from_slice::<InstanceDriverConfig>(read(args().skip(1).next().expect("Need config file parameter")).expect("Failed to open config file")
                                                                                           .as_slice()).expect("Failed to parse config");

  // TODO: we should subscribe to NATS and observe changes

  let _handle = match cfg.clone() {
    | InstanceDriverConfig::USBHID(usb_config) =>
      spawn(run_driver_server::<UsbHidDriver>("instance".to_string(), usb_config, rx_cmd, tx_evt)),
    | InstanceDriverConfig::Serial(_) => {
      todo!("Serial driver not supported");
    }
    | InstanceDriverConfig::OSC(_) => {
      todo!("OSC driver not supported");
    }
  };

  let (tx_brd, _rx_brd) = broadcast::channel(0xff);
  spawn({
    let tx_brd = tx_brd.clone();
    async move {
      while let Some(event) = rx_evt.recv().await {
        let _ = tx_brd.send(event);
      }
    }
  });
  let tx_brd = Arc::new(tx_brd);

  let config = Arc::new(cfg);

  let router = Router::new().route("/events", get(read_events))
                            .route("/config", get(get_config))
                            .route("/parameter/:parameter_id/:channel", put(set_parameter))
                            .with_state(ServerState { tx_cmd, tx_brd, config });

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  axum::Server::bind(&addr).serve(router.into_make_service())
                           .await
                           .expect("successful serving");
}

async fn set_parameter(State(ServerState { tx_cmd, .. }): State<ServerState>,
                       Path((parameter, channel)): Path<(String, usize)>,
                       value: String)
                       -> impl IntoResponse {
  let value = value.as_str().parse::<f64>().map_err(|_| StatusCode::BAD_REQUEST)?;
  tx_cmd.send(InstanceDriverCommand::SetParameters(SetInstanceParameterRequest { parameter, channel, value }))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok::<_, StatusCode>(Json(()))
}

async fn read_events(State(ServerState { tx_brd, .. }): State<ServerState>) -> impl IntoResponse {
  let rx = BroadcastStream::new(tx_brd.subscribe());
  let stream = rx.filter_map(|event| Box::pin(async move { event.ok() }));
  let stream = stream.map(|event| match event {
                       | InstanceDriverEvent::Report(report) =>
                         sse::Event::default().event(report.report_id)
                                              .json_data(json!({"channel": report.channel, "value": report.value})),
                     });

  sse::Sse::new(stream).keep_alive(sse::KeepAlive::new().interval(Duration::from_secs(1)).text("keep-stream-alive"))
}

async fn get_config(State(ServerState { config, .. }): State<ServerState>) -> impl IntoResponse {
  Json((&*config).clone())
}
