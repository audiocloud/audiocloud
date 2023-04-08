use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use tracing::instrument;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::instance::spec::InstanceSpec;
use api::BucketKey;
use domain_server::nats::Nats;

const LOG_DEFAULTS: &'static str = "warn";

type Result<T = ()> = anyhow::Result<T>;

#[derive(Debug, Parser)]
struct Arguments {
  #[clap(long, env, default_value = "nats://localhost:4222")]
  pub nats_url: String,
  #[clap(subcommand)]
  command:      Command,
}

#[derive(Debug, Subcommand)]
enum Command {
  Instance {
    #[clap(subcommand)]
    command: InstanceCommand,
  },
}

#[derive(Debug, Subcommand)]
enum InstanceCommand {
  /// Put an instance spec into the store
  Put {
    /// Instance Id
    id:   String,
    /// File to import
    path: PathBuf,
  },
  List {
    #[clap(short, long, default_value = "yaml")]
    format: OutputFormat,
    /// Filter by name
    #[clap(default_value = "*")]
    filter: String,
  },
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
  Yaml,
  Json,
}

impl FromStr for OutputFormat {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      | "yaml" => Ok(OutputFormat::Yaml),
      | "json" => Ok(OutputFormat::Json),
      | _ => Err(anyhow!("Invalid output format")),
    }
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_DEFAULTS.into()))
                                .with(tracing_subscriber::fmt::layer())
                                .init();

  let args = Arguments::parse();
  let client = async_nats::connect(&args.nats_url).await?;
  let nats = Nats::new(client).await?;

  match args.command {
    | Command::Instance { command } => instance_command(nats, command).await,
  }
}

#[instrument(err, skip(nats))]
async fn instance_command(nats: Nats, cmd: InstanceCommand) -> Result {
  match cmd {
    | InstanceCommand::Put { id, path } => put_instance(nats, id, path).await,
    | InstanceCommand::List { format, filter } => list_instances(nats, filter, format).await,
  }
}

async fn list_instances(nats: Nats, filter: String, format: OutputFormat) -> Result {
  let list = nats.instance_spec.scan(&filter).await?;
  for (id, spec) in list {
    match format {
      | OutputFormat::Yaml => {
        println!("# instance {id}\n{}---", serde_yaml::to_string(&spec)?);
      }
      | OutputFormat::Json => {
        let mut json = serde_json::to_value(&spec)?;
        json.as_object_mut()
            .unwrap()
            .insert("id".to_string(), serde_json::Value::String(id));
        println!("{}", serde_json::to_string(&json)?);
      }
    }
  }

  Ok(())
}

#[instrument(err, skip(nats))]
async fn put_instance(nats: Nats, id: String, path: PathBuf) -> Result {
  let spec = serde_yaml::from_reader::<_, InstanceSpec>(File::open(&path)?)?;

  nats.instance_spec.put(BucketKey::new(id), spec).await?;

  Ok(())
}
