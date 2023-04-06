use std::collections::HashMap;
use std::time::Duration;

use tokio::time::Interval;
use tokio::{select, time};
use tokio_stream::{StreamExt, StreamMap};
use tracing::error;

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::{set_instance_parameters_request, SetInstanceParameterRequest, SetInstanceParameterResponse};
use api::instance::spec::InstanceSpec;
use api::instance::{DesiredInstancePlayState, DesiredInstancePowerState, InstancePlayState, InstancePowerState};

use crate::nats::{EventStream, Nats, WatchStream};
use crate::request_tracker::RequestTracker;

use super::Result;

pub struct InstanceService {
  nats:                    Nats,
  watch_specs:             WatchStream<InstanceSpec>,
  watch_power_control:     WatchStream<InstancePowerControl>,
  watch_play_control:      WatchStream<InstancePlayControl>,
  power_controller_events: StreamMap<String, EventStream<InstanceDriverEvent>>,
  media_instance_events:   StreamMap<String, EventStream<InstanceDriverEvent>>,
  instances:               HashMap<String, Instance>,
  timer:                   Interval,
}

impl InstanceService {
  pub fn new(nats: Nats) -> Self {
    let watch_specs = nats.instance_spec.watch_all();
    let watch_power_control = nats.instance_power_ctrl.watch_all();
    let watch_play_control = nats.instance_play_ctrl.watch_all();
    let power_controller_events = StreamMap::new();
    let media_instance_events = StreamMap::new();

    let instances = HashMap::new();

    let timer = time::interval(Duration::from_secs(1));

    Self { nats,
           timer,
           instances,
           watch_specs,
           watch_power_control,
           watch_play_control,
           power_controller_events,
           media_instance_events }
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
        _ = self.timer.tick() => {
          self.timer_tick().await;
        }
      }
    }
  }

  async fn update_instance_spec(&mut self, instance_id: String, maybe_instance_spec: Option<InstanceSpec>) {
    let entry = self.instances.entry(instance_id.clone()).or_default();
    entry.spec = maybe_instance_spec;

    entry.update(&instance_id, &self.nats).await;
  }

  async fn update_instance_power_control(&mut self, instance_id: String, maybe_instance_power_control: Option<InstancePowerControl>) {
    let entry = self.instances.entry(instance_id.clone()).or_default();
    entry.power_control = maybe_instance_power_control;

    if let Some(power_control) = &entry.power_control {
      entry.power_request.set_desired(power_control.desired);
    }

    entry.update(&instance_id, &self.nats).await;
  }

  async fn update_instance_play_control(&mut self, instance_id: String, maybe_instance_play_control: Option<InstancePlayControl>) {
    let entry = self.instances.entry(instance_id.clone()).or_default();
    entry.play_control = maybe_instance_play_control;

    if let Some(play_control) = &entry.play_control {
      entry.play_request.set_desired(play_control.desired);
    }

    entry.update(&instance_id, &self.nats).await;
  }

  async fn timer_tick(&mut self) {
    for (id, instance) in self.instances.iter_mut() {
      instance.update(id, &self.nats).await;
    }
  }
}

#[derive(Default)]
struct Instance {
  spec:          Option<InstanceSpec>,
  power_control: Option<InstancePowerControl>,
  play_control:  Option<InstancePlayControl>,
  power_request: RequestTracker<DesiredInstancePowerState, InstancePowerState>,
  play_request:  RequestTracker<DesiredInstancePlayState, InstancePlayState>,
  position_ms:   u64,
}

impl Instance {
  pub async fn update(&mut self, id: &str, nats: &Nats) -> bool {
    let power_is_fulfilled = self.update_power(id, nats).await;
    if power_is_fulfilled {
      self.update_play(id, nats).await
    } else {
      false
    }
  }

  async fn update_power(&mut self, id: &str, nats: &Nats) -> bool {
    let desired = self.power_request.get_desired();

    if self.power_request.should_request_update() {
      let success = if let Some(power_spec) = self.spec.as_ref().and_then(|spec| spec.power.as_ref()) {
        let command = power_spec.get_command(desired);
        self.power_request.update_requested(power_state);

        let command = vec![SetInstanceParameterRequest { parameter: command.parameter.clone(),
                                                         channel:   command.channel,
                                                         value:     command.value, }];

        let subject = set_instance_parameters_request(&power_spec.power_controller);

        match nats.request(subject, command).await {
          | Ok(SetInstanceParameterResponse::Success) => true,
          | Ok(other) => {
            error!("Failed to set power state for instance {}: {:?}", id, other);
            false
          }
          | Err(err) => {
            error!("Failed to set power state for instance {}: {:?}", id, err);
            false
          }
        }
      } else {
        true
      };

      if success {
        self.power_request.set_actual(desired.into());
      }

      self.power_request.update_requested_now();
    }

    self.power_request.is_fulfilled()
  }

  async fn update_play(&mut self, id: &str, nats: &Nats) -> bool {
    let desired = self.play_request.get_desired();

    if self.play_request.should_request_update() {
      let success = if let Some(media_spec) = self.spec.as_ref().and_then(|spec| spec.media.as_ref()) {
        let remaining = media_spec.duration_ms - self.position_ms;
        let command = media_spec.get_command(desired, (remaining as f64) / 1000.0);
        self.play_request.update_requested_now();

        let command = vec![SetInstanceParameterRequest { parameter: command.parameter.clone(),
                                                         channel:   command.channel,
                                                         value:     command.value, }];

        let subject = set_instance_parameters_request(id);

        match nats.request(subject, command).await {
          | Ok(SetInstanceParameterResponse::Success) => false,
          | Ok(other) => {
            error!("Failed to set play state for instance {}: {:?}", id, other);
            false
          }
          | Err(err) => {
            error!("Failed to set play state for instance {}: {:?}", id, err);
            false
          }
        }
      } else {
        true
      };

      if success {
        self.play_request.set_actual(desired.into());
      }

      self.play_request.update_requested_now();
    }

    self.play_request.is_fulfilled()
  }
}
