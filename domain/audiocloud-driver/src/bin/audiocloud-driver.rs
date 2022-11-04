/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::path::PathBuf;
use std::{env, fs};

use actix_web::{web, App, HttpServer};
use clap::Parser;
use reqwest::Url;
use tracing::*;

use audiocloud_actix_utils::start_http2_server;
use audiocloud_api::cloud::domains::InstanceDriverConfig;
use audiocloud_api::InstanceDriverId;
use audiocloud_driver::client::DriverClient;
use audiocloud_driver::http_client;
use audiocloud_driver::nats::NatsOpts;
use audiocloud_driver::rest_api;
use audiocloud_driver::supervisor;
use audiocloud_rust_clients::DomainServerClient;

#[derive(Parser, Debug, Clone)]
struct DriverOpts {
    #[clap(flatten)]
    nats: NatsOpts,

    /// The domain server URL. If set, the instance driver will register with the domain server
    #[clap(long, env)]
    domain_server_url: Option<Url>,

    #[clap(long, env, default_value = "0.0.0.0")]
    bind: String,

    #[clap(long, env, default_value = "7400")]
    port: u16,

    #[clap(long, env, default_value = "default")]
    driver_id: InstanceDriverId,

    // Configuration file (each file can have multiple instances)
    config_files: Vec<PathBuf>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,audiocloud_api=debug,audiocloud_driver=debug");
    }

    tracing_subscriber::fmt::init();

    let opts = DriverOpts::parse();

    http_client::init()?;

    let mut config = InstanceDriverConfig::default();

    for file in &opts.config_files {
        config.merge(serde_yaml::from_reader(fs::File::open(file)?)?);
    }

    let domain_client = match opts.domain_server_url.as_ref() {
        Some(url) => {
            let client = DomainServerClient::new(url.clone())?;
            config = client.register_instance_driver(&opts.driver_id, &config).await?;
            Some(client)
        }
        None => None,
    };

    let supervisor = supervisor::init(opts.driver_id, domain_client, config);
    let client = web::Data::new(DriverClient::new(supervisor));

    info!(bind = opts.bind, port = opts.port, " ==== AudioCloud Driver server ==== ");

    start_http2_server(opts.bind.as_str(), opts.port, move |configure| {
        configure.app_data(client.clone()).configure(rest_api::configure);
    }).await?;

    Ok(())
}
