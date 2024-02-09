use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "domain-server")]
#[command(author = "baadc0de <bojan@distopik.com>")]
#[command(version = "0.3")]
#[command(about = "Runs an audiocloud domain server")]
pub(crate) struct Opts {
  #[clap(long, env, default_value = "nats://localhost:4222")]
  pub nats_url: String,
  #[clap(long, env, default_value = "ws://localhost:8000")]
  pub db_url: String,
  #[clap(long, env, default_value = "audiocloud")]
  pub db_namespace: String,
  #[clap(long, env, default_value = "root")]
  pub db_username: String,
  #[clap(long, env, default_value = "root")]
  pub db_password: String,
  #[clap(long, env)]
  pub db_root: bool,
  #[clap(long)]
  pub db_init: bool,
  #[clap(long, env, default_value = "http://localhost:8500")]
  pub consul_url: String,
  #[clap(long, env)]
  pub enable_domain_security_service: bool,
  #[clap(long, env)]
  pub enable_domain_instance_service: bool,
  #[clap(long, env)]
  pub enable_instance_drivers_service: bool,
  #[clap(long, env)]
  pub enable_media_service: bool,
  #[clap(long, env)]
  pub enable_tasks_service: bool,
  #[clap(long, env)]
  pub hostname: Option<String>,
  #[clap(long, env, default_value = "7200")]
  pub api_port: u16,
  #[clap(long, env = "RUST_LOG", default_value = "info,domain_server_proto=trace,tower_http=debug")]
  pub log: String,
  #[clap(long, env, default_value = "cli1aut1w0000p322bmgj2tos01H165PPBRJH2N308A8XARYTED")]
  pub token_secret: String,
  #[clap(short, long)]
  pub verbose: bool,
}
