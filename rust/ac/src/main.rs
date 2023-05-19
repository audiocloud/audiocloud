use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use argon2::{Algorithm, Argon2, Version};
use async_nats::Client;
use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};
use futures::StreamExt;
use password_hash::{PasswordHasher, SaltString};
use tracing::{info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::instance::control::{instance_play_control_key, instance_power_control_key, InstancePlayControl, InstancePowerControl};
use api::instance::driver::config::InstanceDriverConfig;
use api::instance::spec::InstanceSpec;
use api::instance::state::{instance_connection_state_key, instance_play_state_key, instance_power_state_key};
use api::instance::{DesiredInstancePlayState, DesiredInstancePowerState};
use api::media::buckets::{media_upload_spec_key, media_upload_state_key};
use api::media::spec::{MediaDownloadSpec, MediaId, MediaUploadSpec};
use api::media::state::media_download_state_key;
use api::task::player::PlayId;
use api::user::UserSpec;
use api::BucketKey;
use domain_service::nats::Nats;

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
  /// Instance spec, power and play management
  Instance {
    #[clap(subcommand)]
    command: InstanceCommand,
  },
  /// Media downloads and uploads management
  Media {
    #[clap(subcommand)]
    command: MediaCommand,
  },
  /// User management
  User {
    #[clap(subcommand)]
    command: UserCommand,
  },
  /// Key-value database management
  KV {
    #[clap(subcommand)]
    command: KVCommand,
  },
}

#[derive(Debug, Subcommand)]
enum KVCommand {
  /// Reset the key value database
  Reset,
}

#[derive(Debug, Subcommand)]
enum InstanceCommand {
  /// Put an instance spec into the store
  Put {
    /// Instance Id
    id:     String,
    /// File to import
    path:   PathBuf,
    /// Override the host in the spec with this value
    #[clap(long)]
    host:   Option<String>,
    /// If this is specified, the instance will not use the specified driver but a mocked driver which always works
    #[clap(long)]
    mocked: bool,
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
enum MediaCommand {
  Download {
    /// Media id
    id:      MediaId,
    #[clap(subcommand)]
    command: MediaDownloadCommand,
  },
  Upload {
    /// Media id
    id:      MediaId,
    #[clap(subcommand)]
    command: MediaUploadCommand,
  },
}

#[derive(Debug, Subcommand)]
enum UserCommand {
  /// Create a user
  Create {
    /// User id
    id:    String,
    /// User email, if any
    email: String,
  },
  /// Delete a user
  Delete {
    /// User id
    id: String,
  },
  /// List users
  List {
    /// Output format
    #[clap(short, long, default_value = "yaml")]
    format: OutputFormat,
  },
  /// Describe a user
  Describe {
    /// User id
    id: String,
  },
  SetPassword {
    /// User id
    id: String,
  },
}

#[derive(Debug, Subcommand)]
enum MediaDownloadCommand {
  /// Create a download request
  Create {
    /// Source URL to GET the file from
    url:    String,
    /// Sha 256 hash
    sha256: String,
    /// Size of the file in bytes
    size:   Option<u64>,
  },
  /// Get status of download
  Status,
}

#[derive(Debug, Subcommand)]
enum MediaUploadCommand {
  /// Create an upload request
  Create {
    /// Destination URL to PUT the file to
    url: String,
  },
  /// Get status of upload
  Status,
}

#[derive(Debug, Subcommand)]
enum InstancePowerCommand {
  /// Remove power control from the instance
  Delete,
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
  /// Remove play control from the instance
  Delete,
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

  match args.command {
    | Command::Instance { command } => instance_command(Nats::new(client, false).await?, command).await,
    | Command::Media { command } => media_command(Nats::new(client, false).await?, command).await,
    | Command::User { command } => user_command(Nats::new(client, false).await?, command).await,
    | Command::KV { command } => kv_command(client, command).await,
  }
}

#[instrument(err)]
async fn kv_command(client: Client, command: KVCommand) -> Result {
  match command {
    | KVCommand::Reset => kv_reset(client).await,
  }
}

async fn kv_reset(client: Client) -> Result {
  Nats::new(client, true).await?;

  info!("KV database fully reset");

  Ok(())
}

#[instrument(err, skip(nats))]
async fn instance_command(nats: Nats, cmd: InstanceCommand) -> Result {
  match cmd {
    | InstanceCommand::Put { id, path, host, mocked } => put_instance(nats, id, path, host, mocked).await,
    | InstanceCommand::List { format, only_id, filter } => list_instances(nats, filter, format, only_id).await,
    | InstanceCommand::Describe { include_spec, id } => describe_instance(nats, id, include_spec).await,
    | InstanceCommand::Power { id, command } => set_instance_power(nats, id, command).await,
    | InstanceCommand::Play { id, command } => set_instance_play(nats, id, command).await,
  }
}

#[instrument(err, skip(nats))]
async fn media_command(nats: Nats, cmd: MediaCommand) -> Result {
  match cmd {
    | MediaCommand::Download { id, command } => media_download_command(nats, id, command).await,
    | MediaCommand::Upload { id, command } => media_upload_command(nats, id, command).await,
  }
}

async fn media_download_command(nats: Nats, id: MediaId, cmd: MediaDownloadCommand) -> Result {
  match cmd {
    | MediaDownloadCommand::Create { url, sha256, size } => create_media_download(nats, id, url, sha256, size).await,
    | MediaDownloadCommand::Status => media_download_status(nats, id).await,
  }
}

async fn media_upload_command(nats: Nats, id: MediaId, cmd: MediaUploadCommand) -> Result {
  match cmd {
    | MediaUploadCommand::Create { url } => create_media_upload(nats, id, url).await,
    | MediaUploadCommand::Status => media_upload_status(nats, id).await,
  }
}

async fn create_media_download(nats: Nats, id: MediaId, url: String, sha256: String, size: Option<u64>) -> Result {
  let spec = MediaDownloadSpec { from_url: url,
                                 size: size.unwrap_or_default(),
                                 sha256 };

  let revision = nats.media_download_spec.put((&id).into(), spec).await?;

  println!("Download created with revision {revision}");

  Ok(())
}

async fn create_media_upload(nats: Nats, id: MediaId, url: String) -> Result {
  let spec = MediaUploadSpec { to_url: url };

  let revision = nats.media_upload_spec.put(media_upload_spec_key(&id), spec).await?;

  println!("Upload created with revision {revision}");

  Ok(())
}

async fn media_download_status(nats: Nats, id: MediaId) -> Result {
  let mut watch = nats.media_download_state.watch(media_download_state_key(&id));

  println!("Download of media {id}");

  while let Some((_, state)) = watch.next().await {
    match state {
      | None => {
        println!("Deleted or not found");
        break;
      }
      | Some(state) => {
        println!(" * Progress: {:?}", state.progress);

        if let Some(error) = state.error {
          println!(" * Error: {error}");
          break;
        } else if let Some(done) = state.done {
          println!(" * Download complete: {done:?}");
          break;
        }
      }
    }
  }

  Ok(())
}

async fn media_upload_status(nats: Nats, id: MediaId) -> Result {
  let mut watch = nats.media_upload_state.watch(media_upload_state_key(&id));

  println!("Upload of media {id}");

  while let Some((_, state)) = watch.next().await {
    match state {
      | None => {
        println!("Deleted or not found");
        break;
      }
      | Some(state) => {
        println!(" * Progress: {:?}", state.progress);

        if let Some(error) = state.error {
          println!(" * Error: {error}");
          break;
        } else if state.uploaded {
          println!(" * Upload complete!");
          break;
        }
      }
    }
  }

  Ok(())
}

async fn set_instance_play(nats: Nats, id: String, play: InstancePlayCommand) -> Result {
  let play = match play {
    | InstancePlayCommand::Play { duration } => InstancePlayControl { desired: DesiredInstancePlayState::Play { duration: duration as f64,
                                                                                                                play_id:  PlayId::MAX, },
                                                                      until:   Utc::now()
                                                                               + Duration::milliseconds((duration * 1000.0) as i64), },
    | InstancePlayCommand::Stop => InstancePlayControl { desired: DesiredInstancePlayState::Stop,
                                                         until:   Utc::now(), },
    | InstancePlayCommand::Delete => {
      nats.instance_play_ctrl.delete(id.into()).await?;
      return Ok(());
    }
  };

  let revision = nats.instance_play_ctrl.put(id.into(), play).await?;

  println!("Play state updated with revision {revision}");

  Ok(())
}

async fn set_instance_power(nats: Nats, id: String, power: InstancePowerCommand) -> Result {
  let power = match power {
    | InstancePowerCommand::On { duration } => InstancePowerControl { desired: DesiredInstancePowerState::On,
                                                                      until:   Utc::now()
                                                                               + Duration::milliseconds((duration * 1000.0) as i64), },
    | InstancePowerCommand::Off => InstancePowerControl { desired: DesiredInstancePowerState::Off,
                                                          until:   Utc::now(), },
    | InstancePowerCommand::Delete => {
      nats.instance_power_ctrl.delete(id.into()).await?;
      return Ok(());
    }
  };

  let revision = nats.instance_power_ctrl.put(id.into(), power).await?;

  println!("Power state updated with revision {revision}");

  Ok(())
}

async fn describe_instance(nats: Nats, id: String, include_spec: bool) -> Result {
  let spec = if include_spec {
    nats.instance_spec.get(id.clone().into()).await?
  } else {
    None
  };

  let connected_state = nats.instance_connection_state.get(instance_connection_state_key(&id)).await?;
  let power_state = nats.instance_power_state.get(instance_power_state_key(&id)).await?;
  let play_state = nats.instance_play_state.get(instance_play_state_key(&id)).await?;
  let power = nats.instance_power_ctrl.get(instance_power_control_key(&id)).await?;
  let play = nats.instance_play_ctrl.get(instance_play_control_key(&id)).await?;

  println!("Instance: {id}");
  if include_spec {
    println!(" * Spec: {}", serde_json::to_string_pretty(&spec).unwrap());
  }

  println!(" * Connected: {}", serde_json::to_string_pretty(&connected_state).unwrap());

  println!(" * Power: {}", serde_json::to_string_pretty(&power).unwrap());
  println!(" * Power State: {}", serde_json::to_string_pretty(&power_state).unwrap());

  println!(" * Play: {}", serde_json::to_string_pretty(&play).unwrap());
  println!(" * Play State: {}", serde_json::to_string_pretty(&play_state).unwrap());

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
async fn put_instance(nats: Nats, id: String, path: PathBuf, host: Option<String>, mocked: bool) -> Result {
  let mut spec = serde_yaml::from_reader::<_, InstanceSpec>(File::open(&path)?)?;
  if let Some(host) = host {
    spec.host = host;
  }
  if mocked {
    spec.driver = InstanceDriverConfig::Mock;
  }

  let revision = nats.instance_spec.put(id.into(), spec).await?;

  println!("Instance spec updated with revision {revision}");

  Ok(())
}

async fn user_command(nats: Nats, command: UserCommand) -> Result {
  match command {
    | UserCommand::Create { id, email } => create_user(nats, id, email).await,
    | UserCommand::Delete { id } => delete_user(nats, id).await,
    | UserCommand::List { format } => list_users(nats, format).await,
    | UserCommand::Describe { id } => describe_user(nats, id).await,
    | UserCommand::SetPassword { id } => set_password(nats, id).await,
  }
}

async fn create_user(nats: Nats, id: String, email: String) -> Result {
  let salt = create_salt();
  println!("Salt: {salt}");

  let password1 = rpassword::prompt_password("Password: ")?;
  let password2 = rpassword::prompt_password("Repeat: ")?;

  if &password1 != &password2 {
    return Err(anyhow!("Passwords do not match"));
  }

  let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2::Params::default());
  let salt = create_salt();
  let password = argon.hash_password(password1.as_bytes(), &salt)
                      .map_err(|e| anyhow!("Failed to hash password: {e}"))?;
  let password = format!("{password}");

  println!("Hashed: {password}");

  nats.user_spec.put(BucketKey::new(&id), UserSpec { id, email, password }).await?;

  Ok(())
}

async fn delete_user(nats: Nats, id: String) -> Result {
  todo!()
}

async fn list_users(nats: Nats, p1: OutputFormat) -> Result {
  todo!()
}

async fn describe_user(nats: Nats, id: String) -> Result {
  todo!()
}

async fn set_password(nats: Nats, id: String) -> Result {
  let password1 = rpassword::prompt_password("Password: ")?;
  let password2 = rpassword::prompt_password("Repeat: ")?;

  if &password1 != &password2 {
    return Err(anyhow!("Passwords do not match"));
  }

  todo!()
}

fn create_salt() -> SaltString {
  SaltString::generate(rand::thread_rng())
}
