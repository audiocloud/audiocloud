use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use axum::Router;
use clap::Parser;
use futures::FutureExt;
use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use tokio::sync::mpsc;
use tokio::{select, spawn};
use tower_http::cors;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use domain_server::instance::driver::scripting::new_scripting_engine;
use domain_server::media::MediaService;
use domain_server::nats::Nats;
use domain_server::service::Service;
use domain_server::Result;

const LOG_DEFAULTS: &'static str = "info,domain_server=trace,tower_http=debug";

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
struct Arguments {
  /// Enables instance services
  #[clap(long, env, group = "instances")]
  pub enable_instances:        bool,
  /// Enables instance driver services
  #[clap(long, env, group = "instance_drivers")]
  pub enable_instance_drivers: bool,
  /// Enables task management services
  #[clap(long, env, group = "tasks")]
  pub enable_tasks:            bool,
  /// Enables REST API
  #[clap(long, env, group = "api")]
  pub enable_api:              bool,
  /// Enable media management services
  #[clap(long, env, group = "media")]
  pub enable_media:            bool,
  /// NATS JetStream URL
  #[clap(long, env, default_value = "nats://localhost:4222")]
  pub nats_url:                String,
  /// REST API listen address and port
  #[clap(long, env, default_value = "127.0.0.1:7200", group = "api")]
  pub rest_api_bind:           SocketAddr,
  /// The host name of the domain server
  pub host_name:               String,
  /// Media root directory
  #[clap(long, env, default_value = ".media", group = "media")]
  pub media_root:              PathBuf,
  /// Native (default) sample rate.
  #[clap(long, env, default_value = "192000")]
  pub native_sample_rate:      u32,
}

enum InternalEvent {
  InstanceDriversFinished,
  InstancesFinished,
  TasksFinished,
  MediaFinished,
  RestApiFinished,
  RestartInstanceDrivers,
  RestartInstances,
  RestartTasks,
  RestartMedia,
}

#[tokio::main]
async fn main() -> Result {
  use InternalEvent::*;

  tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_DEFAULTS.into()))
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let args = Arguments::parse();

  debug!(url = &args.nats_url, "Connecting to NATS");
  let nats = async_nats::connect(&args.nats_url).await?;
  let nats = Nats::new(nats).await?;
  let service = Service { nats: nats.clone() };

  let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
  let router = Router::new().fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true));

  let router = domain_server::rest_api::rest_api(router);

  let (tx_internal, mut rx_internal) = mpsc::channel(0xff);

  let (scripting_engine, scripting_handle) = new_scripting_engine();

  let create_instance_drivers = || {
    if args.enable_instance_drivers {
      let tx_internal = tx_internal.clone();
      let service =
        domain_server::instance::driver::server::DriverService::new(nats.clone(), scripting_engine.clone(), args.host_name.clone());
      spawn(service.run().then(|res| async move {
                           warn!("Instance driver service exited: {res:?}");
                           let _ = tx_internal.send(InstanceDriversFinished).await;
                         }));
    }
  };
  let instance_drivers_respawn_limit = Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(10u32))));

  let create_instances = || {
    if args.enable_instances {
      let tx_internal = tx_internal.clone();
      let service = domain_server::instance::service::InstanceService::new(nats.clone());
      spawn(service.run().then(|res| async move {
                           warn!("Instance service exited: {res:?}");
                           let _ = tx_internal.send(InstancesFinished).await;
                         }));
    }
  };

  let instances_respawn_limit = Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(10u32))));

  let create_tasks = || {
    if args.enable_tasks {
      let tx_internal = tx_internal.clone();
      spawn(async move {
              error!("Tasks service is not yet implemented");
              Err::<(), _>(anyhow!("Tasks service is not yet implemented"))
            }.then(|res| async move {
               warn!("Tasks service exited: {res:?}");
               let _ = tx_internal.send(TasksFinished).await;
             }));
    }
  };
  let tasks_respawn_limit = Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(10u32))));

  let create_media = || {
    if args.enable_media {
      let nats = nats.clone();
      let args = args.clone();
      let tx_internal = tx_internal.clone();
      spawn(async move {
              MediaService::new(nats.clone(), args.media_root.clone(), args.native_sample_rate).run()
                                                                                               .await
            }.then(|res| async move {
               warn!("Media service exited: {res:?}");
               let _ = tx_internal.send(MediaFinished).await;
             }));
    }
  };
  let media_respawn_limit = Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(10u32))));

  let create_rest_api = || {
    if args.enable_api {
      let bind = args.rest_api_bind;
      let tx_internal = tx_internal.clone();
      info!("REST API listening on {bind}");
      spawn(async move {
              let router = router.layer(cors::CorsLayer::permissive())
                                 .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
                                 .with_state(service);

              Ok::<_, anyhow::Error>(axum::Server::bind(&bind).serve(router.into_make_service_with_connect_info::<SocketAddr>())
                                                              .await?)
            }.then(|res| async move {
               warn!("REST API exited: {res:?}");
               let _ = tx_internal.send(RestApiFinished).await;
             }));
    }
  };

  create_instance_drivers();
  create_instances();
  create_tasks();
  create_media();
  create_rest_api();

  loop {
    select! {
      Some(internal) = rx_internal.recv() => {
        match internal {
          InstanceDriversFinished => {
            let tx_internal = tx_internal.clone();
            let instance_drivers_respawn_limit = instance_drivers_respawn_limit.clone();
            spawn(async move {
              let _ = instance_drivers_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartInstanceDrivers).await;
            });
          },
          InstancesFinished => {
            let tx_internal = tx_internal.clone();
            let instances_respawn_limit = instances_respawn_limit.clone();
            spawn(async move {
              let _ = instances_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartInstances).await;
            });
          },
          TasksFinished => {
            let tx_internal = tx_internal.clone();
            let tasks_respawn_limit = tasks_respawn_limit.clone();
            spawn(async move {
              let _ = tasks_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartTasks).await;
            });
          },
          MediaFinished => {
            let tx_internal = tx_internal.clone();
            let media_respawn_limit = media_respawn_limit.clone();
            spawn(async move {
              let _ = media_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartMedia).await;
            });
          },
          RestApiFinished => {
            error!("Rest API stopped, exiting");
            break;
          },
          RestartInstanceDrivers => {
            create_instance_drivers();
          },
          RestartInstances => {
            create_instances();
          },
          RestartTasks => {
            create_tasks();
          },
          RestartMedia => {
            create_media();
          }
        }
      },
      _ = tokio::signal::ctrl_c() => {
        info!("Caught ctrl+c -> Exiting...");
        break;
      },
      else => {
        break;
      }
    }
  }

  Ok(())
}
