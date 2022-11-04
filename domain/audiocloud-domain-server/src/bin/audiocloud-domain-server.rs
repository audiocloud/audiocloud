/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::path::PathBuf;

use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use tracing::*;

use audiocloud_actix_utils::start_http2_server;
use audiocloud_domain_server::{config, db, events, fixed_instances, media, models, nats, o11y, rest_api, sockets, tasks};

#[derive(Parser)]
struct Opts {
    /// REST and WebSocket API port
    #[clap(short, long, env, default_value = "7200")]
    port: u16,

    /// REST and WebSocket API host
    #[clap(short, long, env, default_value = "0.0.0.0")]
    bind: String,

    #[clap(short, long, env)]
    tls_cert: Option<PathBuf>,

    /// NATS URL
    #[clap(long, env, default_value = "nats://localhost:4222")]
    nats_url: String,

    #[clap(flatten)]
    db: db::DataOpts,

    #[clap(flatten)]
    media: media::MediaOpts,

    #[clap(flatten)]
    config: config::ConfigOpts,

    #[clap(flatten)]
    sockets: sockets::SocketsOpts,

    #[clap(flatten)]
    tasks: tasks::TaskOpts,

    #[clap(flatten)]
    rest: rest_api::RestOpts,

    #[clap(flatten)]
    o11y: o11y::O11yOpts,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // the domain server is basically a bunch of timers and event handlers running on top of an sqlite database.

    let _ = dotenv::dotenv();

    let mut opts = Opts::parse();

    info!(source = %opts.config.describe(), "Loading config");

    let cfg = config::init(opts.config).await?;

    if opts.o11y.domain_id.is_empty() {
        opts.o11y.domain_id = cfg.domain_id.clone();
    }

    info!(" ⚡ Tracing");

    let _tracing_guard = o11y::init_tracing(&opts.o11y)?;

    info!(" ⚡ Metrics");

    let _metrics_guard = o11y::init_metrics(&opts.o11y)?;

    info!(" ⚡ Database");

    let db = db::init(opts.db).await?;

    info!(" ⚡ NATS");

    let _nats_guard = nats::init(&opts.nats_url).await?;

    info!(" ⚡ Models");

    models::init(&cfg, db.clone()).await?;

    info!(" ⚡ Media");

    media::init(opts.media, db.clone()).await?;

    info!(" ⚡ Instances");

    let routing = fixed_instances::init(&cfg, db.clone()).await?;

    info!(" ⚡ Tasks (Offline)");

    tasks::init(db.clone(), &opts.tasks, &cfg, routing)?;

    info!(" ⚡ Cloud Events");

    events::init(cfg.command_source.clone(), cfg.event_sink.clone()).await?;

    info!(" ⚡ Tasks (Online)");

    tasks::become_online().await?;

    info!(" ⚡ Sockets");

    sockets::init(opts.sockets)?;

    info!(bind = opts.bind, port = opts.port, " ==== AudioCloud Domain server ==== ");

    let rest_opts = web::Data::new(opts.rest.clone());
    if rest_opts.rest_auth_strategy.is_development() {
        warn!("*** development authentication strategy enabled! ***");
    }

    let configure = |service: &mut ServiceConfig| {
        service.configure(rest_api::configure).configure(sockets::configure);
    };

    start_http2_server(opts.bind.as_str(), opts.port, configure).await?;

    Ok(())
}
