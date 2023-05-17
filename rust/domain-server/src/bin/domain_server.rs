use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use axum::http::header::{ACCEPT, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE, COOKIE, HOST, IF_MATCH, IF_NONE_MATCH, IF_UNMODIFIED_SINCE};
use axum::http::Method;
use axum::Router;
use clap::Parser;
use futures::channel::mpsc;
use futures::{FutureExt, SinkExt, StreamExt};
use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use tokio::{select, spawn};
use tower_http::cors;
use tower_http::cors::AllowOrigin;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use domain_server::instance::driver::scripting::new_scripting_engine;
use domain_server::media::service::MediaService;
use domain_server::nats::Nats;
use domain_server::service::{Service, ServiceConfig};
use domain_server::Result;

const LOG_DEFAULTS: &'static str = "info,domain_server=trace,tower_http=debug";

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
struct Arguments {
  /// Enables instance services
  #[clap(long, env)]
  pub enable_instances:        bool,
  /// Enables instance driver services
  #[clap(long, env)]
  pub enable_instance_drivers: bool,
  /// Enables task management services
  #[clap(long, env)]
  pub enable_tasks:            bool,
  /// Enables REST API
  #[clap(long, env)]
  pub enable_api:              bool,
  /// Enable media management services
  #[clap(long, env)]
  pub enable_media:            bool,
  /// NATS JetStream URL
  #[clap(long, env, default_value = "nats://localhost:4222")]
  pub nats_url:                String,
  /// REST API listen address and port
  #[clap(long, env, default_value = "127.0.0.1:7200")]
  pub rest_api_bind:           SocketAddr,
  /// The host name of the domain server
  #[clap(long, env, default_value = "localhost.localdomain")]
  pub driver_host_name:        String,
  /// Media root directory
  #[clap(long, env, default_value = ".media")]
  pub media_root:              PathBuf,
  /// Native (default) sample rate.
  #[clap(long, env, default_value = "192000")]
  pub native_sample_rate:      u32,
  /// Reset the key value database before starting
  #[clap(long, env)]
  pub reset_kv_database:       bool,
  /// Secret used to sign the Json Web Tokens (JWTs) used for authentication
  #[clap(long, env, default_value = "6ChatwwXQMLRYo9GbtqhwshxhzauquhY")]
  pub jwt_secret:              String,
}

enum InternalEvent {
  InstanceDriversFinished,
  InstancesFinished,
  TasksFinished,
  MediaFinished,
  RestApiFinished,
  ScriptingFinished,
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
  let config = ServiceConfig { jwt_secret: args.jwt_secret.clone(), };

  debug!(url = &args.nats_url, "Connecting to NATS");
  let nats = async_nats::connect(&args.nats_url).await?;
  let service = Nats::new(nats, args.reset_kv_database).await?;
  let service = Service { nats:   service.clone(),
                          config: Arc::new(config), };

  let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
  let router = Router::new().fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true));

  let router = domain_server::rest_api::rest_api(router, service.clone());

  let (tx_internal, mut rx_internal) = mpsc::channel(0xff);

  let (scripting_engine, scripting_handle) = new_scripting_engine();

  let create_instance_drivers = || {
    if args.enable_instance_drivers {
      let mut tx_internal = tx_internal.clone();
      info!("Starting instance driver service: {}", args.driver_host_name);
      let service = domain_server::instance::driver::server::DriverService::new(service.clone(),
                                                                                scripting_engine.clone(),
                                                                                args.driver_host_name.clone());
      spawn(service.run().then(|res| async move {
                           warn!("Instance driver service exited: {res:?}");
                           let _ = tx_internal.send(InstanceDriversFinished).await;
                         }));
    }
  };
  let instance_drivers_respawn_limit = Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(10u32))));

  let create_instances = || {
    if args.enable_instances {
      let mut tx_internal = tx_internal.clone();
      info!("Starting instances service");
      let service = domain_server::instance::service::InstanceService::new(service.clone());
      spawn(service.run().then(|res| async move {
                           warn!("Instance service exited: {res:?}");
                           let _ = tx_internal.send(InstancesFinished).await;
                         }));
    }
  };

  let instances_respawn_limit = Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(10u32))));

  let create_tasks = || {
    if args.enable_tasks {
      let mut tx_internal = tx_internal.clone();
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
      let service = service.clone();
      let args = args.clone();
      let mut tx_internal = tx_internal.clone();
      spawn(async move {
              info!("Starting media service");
              MediaService::new(service.clone(), args.media_root.clone(), args.native_sample_rate).run()
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
      let service = service.clone();
      let bind = args.rest_api_bind;
      let mut tx_internal = tx_internal.clone();
      info!("REST API listening on {bind}");
      spawn(async move {
              let router = router.layer(cors::CorsLayer::new().allow_credentials(true)
                                                              .allow_headers([AUTHORIZATION,
                                                                              ACCEPT,
                                                                              CONTENT_TYPE,
                                                                              CACHE_CONTROL,
                                                                              HOST,
                                                                              IF_MATCH,
                                                                              IF_NONE_MATCH,
                                                                              IF_UNMODIFIED_SINCE,
                                                                              COOKIE])
                                                              .allow_methods([Method::GET,
                                                                              Method::POST,
                                                                              Method::PUT,
                                                                              Method::DELETE,
                                                                              Method::OPTIONS,
                                                                              Method::CONNECT])
                                                              .allow_origin(AllowOrigin::mirror_request()))
                                 .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
                                 .with_state(service.clone());

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

  spawn({
    let mut tx_internal = tx_internal.clone();

    async move {
      let _ = scripting_handle.join().await;
      let _ = tx_internal.send(ScriptingFinished).await;
    }
  });

  loop {
    select! {
      Some(internal) = rx_internal.next() => {
        match internal {
          InstanceDriversFinished => {
            let mut tx_internal = tx_internal.clone();
            let instance_drivers_respawn_limit = instance_drivers_respawn_limit.clone();
            spawn(async move {
              let _ = instance_drivers_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartInstanceDrivers).await;
            });
          },
          InstancesFinished => {
            let mut tx_internal = tx_internal.clone();
            let instances_respawn_limit = instances_respawn_limit.clone();
            spawn(async move {
              let _ = instances_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartInstances).await;
            });
          },
          TasksFinished => {
            let mut tx_internal = tx_internal.clone();
            let tasks_respawn_limit = tasks_respawn_limit.clone();
            spawn(async move {
              let _ = tasks_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartTasks).await;
            });
          },
          MediaFinished => {
            let mut tx_internal = tx_internal.clone();
            let media_respawn_limit = media_respawn_limit.clone();
            spawn(async move {
              let _ = media_respawn_limit.until_ready().await;
              let _ = tx_internal.send(RestartMedia).await;
            });
          },
          RestApiFinished => {
            error!("FATAL: Rest API stopped, exiting");
            break;
          },
          ScriptingFinished => {
            error!("FATAL: Scripting engine stopped, exiting");
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
