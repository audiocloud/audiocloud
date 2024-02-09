use std::net::SocketAddr;

use axum::Router;
use axum_connect::prelude::*;
use clap::Parser;
use hmac::digest::KeyInit;
use hmac::Hmac;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::filter::{Directive, LevelFilter};
use tracing_subscriber::prelude::*;

use api_proto::{DomainInstanceDriverService, DomainInstanceService, DomainSecurityService};
use domain_db::Db;

use crate::context::ServiceContextFactory;

pub mod args;
pub mod context;
pub mod db_init;
pub mod error;
pub mod instance;
pub mod instance_driver;
pub mod nats;
pub mod registry;
pub mod security;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = args::Opts::parse();

  let filter = tracing_subscriber::EnvFilter::from(&args.log);
  if args.verbose {
    filter.add_directive(Directive::from(LevelFilter::DEBUG));
  }

  tracing_subscriber::registry().with(filter)
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let in_mem_db = Db::new(&args.db_url, args.db_root, &args.db_namespace, &args.db_username, &args.db_password).await?;

  let nats_client = async_nats::connect(&args.nats_url).await?;

  if args.db_init {
    db_init::run(&in_mem_db).await?;
  }

  let factory = ServiceContextFactory { db:         in_mem_db,
                                        nats:       nats_client,
                                        token_hmac: Hmac::new_from_slice(args.token_secret.as_bytes())?, };

  let mut app = Router::new();

  if args.enable_domain_security_service {
    app = app.rpc(DomainSecurityService::user_login(security::user_login_handler))
             .rpc(DomainSecurityService::create_token(security::create_token_handler))
             .rpc(DomainSecurityService::invalidate_token(security::invalidate_token_handler))
             .rpc(DomainSecurityService::describe_token(security::describe_token_handler))
             .rpc(DomainSecurityService::register_user(security::register_user_handler))
             .rpc(DomainSecurityService::register_app(security::register_app_handler))
             .rpc(DomainSecurityService::update_app(security::update_app_handler))
             .rpc(DomainSecurityService::create_api_key(security::create_api_key_handler))
             .rpc(DomainSecurityService::invalidate_api_key(security::invalidate_api_key_handler))
             .rpc(DomainSecurityService::list_users(security::list_users_handler))
             .rpc(DomainSecurityService::list_apps(security::list_apps_handler))
             .rpc(DomainSecurityService::list_api_keys(security::list_api_keys_handler));
  }

  if args.enable_domain_instance_service {
    // TODO: implement
    app = app.rpc(DomainInstanceService::set_instance_spec(instance::set_instance_spec_handler))
             .rpc(DomainInstanceService::set_parameter(instance::set_parameter_handler))
             .rpc(DomainInstanceService::set_instance_desired_media_state(instance::set_instance_desired_media_state_handler))
             .rpc(DomainInstanceService::list_instances(instance::list_instances_handler))
             .rpc(DomainInstanceService::describe_instance(instance::describe_instance_handler))
             .rpc(DomainInstanceService::claim_instance(instance::claim_instance_handler))
             .rpc(DomainInstanceService::release_instance(instance::release_instance_handler))
             .rpc(DomainInstanceService::add_instance_maintenance(instance::add_instance_maintenance_handler))
             .rpc(DomainInstanceService::update_instance_maintenance(instance::update_instance_maintenance_handler))
             .rpc(DomainInstanceService::subscribe_instance_events(instance::subscribe_instance_events_handler));
  }

  if args.enable_instance_drivers_service {
    app = app.rpc(DomainInstanceDriverService::set_parameter(instance_driver::set_parameter_handler))
             .rpc(DomainInstanceDriverService::subscribe_instance_events(instance_driver::subscribe_instance_events_handler))
  }

  if args.enable_media_service {
    // TODO: implement
  }

  if args.enable_tasks_service {
    // TODO: implement
  }

  let app = app.layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
               .with_state(factory);

  let addr = SocketAddr::from(([127, 0, 0, 1], args.api_port));
  println!("listening on http://{}", addr);
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

  Ok(())
}
