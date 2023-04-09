use fs::read;
use std::env::args;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{headers, routing::put, Json, Router, TypedHeader};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};
use tokio::{select, spawn, time};
use tower_http::cors;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::instance::driver::config::InstanceDriverConfig;
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterRequest;
use api::instance::driver::ws::{WsDriverEvent, WsDriverRequest};
use domain_server::instance::driver::run_driver::{run_driver_server, InstanceDriverCommand};
use domain_server::instance::driver::usb_hid::UsbHidDriver;

#[derive(Clone)]
struct ServerState {
  tx_cmd: mpsc::Sender<InstanceDriverCommand>,
  tx_brd: Arc<broadcast::Sender<InstanceDriverEvent>>,
  config: Arc<InstanceDriverConfig>,
}

const LOG_DEFAULTS: &'static str = "info,driver=trace,domain_server=trace,tower_http=debug";

#[tokio::main]
async fn main() {
  tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_DEFAULTS.into()))
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let (tx_cmd, rx_cmd) = mpsc::channel(0xff);
  let (tx_evt, mut rx_evt) = mpsc::channel(0xff);

  let cfg = serde_yaml::from_slice::<InstanceDriverConfig>(read(args().skip(1).next().expect("Need config file parameter")).expect("Failed to open config file")
    .as_slice()).expect("Failed to parse config");

  let _handle = match cfg.clone() {
    | InstanceDriverConfig::USBHID(usb_config) =>
      spawn(run_driver_server::<UsbHidDriver>("instance".to_string(), usb_config, rx_cmd, tx_evt)),
    | InstanceDriverConfig::Serial(_) => {
      todo!("Serial driver not supported");
    }
    | InstanceDriverConfig::OSC(_) => {
      todo!("OSC driver not supported");
    }
    | InstanceDriverConfig::HTTP(_) => {
      todo!("HTTP driver not supported");
    }
    | InstanceDriverConfig::SPI(_) => {
      todo!("SPI driver not supported");
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

  let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

  let router = Router::new().fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
                            .route("/ws", get(ws_handler))
                            .route("/config", get(get_config))
                            .route("/parameter/:parameter_id/:channel", put(set_parameter))
                            .with_state(ServerState { tx_cmd, tx_brd, config })
                            .layer(cors::CorsLayer::permissive())
                            .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)));

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  debug!("listening on {addr}");
  axum::Server::bind(&addr).serve(router.into_make_service_with_connect_info::<SocketAddr>())
                           .await
                           .unwrap();
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

async fn ws_handler(State(state): State<ServerState>,
                    ws: WebSocketUpgrade,
                    user_agent: Option<TypedHeader<headers::UserAgent>>,
                    ConnectInfo(remote): ConnectInfo<SocketAddr>)
                    -> impl IntoResponse {
  let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
    user_agent.to_string()
  } else {
    String::from("Unknown browser")
  };
  info!(%user_agent, "connected from {remote}.");
  ws.on_upgrade(move |socket| handle_socket(state, socket, remote))
}

#[instrument(skip(state, socket))]
async fn handle_socket(state: ServerState, socket: WebSocket, remote: SocketAddr) {
  let mut sub = state.tx_brd.subscribe();

  let (mut tx, mut rx) = socket.split();

  let _ = tx.send(Message::Text(serde_json::to_string(&WsDriverEvent::Config { config: state.config.as_ref().clone(), }).unwrap()))
            .await;

  loop {
    select! {
      message = rx.next() => {
        let Some(message) = message else { break; };
        let message = match message {
          Ok(message) => message,
          Err(err) => {
            warn!(%err, "failed to receive message, bailing");
            break;
          }
        };

        let message = match message {
          | Message::Close(_) => {
            info!("closed by remote, bailing");
            break;
          }
          | Message::Text(message) => { message }
          | _ => {
            continue;
          }
        };

        let Ok(command) = serde_json::from_str::<WsDriverRequest>(&message) else { continue; };
        debug!(?command, "received");
        match command {
          | WsDriverRequest::SetParameter(request) => {
            if let Err(err) = state.tx_cmd.send(InstanceDriverCommand::SetParameters(request)).await {
              error!("failed to send command, bailing: {err}");
              break;
            }
          },
        }
      },
      Ok(event) = sub.recv() => {
        info!(?event, "received event");
        let event = match event {
          | InstanceDriverEvent::Report(report) => WsDriverEvent::Report(report),
          | InstanceDriverEvent::Connected { .. } => continue,
        };
        let Ok(encoded) = serde_json::to_string_pretty(&event) else { continue; };
        if let Err(err) = tx.send(Message::Text(encoded)).await {
          error!("failed to send message, bailing: {err}");
          break;
        }
      },
      _ = time::sleep(Duration::from_millis(3000)) => {
        if let Err(err) = tx.send(Message::Text(serde_json::to_string_pretty(&WsDriverEvent::KeepAlive).unwrap())).await {
          error!("failed to send keep alive message, bailing: {err}");
          break;
        }
      },
      else => {
        break;
      }
    }
  }

  info!("disconnected.");
}

async fn get_config(State(ServerState { config, .. }): State<ServerState>) -> impl IntoResponse {
  Json((&*config).clone())
}
