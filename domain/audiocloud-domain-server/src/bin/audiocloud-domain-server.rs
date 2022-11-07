/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::path::PathBuf;

use actix_web::web;
use actix_web::web::ServiceConfig;
use clap::Parser;
use tracing::*;

use audiocloud_actix_utils::start_http2_server;
use audiocloud_api::DomainId;
use audiocloud_domain_server::{config, db, events, fixed_instances, media, models, nats, rest_api, sockets, tasks};

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

    /// Name of this domain server instance
    #[clap(long, env, default_value = "")]
    domain_id: DomainId,

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
    o11y: audiocloud_tracing::O11yOpts,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // the domain server is basically a bunch of timers and event handlers running on top of an sqlite database.

    let _ = dotenv::dotenv();

    let mut opts = Opts::parse();

    info!(source = %opts.config.describe(), "Loading config");

    let cfg = config::init(opts.config).await?;

    if opts.domain_id.is_empty() {
        opts.domain_id = cfg.domain_id.clone();
    }

    info!(" ⚡ Tracing");

    let tracing_guard = audiocloud_tracing::init(&opts.o11y, "audiocloud-domain", opts.domain_id.as_str())?;

    info!(" ⚡ Metrics");

    let _metrics_guard = audiocloud_tracing::init_metrics(&opts.o11y, "audiocloud-domain", opts.domain_id.as_str())?;

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

    audiocloud_tracing::shutdown(tracing_guard).await;

    Ok(())
}
