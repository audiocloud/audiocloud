use fs::read;
use std::env::args;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, fs};

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{headers, routing::put, Json, Router, TypedHeader};
use tokio::sync::{broadcast, mpsc};
use tokio::{select, spawn};
use tower_http::cors;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::EnvFilter;

use api::driver::{InstanceDriverConfig, InstanceDriverEvent, SetInstanceParameterRequest, WsDriverEvent, WsDriverRequest};
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
                                                       .with_thread_ids(true)
                                                       .with_target(false)
                                                       .with_env_filter(EnvFilter::from_default_env())
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

  let router = Router::new().route("/ws", get(ws_handler))
                            .route("/config", get(get_config))
                            .route("/parameter/:parameter_id/:channel", put(set_parameter))
                            .with_state(ServerState { tx_cmd, tx_brd, config })
                            .layer(cors::CorsLayer::very_permissive());

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

async fn ws_handler(State(state): State<ServerState>,
                    ws: WebSocketUpgrade,
                    user_agent: Option<TypedHeader<headers::UserAgent>>,
                    ConnectInfo(addr): ConnectInfo<SocketAddr>)
                    -> impl IntoResponse {
  let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
    user_agent.to_string()
  } else {
    String::from("Unknown browser")
  };
  info!(%user_agent, %addr, "connected.");
  // finalize the upgrade process by returning upgrade callback.
  // we can customize the callback by sending additional info such as address.
  ws.on_upgrade(move |socket| handle_socket(state, socket, addr))
}

#[instrument(skip(state, socket))]
async fn handle_socket(state: ServerState, mut socket: WebSocket, who: SocketAddr) {
  let mut sub = state.tx_brd.subscribe();

  loop {
    select! {
      Some(Ok(message)) = socket.recv() => {
        let Message::Text(message) = message else { continue; };
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
        let event = match event {
          | InstanceDriverEvent::Report(report) => WsDriverEvent::Report(report),
        };
        let Ok(encoded) = serde_json::to_string_pretty(&event) else { continue; };
        if let Err(err) = socket.send(Message::Text(encoded)).await {
          error!("failed to send message, bailing: {err}");
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
