use std::collections::{HashMap, HashSet};
use std::time::Duration;

use chrono::Utc;
use futures::StreamExt;
use tokio::select;
use tokio::sync::mpsc;
use tokio::time::Interval;
use tokio_stream::StreamMap;
use tracing::{debug, instrument};

use api::instance::spec::{instance_spec_key, InstanceSpec};
use api::instance::state::{instance_connection_state_key, instance_play_state_key, instance_power_state_key};
use api::instance::{InstanceConnectionState, InstancePlayState, InstancePowerState};
use api::media::spec::MediaId;
use api::media::state::{media_download_state_key, MediaDownloadState};
use api::task::buckets::{task_control_key, task_spec_key};
use api::task::spec::TaskSpec;
use api::task::DesiredTaskPlayState;
use async_audio_engine::GraphPlayer;

use crate::nats::{Nats, WatchStream, WatchStreamMap};
use crate::tasks::Result;

pub struct RunDomainTask {
  id: String,
  spec: TaskSpec,
  timer: Interval,
  tx_external: mpsc::Sender<ExternalTask>,
  rx_external: mpsc::Receiver<ExternalTask>,
  watch_spec: WatchStream<String, TaskSpec>,
  watch_control: WatchStream<String, DesiredTaskPlayState>,
  watch_instance_specs: WatchStreamMap<String, InstanceSpec>,
  watch_instance_connection_state: WatchStreamMap<String, InstanceConnectionState>,
  watch_instance_power_states: WatchStreamMap<String, InstancePowerState>,
  watch_instance_play_states: WatchStreamMap<String, InstancePlayState>,
  watch_download_states: WatchStreamMap<MediaId, MediaDownloadState>,
  instances: HashMap<String, TaskInstance>,
  media: HashMap<MediaId, TaskMedia>,
  desired_play_state: DesiredTaskPlayState,
  player: Option<GraphPlayer>,
  nats: Nats,
}

enum ExternalTask {}

impl RunDomainTask {
  pub fn new(id: String, spec: TaskSpec, nats: Nats) -> RunDomainTask {
    let watch_spec = nats.task_spec.watch(task_spec_key(&id));
    let watch_control = nats.task_ctrl.watch(task_control_key(&id));
    let watch_instance_specs = StreamMap::new();
    let watch_instance_power_states = StreamMap::new();
    let watch_instance_play_states = StreamMap::new();
    let watch_instance_connection_state = StreamMap::new();
    let watch_download_states = StreamMap::new();

    let instances = HashMap::new();
    let media = HashMap::new();

    let player = None;
    let desired_play_state = DesiredTaskPlayState::Idle;

    let (tx_external, rx_external) = mpsc::channel(0xff);
    let timer = tokio::time::interval(Duration::from_secs(1));

    let mut rv = Self { id,
                        spec,
                        timer,
                        player,
                        media,
                        nats,
                        instances,
                        watch_spec,
                        watch_control,
                        tx_external,
                        rx_external,
                        desired_play_state,
                        watch_instance_specs,
                        watch_instance_connection_state,
                        watch_instance_power_states,
                        watch_instance_play_states,
                        watch_download_states };

    rv.resubscribe_media();
    rv.resubscribe_instances();

    rv
  }

  #[instrument(err, skip(self), fields(id = self.id))]
  pub async fn run(mut self) -> Result {
    while Utc::now() < self.spec.to {
      select! {
        Some((instance_id, (_, maybe_instance_spec_update))) = self.watch_instance_specs.next(), if !self.watch_instance_specs.is_empty() => {
          self.instance_spec_updated(instance_id, maybe_instance_spec_update);
        },
        Some((instance_id, (_, maybe_instance_power_state_update))) = self.watch_instance_power_states.next(), if !self.watch_instance_power_states.is_empty() => {
          self.instance_power_state_updated(instance_id, maybe_instance_power_state_update);
        },
        Some((instance_id, (_, maybe_instance_connection_state_update))) = self.watch_instance_connection_state.next(), if !self.watch_instance_connection_state.is_empty() => {
          self.instance_connection_state_updated(instance_id, maybe_instance_connection_state_update);
        },
        Some((instance_id, (_, maybe_instance_play_state_update))) = self.watch_instance_play_states.next(), if !self.watch_instance_play_states.is_empty() => {
          self.instance_play_state_updated(instance_id, maybe_instance_play_state_update);
        },
        Some((media_id, (_, maybe_download_state_update))) = self.watch_download_states.next(), if !self.watch_download_states.is_empty() => {
          self.download_state_updated(media_id, maybe_download_state_update);
        },
        Some((_, maybe_new_spec)) = self.watch_spec.next() => {
          if let Some(new_spec) = maybe_new_spec {
            self.set_spec(new_spec);
          } else {
            // we are done, go..
            break;
          }
        },
        Some((_, maybe_new_control)) = self.watch_control.next() => {
          self.set_desired_play_state(maybe_new_control);
        },
        Some(external_task) = self.rx_external.recv() => {
          self.external_task_completed(external_task);
        },
        _ = self.timer.tick() => {
          self.timer_tick();
        }
      }
    }

    debug!("Task finished, cleaning up");

    self.cleanup();

    Ok(())
  }

  fn set_spec(&mut self, new_spec: TaskSpec) {
    if &self.spec == &new_spec {
      return;
    }

    self.resubscribe_instances();
    self.resubscribe_media();
  }

  fn set_desired_play_state(&mut self, new_control: Option<DesiredTaskPlayState>) {
    let new_control = new_control.unwrap_or_default();
  }

  fn resubscribe_instances(&mut self) {
    let to_remove = self.watch_instance_play_states
                        .keys()
                        .filter(|key| !self.spec.instances.contains_key(*key))
                        .cloned()
                        .collect::<HashSet<_>>();

    for instance_id in &to_remove {
      self.watch_instance_play_states.remove(instance_id);
      self.watch_instance_specs.remove(instance_id);
      self.instances.remove(instance_id);
    }

    for instance_id in self.spec.instances.values() {
      self.watch_instance_specs
          .insert(instance_id.clone(), self.nats.instance_spec.watch(instance_spec_key(&instance_id)));

      self.watch_instance_power_states.insert(instance_id.clone(),
                                              self.nats.instance_power_state.watch(instance_power_state_key(&instance_id)));

      self.watch_instance_play_states.insert(instance_id.clone(),
                                             self.nats.instance_play_state.watch(instance_play_state_key(&instance_id)));

      self.watch_instance_connection_state.insert(instance_id.clone(),
                                                  self.nats
                                                      .instance_connection_state
                                                      .watch(instance_connection_state_key(&instance_id)));
    }
  }

  fn resubscribe_media(&mut self) {
    let to_remove = self.watch_download_states
                        .keys()
                        .filter(|key| !self.spec.graph_spec.sources.values().any(|source| &source.media_id == *key))
                        .cloned()
                        .collect::<HashSet<_>>();

    for media_id in &to_remove {
      self.watch_download_states.remove(media_id);
      self.media.remove(media_id);
    }

    for source in self.spec.graph_spec.sources.values() {
      if self.watch_download_states.contains_key(&source.media_id) {
        continue;
      }

      self.watch_download_states.insert(source.media_id.clone(),
                                        self.nats.media_download_state.watch(media_download_state_key(&source.media_id)));
    }
  }

  fn instance_spec_updated(&mut self, instance_id: String, spec: Option<InstanceSpec>) {
    self.instances.entry(instance_id).or_default().spec = spec;

    self.update_play_state();
  }

  fn instance_power_state_updated(&mut self, instance_id: String, spec: Option<InstancePowerState>) {
    self.instances.entry(instance_id).or_default().power_state = spec;

    self.update_play_state();
  }

  fn instance_connection_state_updated(&mut self, instance_id: String, spec: Option<InstanceConnectionState>) {
    self.instances.entry(instance_id).or_default().connection_state = spec;

    self.update_play_state();
  }

  fn instance_play_state_updated(&mut self, instance_id: String, spec: Option<InstancePlayState>) {
    self.instances.entry(instance_id).or_default().play_state = spec;

    self.update_play_state();
  }

  fn download_state_updated(&mut self, media_id: MediaId, state: Option<MediaDownloadState>) {
    self.media.entry(media_id).or_default().state = state;

    self.update_play_state();
  }

  fn external_task_completed(&mut self, external: ExternalTask) {
    match external {}
  }

  fn update_play_state(&mut self) {
    let should_play = self.desired_play_state.is_playing();
    let is_playing = self.player.is_some();

    let (missing_instances, unready_instances) = self.get_missing_or_unready_instances();
    let (missing_media, unready_media) = self.get_missing_or_unready_media();
    let is_ready = missing_instances.is_empty() && unready_instances.is_empty() && missing_media.is_empty() && unready_media.is_empty();

    if should_play && is_ready {
      if !is_playing {
        self.start_player();
      }
    } else {
      if is_playing {
        self.stop_player();
      }
    }
  }

  fn start_player(&mut self) {}

  fn stop_player(&mut self) {}

  fn get_missing_or_unready_instances(&self) -> (HashSet<String>, HashSet<String>) {
    let mut missing_instances = HashSet::new();
    let mut unready_instances = HashSet::new();

    for instance_id in self.spec.instances.values() {
      let Some(instance) = self.instances.get(instance_id) else {
        missing_instances.insert(instance_id.clone());
        continue;
      };

      let Some(spec) = instance.spec.as_ref() else {
        missing_instances.insert(instance_id.clone());
        continue;
      };

      if instance.connection_state
                 .as_ref()
                 .map(|connection| connection.is_connected())
                 .unwrap_or(false)
      {
        unready_instances.insert(instance_id.clone());
        continue;
      }

      if spec.media.is_some() {
        let power_state = match instance.power_state.as_ref() {
          | None =>
            if spec.power.is_some() {
              unready_instances.insert(instance_id.clone());
              continue;
            } else {
              &InstancePowerState::On
            },
          | Some(state) => state,
        };

        if !power_state.is_on() {
          unready_instances.insert(instance_id.clone());
          continue;
        }

        let play_state = match instance.play_state.as_ref() {
          | None => {
            if spec.media.is_some() {
              unready_instances.insert(instance_id.clone());
            }
            continue;
          }
          | Some(state) => state,
        };

        match &self.desired_play_state {
          | DesiredTaskPlayState::Idle =>
            if play_state != &InstancePlayState::Idle {
              unready_instances.insert(instance_id.clone());
            },
          | DesiredTaskPlayState::Play { play_id, .. } => match play_state {
            | InstancePlayState::Playing { play_id: instance_play_id, .. } if play_id == instance_play_id => {}
            | _ => {
              unready_instances.insert(instance_id.clone());
            }
          },
        }
      }
    }

    (missing_instances, unready_instances)
  }

  fn get_missing_or_unready_media(&self) -> (HashSet<MediaId>, HashSet<MediaId>) {
    let mut missing_media = HashSet::new();
    let mut downloading_media = HashSet::new();

    for source in self.spec.graph_spec.sources.values() {
      let media_id = &source.media_id;
      let media = match self.media.get(media_id) {
        | None => {
          missing_media.insert(media_id.to_owned());
          continue;
        }
        | Some(media) => media,
      };

      match media.state.as_ref() {
        | None => {
          missing_media.insert(media_id.to_owned());
        }
        | Some(state) =>
          if state.done.is_none() {
            downloading_media.insert(media_id.to_owned());
          },
      }
    }

    (missing_media, downloading_media)
  }

  fn cleanup(&mut self) {
    let is_playing = self.player.is_some();
    if is_playing {
      self.stop_player();
    }
  }

  fn timer_tick(&mut self) {}
}

#[derive(Default)]
struct TaskInstance {
  spec:             Option<InstanceSpec>,
  connection_state: Option<InstanceConnectionState>,
  power_state:      Option<InstancePowerState>,
  play_state:       Option<InstancePlayState>,
}

#[derive(Default)]
struct TaskMedia {
  state: Option<MediaDownloadState>,
}
