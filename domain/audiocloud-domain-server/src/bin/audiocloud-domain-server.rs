/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::path::PathBuf;

use axum::Router;
use clap::Parser;
use tracing::*;

use actix_web::web;
use actix_web::web::ServiceConfig;
use audiocloud_actix_utils::start_http2_server;
use audiocloud_api::{DomainId, ServicePorts};
use audiocloud_domain_server::{config, db, events, fixed_instances, media, models, nats, rest_api, sockets, tasks, DomainContext};
use audiocloud_http::http_server;

#[derive(Parser)]
struct Opts {
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

    #[clap(flatten)]
    http: audiocloud_http::HttpOpts,
}

#[tokio::main]
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

    info!(" ⚡ Messaging");

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

    sockets::init(opts.sockets).await?;

    let port = ServicePorts::DomainServerHttps as u16;

    info!(http = %opts.bind, port, " ==== AudioCloud Domain server ==== ");

    if rest_opts.rest_auth_strategy.is_development() {
        warn!("*** development authentication strategy enabled! ***");
    }

    let ctx = DomainContext { auth_strategy: rest_opts.rest_auth_strategy, };

    let router = Router::with_state(ctx);
    let router = rest_api::configure(router);
    let router = sockets::configure(router);

    http_server(&opts.http, port, router).await?;

    audiocloud_tracing::shutdown(tracing_guard).await;

    Ok(())
}
