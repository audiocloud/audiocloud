use std::path::PathBuf;
use std::{env, fs};

use actix_web::{web, App, HttpServer};
use clap::Parser;
use reqwest::Url;
use tracing::*;

use audiocloud_api::cloud::domains::InstanceDriverConfig;
use audiocloud_api::InstanceDriverId;
use audiocloud_driver::drivers;
use audiocloud_driver::http_client;
use audiocloud_driver::nats::NatsOpts;
use audiocloud_driver::rest_api;
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

    if let Some(url) = opts.domain_server_url.as_ref() {
        let client = DomainServerClient::new(url.clone())?;
        config = client.register_instance_driver(&opts.driver_id, &config).await?;
    }

    let drivers = drivers::init(config)?;
    let drivers = web::Data::new(drivers);

    info!(bind = opts.bind, port = opts.port, " ==== AudioCloud Driver server ==== ");

    HttpServer::new(move || App::new().app_data(drivers.clone()).configure(rest_api::configure)).bind((opts.bind.as_str(), opts.port))?
                                                                                                .run()
                                                                                                .await?;

    Ok(())
}
