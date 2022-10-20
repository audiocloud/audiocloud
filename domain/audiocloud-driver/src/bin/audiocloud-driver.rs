use std::path::PathBuf;
use std::{env, fs};

use actix_web::{App, HttpServer};
use clap::Parser;
use tracing::*;

use audiocloud_driver::nats::NatsOpts;
use audiocloud_driver::rest_api;
use audiocloud_driver::supervisor;
use audiocloud_driver::{http_client, ConfigFile};

#[derive(Parser, Debug, Clone)]
struct DriverOpts {
    #[clap(flatten)]
    nats: NatsOpts,

    // Configuration file (array of instances)
    config_file: PathBuf,

    #[clap(long, env, default_value = "0.0.0.0")]
    bind: String,

    #[clap(long, env, default_value = "7400")]
    port: u16,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            "info,audiocloud_api=debug,audiocloud_driver=debug",
        );
    }

    tracing_subscriber::fmt::init();

    let opts = DriverOpts::parse();

    http_client::init()?;

    let instances = serde_yaml::from_reader::<_, ConfigFile>(fs::File::open(opts.config_file)?)?;

    supervisor::init(opts.nats, instances).await?;

    info!(
        bind = opts.bind,
        port = opts.port,
        " ==== AudioCloud Driver server ==== "
    );

    HttpServer::new(move || App::new().configure(rest_api::configure))
        .bind((opts.bind.as_str(), opts.port))?
        .run()
        .await?;

    Ok(())
}
