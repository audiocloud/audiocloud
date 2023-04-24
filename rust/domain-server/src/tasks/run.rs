use std::collections::{HashMap, HashSet};
use std::time::Duration;

use chrono::Utc;
use futures::channel::mpsc;
use futures::StreamExt;
use tokio::select;
use tokio::time::Interval;
use tokio_stream::StreamMap;
use tracing::{debug, instrument};

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::spec::{instance_spec_key, InstanceSpec};
use api::instance::state::{instance_connection_state_key, instance_play_state_key, instance_power_state_key};
use api::instance::{DesiredInstancePlayState, DesiredInstancePowerState, InstanceConnectionState, InstancePlayState, InstancePowerState};
use api::media::spec::MediaId;
use api::media::state::{media_download_state_key, MediaDownloadState};
use api::task::buckets::{task_control_key, task_spec_key};
use api::task::spec::TaskSpec;
use api::task::DesiredTaskPlayState;
use api::BucketKey;

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
  player: Option<()>,
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
          self.instance_spec_updated(instance_id, maybe_instance_spec_update).await;
        },
        Some((instance_id, (_, maybe_instance_power_state_update))) = self.watch_instance_power_states.next(), if !self.watch_instance_power_states.is_empty() => {
          self.instance_power_state_updated(instance_id, maybe_instance_power_state_update).await;
        },
        Some((instance_id, (_, maybe_instance_connection_state_update))) = self.watch_instance_connection_state.next(), if !self.watch_instance_connection_state.is_empty() => {
          self.instance_connection_state_updated(instance_id, maybe_instance_connection_state_update).await;
        },
        Some((instance_id, (_, maybe_instance_play_state_update))) = self.watch_instance_play_states.next(), if !self.watch_instance_play_states.is_empty() => {
          self.instance_play_state_updated(instance_id, maybe_instance_play_state_update).await;
        },
        Some((media_id, (_, maybe_download_state_update))) = self.watch_download_states.next(), if !self.watch_download_states.is_empty() => {
          self.download_state_updated(media_id, maybe_download_state_update).await;
        },
        Some((_, maybe_new_spec)) = self.watch_spec.next() => {
          if let Some(new_spec) = maybe_new_spec {
            self.set_spec(new_spec).await;
          } else {
            // we are done, go..
            break;
          }
        },
        Some((_, maybe_new_control)) = self.watch_control.next() => {
          self.set_desired_play_state(maybe_new_control).await;
        },
        Some(external_task) = self.rx_external.next() => {
          self.external_task_completed(external_task);
        },
        _ = self.timer.tick() => {
          self.timer_tick().await;
        }
      }
    }

    debug!("Task finished, cleaning up");

    self.cleanup();

    Ok(())
  }

  async fn set_spec(&mut self, new_spec: TaskSpec) {
    if &self.spec == &new_spec {
      return;
    }

    self.resubscribe_instances();
    self.resubscribe_media();

    for instance in self.instances.values_mut() {
      instance.play_control = None;
      instance.power_control = None;
    }

    self.update_instance_power_play_state().await;
  }

  async fn set_desired_play_state(&mut self, new_control: Option<DesiredTaskPlayState>) {
    self.desired_play_state = new_control.unwrap_or_default();
    self.update_instance_power_play_state().await;
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

  async fn instance_spec_updated(&mut self, instance_id: String, spec: Option<InstanceSpec>) {
    self.instances.entry(instance_id).or_default().spec = spec;

    self.update_instance_power_play_state().await;
  }

  async fn instance_power_state_updated(&mut self, instance_id: String, spec: Option<InstancePowerState>) {
    self.instances.entry(instance_id).or_default().power_state = spec;

    self.update_instance_power_play_state().await;
  }

  async fn instance_connection_state_updated(&mut self, instance_id: String, spec: Option<InstanceConnectionState>) {
    self.instances.entry(instance_id).or_default().connection_state = spec;

    self.update_instance_power_play_state().await;
  }

  async fn instance_play_state_updated(&mut self, instance_id: String, spec: Option<InstancePlayState>) {
    self.instances.entry(instance_id).or_default().play_state = spec;

    self.update_instance_power_play_state().await;
  }

  async fn download_state_updated(&mut self, media_id: MediaId, state: Option<MediaDownloadState>) {
    self.media.entry(media_id).or_default().state = state;

    self.update_instance_power_play_state().await;
  }

  fn external_task_completed(&mut self, external: ExternalTask) {
    match external {}
  }

  async fn propagate_instance_power_control(&mut self) {
    let desired = DesiredInstancePowerState::On;

    for (instance_id, instance) in &mut self.instances {
      let Some(spec) = instance.spec.as_ref() else { continue };
      let Some(_) = spec.power.as_ref() else { continue }; // only interested in instances with media spec

      if instance.power_control
                 .as_ref()
                 .map(|power_control| &power_control.desired != &desired)
                 .unwrap_or(true)
      {
        let control = InstancePowerControl { desired: desired.clone(),
                                             until:   self.spec.to, };

        let _ = self.nats
                    .instance_power_ctrl
                    .put(BucketKey::new(instance_id), control.clone())
                    .await;

        instance.power_control = Some(control);
      }
    }
  }

  async fn propagate_instance_play_control(&mut self) {
    let desired = DesiredInstancePlayState::from(self.desired_play_state.clone());

    for (instance_id, instance) in &mut self.instances {
      let Some(spec) = instance.spec.as_ref() else { continue };
      let Some(_) = spec.media.as_ref() else { continue }; // only interested in instances with media spec

      if instance.play_control
                 .as_ref()
                 .map(|play_control| &play_control.desired != &desired)
                 .unwrap_or(true)
      {
        let control = InstancePlayControl { desired: desired.clone(),
                                            until:   self.spec.to, };

        let _ = self.nats.instance_play_ctrl.put(BucketKey::new(instance_id), control.clone()).await;

        instance.play_control = Some(control);
      }
    }
  }

  async fn update_instance_power_play_state(&mut self) {
    let should_play = self.desired_play_state.is_playing();
    let is_playing = self.player.is_some();

    let (missing_instances, unready_instances) = self.get_missing_or_unready_instances();
    let (missing_media, unready_media) = self.get_missing_or_unready_media();
    let is_ready = missing_instances.is_empty() && unready_instances.is_empty() && missing_media.is_empty() && unready_media.is_empty();

    // we are OK, we can propagate
    if missing_instances.is_empty() {
      self.propagate_instance_power_control().await;
      self.propagate_instance_play_control().await;
    }

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

      if spec.power.is_some() {
        match (instance.power_control.as_ref(), instance.power_state.as_ref()) {
          | (Some(power_control), Some(power_state)) if power_state == &power_control.desired => {}
          | (None, None) => {}
          | _ => {
            unready_instances.insert(instance_id.clone());
            continue;
          }
        }
      }

      if spec.media.is_some() {
        match (instance.play_control.as_ref(), instance.play_state.as_ref()) {
          | (Some(play_control), Some(play_state)) if play_state == &play_control.desired => {}
          | (None, None) => {}
          | _ => {
            unready_instances.insert(instance_id.clone());
            continue;
          }
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

  async fn timer_tick(&mut self) {
    self.update_instance_power_play_state().await;
  }
}

#[derive(Default)]
struct TaskInstance {
  spec:             Option<InstanceSpec>,
  connection_state: Option<InstanceConnectionState>,
  power_state:      Option<InstancePowerState>,
  play_state:       Option<InstancePlayState>,
  power_control:    Option<InstancePowerControl>,
  play_control:     Option<InstancePlayControl>,
}

#[derive(Default)]
struct TaskMedia {
  state: Option<MediaDownloadState>,
}
