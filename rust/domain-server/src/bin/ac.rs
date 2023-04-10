use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};
use tracing::instrument;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::spec::InstanceSpec;
use api::instance::{DesiredInstancePlayState, DesiredInstancePowerState};
use api::task::spec::PlayId;
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
  /// Describe instance and its state in detail
  Describe {
    /// Include spec along with state.
    #[clap(long)]
    include_spec: bool,
    /// Instance Id
    id:           String,
  },
  /// List instance specs
  List {
    /// Output format
    #[clap(short, long, default_value = "yaml")]
    format:  OutputFormat,
    /// Only show the instance id, not the spec body
    #[clap(long)]
    only_id: bool,
    /// Filter by name
    #[clap(default_value = "*")]
    filter:  String,
  },
  /// Manage instance power control
  Power {
    /// Instance Id
    id:      String,
    /// Power command
    #[clap(subcommand)]
    command: InstancePowerCommand,
  },
  /// Manage instance play control
  Play {
    /// Instance Id
    id:      String,
    /// Power command
    #[clap(subcommand)]
    command: InstancePlayCommand,
  },
}

#[derive(Debug, Subcommand)]
enum InstancePowerCommand {
  /// Power on the instance
  On {
    /// How long to latch the play state
    #[clap(long, short, default_value = "3600")]
    duration: f64,
  },
  /// Power off the instance
  Off,
}

#[derive(Debug, Subcommand)]
enum InstancePlayCommand {
  /// Start the instance
  Play {
    /// How long to latch the play state
    #[clap(long, short, default_value = "60")]
    duration: f64,
  },
  /// Stop the instance
  Stop,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
  Yaml,
  Json,
}

impl FromStr for OutputFormat {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
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
    | InstanceCommand::List { format, only_id, filter } => list_instances(nats, filter, format, only_id).await,
    | InstanceCommand::Describe { include_spec, id } => describe_instance(nats, id, include_spec).await,
    | InstanceCommand::Power { id, command } => set_instance_power(nats, id, command).await,
    | InstanceCommand::Play { id, command } => set_instance_play(nats, id, command).await,
  }
}

async fn set_instance_play(nats: Nats, id: String, play: InstancePlayCommand) -> Result {
  let play = match play {
    | InstancePlayCommand::Play { duration } => InstancePlayControl { desired: DesiredInstancePlayState::Play { duration: duration as f64,
                                                                                                                play_id:  PlayId::MAX, },
                                                                      until:   Utc::now()
                                                                               + Duration::milliseconds((duration * 1000.0) as i64), },
    | InstancePlayCommand::Stop => InstancePlayControl { desired: DesiredInstancePlayState::Stop,
                                                         until:   Utc::now(), },
  };

  nats.instance_play_ctrl.put(BucketKey::new(id), play).await?;

  Ok(())
}

async fn set_instance_power(nats: Nats, id: String, power: InstancePowerCommand) -> Result {
  let power = match power {
    | InstancePowerCommand::On { duration } => InstancePowerControl { desired: DesiredInstancePowerState::On,
                                                                      until:   Utc::now()
                                                                               + Duration::milliseconds((duration * 1000.0) as i64), },
    | InstancePowerCommand::Off => InstancePowerControl { desired: DesiredInstancePowerState::Off,
                                                          until:   Utc::now(), },
  };

  nats.instance_power_ctrl.put(BucketKey::new(id), power).await?;

  Ok(())
}

async fn describe_instance(nats: Nats, id: String, include_spec: bool) -> Result {
  let spec = nats.instance_spec.get(BucketKey::new(&id)).await?;
  let state = if include_spec {
    nats.instance_state.get(BucketKey::new(&id)).await?
  } else {
    None
  };
  let power = nats.instance_power_ctrl.get(BucketKey::new(&id)).await?;
  let play = nats.instance_play_ctrl.get(BucketKey::new(&id)).await?;

  println!("Instance: {id}");
  if include_spec {
    println!(" * Spec: {}", serde_json::to_string_pretty(&spec).unwrap());
  }
  println!(" * State: {}", serde_json::to_string_pretty(&state).unwrap());
  println!(" * Power: {}", serde_json::to_string_pretty(&power).unwrap());
  println!(" * Play: {}", serde_json::to_string_pretty(&play).unwrap());

  Ok(())
}

async fn list_instances(nats: Nats, filter: String, format: OutputFormat, only_id: bool) -> Result {
  let list = nats.instance_spec.scan(&filter).await?;
  for (id, spec) in list {
    if only_id {
      println!("{id}");
      continue;
    }

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
