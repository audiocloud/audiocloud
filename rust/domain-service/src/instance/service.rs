use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;
use futures::channel::mpsc;
use futures::future::join_all;
use futures::{SinkExt, StreamExt};
use tokio::time::Interval;
use tokio::{select, spawn, time};
use tokio_stream::StreamMap;
use tracing::{error, instrument, trace};

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::{SetInstanceParameter, SetInstanceParameterResponse};
use api::instance::spec::InstanceSpec;
use api::instance::{
  DesiredInstancePlayState, DesiredInstancePowerState, InstancePlayState, InstancePlayStateTransition, InstancePowerState,
};

use crate::nats::{EventStream, WatchStream};
use crate::request_tracker::RequestTracker;
use crate::service::Service;

use super::Result;

pub struct InstanceService {
  service:               Service,
  watch_specs:           WatchStream<String, InstanceSpec>,
  watch_power_control:   WatchStream<String, InstancePowerControl>,
  watch_play_control:    WatchStream<String, InstancePlayControl>,
  media_instance_events: StreamMap<String, EventStream<InstanceDriverEvent>>,
  tx_internal:           mpsc::Sender<InternalEvent>,
  rx_internal:           mpsc::Receiver<InternalEvent>,
  instances:             HashMap<String, Instance>,
  timer:                 Interval,
}

impl InstanceService {
  pub fn new(service: Service) -> Self {
    let watch_specs = service.watch_all_instance_specs();
    let watch_power_control = service.watch_all_instance_power_controls();
    let watch_play_control = service.watch_all_instance_play_controls();
    let media_instance_events = StreamMap::new();

    let instances = HashMap::new();

    let timer = time::interval(Duration::from_secs(1));

    let (tx_internal, rx_internal) = mpsc::channel(0xff);

    Self { service,
           timer,
           instances,
           watch_specs,
           watch_power_control,
           watch_play_control,
           media_instance_events,
           tx_internal,
           rx_internal }
  }

  pub async fn run(mut self) -> Result {
    loop {
      select! {
        Some((instance_id, maybe_instance_spec)) = self.watch_specs.next() => {
          self.update_instance_spec(instance_id, maybe_instance_spec).await;
        },
        Some((instance_id, maybe_instance_power_control)) = self.watch_power_control.next() => {
          self.update_instance_power_control(instance_id, maybe_instance_power_control).await;
        },
        Some((instance_id, maybe_instance_play_control)) = self.watch_play_control.next() => {
          self.update_instance_play_control(instance_id, maybe_instance_play_control).await;
        },
        Some((_, (instance_id, event))) = self.media_instance_events.next() => {
          self.media_instance_event(instance_id, event).await;
        },
        Some(internal_update) = self.rx_internal.next() => {
          self.internal_update(internal_update).await;
        }
        _ = self.timer.tick() => {
          self.timer_tick().await;
        }
      }
    }
  }

  async fn internal_update(&mut self, update: InternalEvent) {
    match update {
      | InternalEvent::InstancePowerSetSuccess { instance_id, desired } => {
        let entry = self.instances.entry(instance_id.clone()).or_default();

        if entry.power_set_success(&instance_id, desired) {
          entry.update(&instance_id, &self.service, &self.tx_internal).await;
        }
      }
    }
  }

  async fn update_instance_spec(&mut self, instance_id: String, maybe_instance_spec: Option<InstanceSpec>) {
    let entry = self.instances.entry(instance_id.clone()).or_default();
    entry.spec = maybe_instance_spec;

    match entry.spec.as_ref() {
      | None => {
        self.media_instance_events.remove(&instance_id);
      }
      | Some(spec) => {
        if spec.media.is_some() && !self.media_instance_events.contains_key(&instance_id) {
          self.media_instance_events
              .insert(instance_id.clone(), self.service.subscribe_to_instance_events(&instance_id));
        }

        if spec.power.is_some() {
          if let Ok(actual_power) = self.service.get_instance_power_state(&instance_id).await {
            entry.persisted_power_state = actual_power;
          }
        }

        if spec.media.is_some() {
          if let Ok(actual_play) = self.service.get_instance_play_state(&instance_id).await {
            entry.persisted_play_state = actual_play;
          }
        }
      }
    }

    entry.update(&instance_id, &self.service, &self.tx_internal).await;
  }

  #[instrument(skip(self))]
  async fn update_instance_power_control(&mut self, instance_id: String, maybe_instance_power_control: Option<InstancePowerControl>) {
    let entry = self.instances.entry(instance_id.clone()).or_default();
    entry.power_control = maybe_instance_power_control;

    trace!("updating power control");

    if let Some(power_control) = &entry.power_control {
      if entry.power_request.set_desired(power_control.desired) {
        entry.update(&instance_id, &self.service, &self.tx_internal).await;
      }
    }
  }

  async fn update_instance_play_control(&mut self, instance_id: String, maybe_instance_play_control: Option<InstancePlayControl>) {
    let entry = self.instances.entry(instance_id.clone()).or_default();
    entry.play_control = maybe_instance_play_control;

    if let Some(play_control) = &entry.play_control {
      if entry.play_request.set_desired(play_control.desired) {
        entry.update(&instance_id, &self.service, &self.tx_internal).await;
      }
    }
  }

  async fn timer_tick(&mut self) {
    join_all(self.instances
                 .iter_mut()
                 .map(|(id, instance)| instance.update(id, &self.service, &self.tx_internal))).await;
  }

  async fn media_instance_event(&mut self, instance_id: String, event: InstanceDriverEvent) {
    if let InstanceDriverEvent::Report(report) = event {
      let entry = self.instances.entry(instance_id.clone()).or_default();
      let mut needs_update = false;
      if let Some(media_spec) = entry.spec.as_ref().and_then(|spec| spec.media.as_ref()) {
        if &media_spec.position_report == &report.report_id {
          entry.position_ms = (report.value * 1000.0) as u64;
          needs_update = true;
        }

        for trigger in &media_spec.play_state_triggers {
          if trigger.is_triggered(&report.report_id, report.value) {
            if entry.play_request.set_actual(match trigger.then {
                                   | InstancePlayStateTransition::SetRewinding => InstancePlayState::Rewinding,
                                   | InstancePlayStateTransition::SetIdle => InstancePlayState::Idle,
                                   | InstancePlayStateTransition::SetBusy => InstancePlayState::Busy,
                                   | InstancePlayStateTransition::SetPlaying => match entry.play_request.get_desired() {
                                     | DesiredInstancePlayState::Stop => UNKNOWN_PLAYING_STATE,
                                     | DesiredInstancePlayState::Play { play_id, duration } =>
                                       InstancePlayState::Playing { play_id, duration },
                                   },
                                 })
            {
              needs_update = true;
            }
          }
        }
      }

      if needs_update {
        entry.update(&instance_id, &self.service, &self.tx_internal).await;
      }
    }
  }
}

#[derive(Default)]
struct Instance {
  spec:                  Option<InstanceSpec>,
  power_control:         Option<InstancePowerControl>,
  play_control:          Option<InstancePlayControl>,
  persisted_power_state: Option<InstancePowerState>,
  persisted_play_state:  Option<InstancePlayState>,
  power_request:         RequestTracker<DesiredInstancePowerState, InstancePowerState>,
  play_request:          RequestTracker<DesiredInstancePlayState, InstancePlayState>,
  position_ms:           u64,
}

impl Instance {
  pub async fn update(&mut self, id: &str, service: &Service, tx_internal: &mpsc::Sender<InternalEvent>) -> bool {
    let power_is_fulfilled = self.update_power(id, service, tx_internal).await;
    if power_is_fulfilled {
      self.update_play(id, service).await
    } else {
      false
    }
  }

  async fn update_power(&mut self, id: &str, service: &Service, tx_internal: &mpsc::Sender<InternalEvent>) -> bool {
    let mut desired = self.power_request.get_desired();

    let idle_ms = self.idle_ms();

    if let Some(control) = self.power_control.as_ref() {
      if Utc::now() + chrono::Duration::milliseconds(idle_ms as i64) > control.until {
        desired = DesiredInstancePowerState::Off;
        self.power_request.set_desired(desired);
      }
    }

    let elapsed = self.power_request.actual_elapsed_ms();
    match self.power_request.get_actual() {
      | InstancePowerState::CoolingDown =>
        if elapsed > self.cooling_down_ms() {
          self.power_request.set_actual(InstancePowerState::Off);
        },
      | InstancePowerState::WarmingUp =>
        if elapsed > self.warming_up_ms() {
          self.power_request.set_actual(InstancePowerState::On);
        },
      | _ => {}
    }

    if self.power_request.should_request_update() {
      if let Some(power_spec) = self.spec.as_ref().and_then(|spec| spec.power.as_ref()) {
        let command = power_spec.get_command(desired);
        self.power_request.update_requested_now();

        let command = vec![SetInstanceParameter { parameter: command.parameter.clone(),
                                                  channel:   command.channel,
                                                  value:     command.value, }];

        let desired = self.power_request.get_desired();
        let instance_id = id.to_owned();

        let mut tx_internal = tx_internal.clone();
        let service = service.clone();

        spawn(async move {
          match service.set_instance_parameters(&instance_id, command).await {
            | Ok(SetInstanceParameterResponse::Success) => {
              let _ = tx_internal.send(InternalEvent::InstancePowerSetSuccess { instance_id, desired })
                                 .await;
            }
            | Ok(other) => {
              error!(instance_id, "Failed to set power state for instance: {other}");
            }
            | Err(err) => {
              error!(instance_id, ?err, "Failed to set power state for instance: {err}");
            }
          }
        });
      }

      self.power_request.update_requested_now();
    }

    let actual = self.power_request.get_actual();
    if self.persisted_power_state.as_ref() != Some(&actual) {
      if let Ok(_) = service.set_instance_power_state(id, actual).await {
        let _ = service.publish_instance_driver_event(id, InstanceDriverEvent::PowerStateChanged { state: actual })
                       .await;

        self.persisted_power_state = Some(actual);
      }
    }

    self.power_request.is_fulfilled()
  }

  fn power_set_success(&mut self, _instance_id: &str, desired: DesiredInstancePowerState) -> bool {
    self.power_request.set_actual(match desired {
                        | DesiredInstancePowerState::Off => InstancePowerState::CoolingDown,
                        | DesiredInstancePowerState::On => InstancePowerState::WarmingUp,
                      })
  }

  async fn update_play(&mut self, id: &str, service: &Service) -> bool {
    let mut desired = self.play_request.get_desired();

    if let Some(control) = self.play_control.as_ref() {
      if Utc::now() > control.until {
        desired = DesiredInstancePlayState::Stop;
        self.play_request.set_desired(desired);
      }
    }

    if self.play_request.should_request_update() {
      if let Some(media_spec) = self.spec.as_ref().and_then(|spec| spec.media.as_ref()) {
        let remaining = media_spec.duration_ms - self.position_ms;
        let command = media_spec.get_command(desired, (remaining as f64) / 1000.0);
        self.play_request.update_requested_now();

        let command = vec![SetInstanceParameter { parameter: command.parameter.clone(),
                                                  channel:   command.channel,
                                                  value:     command.value, }];

        let instance_id = id.to_owned();
        let service = service.clone();
        let id = id.to_owned();

        spawn(async move {
          match service.set_instance_parameters(&id, command).await {
            | Ok(SetInstanceParameterResponse::Success) => {
              // the instance will send a report that indicates changed play state, so we don't
              // have to report back here
            }
            | Ok(other) => {
              error!(instance_id, "Failed to set play state for instance: {other}");
            }
            | Err(err) => {
              error!(instance_id, ?err, "Failed to set play state for instance: {err}");
            }
          }
        });

        self.play_request.update_requested_now();
      }
    }

    let actual = self.play_request.get_actual();
    if self.persisted_play_state.as_ref() != Some(&actual) {
      if let Ok(_) = service.set_instance_play_state(id, actual.clone()).await {
        let _ = service.publish_instance_driver_event(id, InstanceDriverEvent::PlayStateChanged { state: actual })
                       .await;

        self.persisted_play_state = Some(actual);
      }
    }

    self.play_request.is_fulfilled()
  }

  fn idle_ms(&mut self) -> u64 {
    self.spec
        .as_ref()
        .and_then(|spec| spec.power.as_ref())
        .map(|power_spec| power_spec.idle_ms)
        .unwrap_or(5 * 60 * 1000) // default is 5 minutes
  }

  fn cooling_down_ms(&self) -> u64 {
    self.spec
        .as_ref()
        .and_then(|spec| spec.power.as_ref())
        .map(|power_spec| power_spec.cool_down_ms)
        .unwrap_or(5 * 1000) // default is 5 seconds
  }

  fn warming_up_ms(&self) -> u64 {
    self.spec
        .as_ref()
        .and_then(|spec| spec.power.as_ref())
        .map(|power_spec| power_spec.warm_up_ms)
        .unwrap_or(15 * 1000) // default is 15 seconds
  }
}

enum InternalEvent {
  InstancePowerSetSuccess {
    instance_id: String,
    desired:     DesiredInstancePowerState,
  },
}

const UNKNOWN_PLAYING_STATE: InstancePlayState = InstancePlayState::Playing { play_id:  u64::MAX,
                                                                              duration: 0.0, };
