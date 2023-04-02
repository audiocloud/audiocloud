use std::collections::HashSet;
use std::mem;

use anyhow::anyhow;
use async_nats::jetstream::{kv, Context};
use chrono::Utc;
use futures::StreamExt;
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::StreamMap;

use api::driver::InstanceDriverEvent;
use api::graph::{AudioGraphModification, GraphPlaybackEvent};
use api::{driver, media};
use async_audio_engine::GraphPlayer;

use crate::tasks::{Result, TaskSpec};

pub enum Command {
  Terminate,
  SetSpec(TaskSpec),
  ModifyGraph(Vec<AudioGraphModification>),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum WatchKey {
  InstanceSpec(String),
  InstanceState(String),
  MediaState(String),
}

type WatchStream<'a> = StreamMap<WatchKey, kv::Watch<'a>>;
type NatsResult<T> = std::result::Result<T, async_nats::Error>;

struct TaskService<'a> {
  jetstream:             Context,
  task_id:               String,
  spec:                  TaskSpec,
  rx_cmd:                mpsc::Receiver<Command>,
  enabled:               bool,
  instance_state_bucket: kv::Store,
  instance_spec_bucket:  kv::Store,
  media_state_bucket:    kv::Store,

  // changes to instance state and events like reports
  events_monitor: WatchStream<'a>,

  // pending instance events to send out as task event
  instance_events: Vec<InstanceDriverEvent>,

  // pending player events to send out as task event
  player_events: Vec<GraphPlaybackEvent>,

  // the current player of the graph, if any
  current_player: Option<GraphPlayer>,
}

impl<'a> TaskService<'a> {
  pub async fn new(jetstream: Context, task_id: String, spec: TaskSpec, rx_cmd: mpsc::Receiver<Command>) -> Result<TaskService<'a>> {
    let instance_state_bucket = jetstream.get_key_value(driver::buckets::INSTANCE_STATE)
                                         .await
                                         .map_err(|err| anyhow!("Error subscribing to instance state k/v bucket: {err}"))?;

    let instance_spec_bucket = jetstream.get_key_value(driver::buckets::INSTANCE_SPEC)
                                        .await
                                        .map_err(|err| anyhow!("Error subscribing to instance spec k/v bucket: {err}"))?;

    let media_status_bucket = jetstream.get_key_value(media::buckets::MEDIA_STATE)
                                       .await
                                       .map_err(|err| anyhow!("Error subscribing to media status k/v bucket: {err}"))?;

    let mut task_service = Self { jetstream:             { jetstream },
                                  task_id:               { task_id },
                                  spec:                  { spec },
                                  rx_cmd:                { rx_cmd },
                                  enabled:               { true },
                                  events_monitor:        { StreamMap::new() },
                                  instance_events:       { vec![] },
                                  player_events:         { vec![] },
                                  current_player:        { None },
                                  instance_state_bucket: { instance_state_bucket },
                                  instance_spec_bucket:  { instance_spec_bucket },
                                  media_state_bucket:    { media_status_bucket }, };

    task_service.initialize().await?;
    task_service.resubscribe_instance_watches().await?;

    Ok(task_service)
  }

  async fn initialize(&mut self) -> Result {
    Ok(())
  }

  async fn run_task(mut self) -> Result {
    use WatchKey::*;

    while Utc::now() < self.spec.to && self.enabled {
      select! {
        Some((key, event)) = self.events_monitor.next(), if !self.events_monitor.is_empty() => {
          match key {
              | InstanceSpec(instance_id) => self.on_instance_spec_changed(instance_id, event).await?,
              | InstanceState(instance_id) => self.on_instance_state_changed(instance_id, event).await?,
              | MediaState(media_url) => self.on_media_state_changed(media_url, event).await?,
          }
        },
        Some(cmd) = self.rx_cmd.recv() => {
          self.execute_command(cmd).await?;
        },
        else => break
      }
    }

    Ok(())
  }

  async fn on_instance_spec_changed(&mut self, instance_id: String, event: NatsResult<kv::Entry>) -> Result {
    Ok(())
  }

  async fn on_instance_state_changed(&mut self, instance_id: String, event: NatsResult<kv::Entry>) -> Result {
    Ok(())
  }

  async fn on_media_state_changed(&mut self, media_url: String, event: NatsResult<kv::Entry>) -> Result {
    Ok(())
  }

  async fn execute_command(&mut self, cmd: Command) -> Result {
    match cmd {
      | Command::Terminate => {
        self.enabled = false;
      }
      | Command::SetSpec(new_spec) => {
        self.spec = new_spec;

        self.resubscribe_instance_watches().await?;
      }
      | Command::ModifyGraph(modifications) => {}
    }

    Ok(())
  }

  async fn resubscribe_instance_watches(&mut self) -> Result {
    use WatchKey::*;

    let to_remove = self.events_monitor
                        .keys()
                        .filter(|key| match key {
                          | InstanceSpec(id) | InstanceState(id) => !self.spec.instances.contains_key(id),
                          | MediaState(url) => !self.spec.graph_spec.sources.values().any(|spec| &spec.source_url == url),
                        })
                        .cloned()
                        .collect::<HashSet<_>>();

    for key in to_remove {
      self.events_monitor.remove(&key);
    }

    for (id, instance_id) in &self.spec.instances {
      let key = InstanceState(id.to_owned());

      if !self.events_monitor.contains_key(&key) {
        let stream = self.instance_state_bucket
                         .watch(instance_id.clone())
                         .await
                         .map_err(|err| anyhow!("Failed to watch instance state: {instance_id}: {err}"))?;

        // fuck off rust borrow checker - the streams we are getting here will get dropped when stream map is dropped
        self.events_monitor.insert(key, unsafe { mem::transmute(stream) });
      }

      let key = InstanceSpec(id.to_owned());
      if !self.events_monitor.contains_key(&key) {
        let stream = self.instance_spec_bucket
                         .watch(instance_id.clone())
                         .await
                         .map_err(|err| anyhow!("Failed to watch instance state: {instance_id}: {err}"))?;

        // fuck off rust borrow checker - the streams we are getting here will get dropped when stream map is dropped
        self.events_monitor.insert(key, unsafe { mem::transmute(stream) });
      }
    }

    for source in self.spec.graph_spec.sources.values() {
      let source_url = &source.source_url;
      let key = MediaState(source_url.clone());

      if !self.events_monitor.contains_key(&key) {
        let stream = self.instance_spec_bucket
                         .watch(source_url.clone())
                         .await
                         .map_err(|err| anyhow!("Failed to watch media state: {source_url}: {err}"))?;

        // fuck off rust borrow checker - the streams we are getting here will get dropped when stream map is dropped
        self.events_monitor.insert(key, unsafe { mem::transmute(stream) });
      }
    }

    Ok(())
  }
}
