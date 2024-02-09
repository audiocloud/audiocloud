use std::collections::HashMap;

use api::auth::Auth;
use api::instance::control::{instance_play_control_key, instance_power_control_key, InstancePlayControl, InstancePowerControl};
use api::instance::driver::events::{instance_driver_events, InstanceDriverEvent};
use api::instance::driver::requests::{set_instance_parameters_request, SetInstanceParameter, SetInstanceParameterResponse};
use api::instance::spec::{instance_spec_key, InstanceSpec};
use api::instance::state::{instance_connection_state_key, instance_play_state_key, instance_power_state_key};
use api::instance::{InstanceConnectionState, InstancePlayState, InstancePowerState};

use crate::nats::{EventStream, RequestStream, WatchStream};

use super::{Result, Service};

impl Service {
  pub async fn list_instances(&self, auth: Auth, filter: String) -> Result<HashMap<String, InstanceSpec>> {
    Ok(self.nats.instance_spec.scan(filter.as_str()).await?)
  }

  pub fn subscribe_to_instance_events(&self, instance_id: &str) -> EventStream<InstanceDriverEvent> {
    self.nats.subscribe_to_events(instance_driver_events(instance_id))
  }

  pub async fn publish_instance_driver_event(&self, instance_id: &str, event: InstanceDriverEvent) -> Result {
    self.nats.publish_event(instance_driver_events(instance_id), event).await
  }

  pub fn watch_instance_specs(&self, instance_id: &str) -> WatchStream<String, InstanceSpec> {
    self.nats.instance_spec.watch(instance_spec_key(&instance_id))
  }

  pub fn watch_all_instance_specs(&self) -> WatchStream<String, InstanceSpec> {
    self.nats.instance_spec.watch_all()
  }

  pub async fn set_instance_spec(&self, auth: Auth, instance_id: String, spec: InstanceSpec) -> Result {
    self.nats.instance_spec.put(instance_spec_key(&instance_id), spec).await?;

    Ok(())
  }

  pub fn watch_instance_power_control(&self, instance_id: &str) -> WatchStream<String, InstancePowerControl> {
    self.nats.instance_power_ctrl.watch(instance_power_control_key(&instance_id))
  }

  pub fn watch_all_instance_power_controls(&self) -> WatchStream<String, InstancePowerControl> {
    self.nats.instance_power_ctrl.watch_all()
  }

  pub async fn get_instance_power_state(&self, instance_id: &str) -> Result<Option<InstancePowerState>> {
    Ok(self.nats.instance_power_state.get(instance_power_state_key(&instance_id)).await?)
  }

  pub async fn set_instance_power_control(&self, instance_id: &str, power: InstancePowerControl) -> Result {
    self.nats
        .instance_power_ctrl
        .put(instance_power_control_key(&instance_id), power)
        .await?;

    Ok(())
  }

  pub fn watch_all_instance_power_states(&self) -> WatchStream<String, InstancePowerState> {
    self.nats.instance_power_state.watch_all()
  }

  pub async fn set_instance_power_state(&self, instance_id: &str, power: InstancePowerState) -> Result {
    self.nats
        .instance_power_state
        .put(instance_power_state_key(&instance_id), power)
        .await?;

    Ok(())
  }

  pub async fn set_instance_play_control(&self, instance_id: &str, play: InstancePlayControl) -> Result {
    self.nats
        .instance_play_ctrl
        .put(instance_play_control_key(&instance_id), play)
        .await?;

    Ok(())
  }

  pub fn watch_instance_play_control(&self, instance_id: &str) -> WatchStream<String, InstancePlayControl> {
    self.nats.instance_play_ctrl.watch(instance_play_control_key(&instance_id))
  }

  pub fn watch_all_instance_play_controls(&self) -> WatchStream<String, InstancePlayControl> {
    self.nats.instance_play_ctrl.watch_all()
  }

  pub fn watch_instance_play_state(&self, instance_id: &str) -> WatchStream<String, InstancePlayState> {
    self.nats.instance_play_state.watch(instance_play_state_key(&instance_id))
  }

  pub async fn get_instance_play_state(&self, instance_id: &str) -> Result<Option<InstancePlayState>> {
    Ok(self.nats.instance_play_state.get(instance_play_state_key(&instance_id)).await?)
  }

  pub async fn set_instance_play_state(&self, instance_id: &str, play: InstancePlayState) -> Result {
    self.nats
        .instance_play_state
        .put(instance_play_state_key(&instance_id), play)
        .await?;

    Ok(())
  }

  pub fn watch_instance_connection_state(&self, instance_id: &str) -> WatchStream<String, InstanceConnectionState> {
    self.nats
        .instance_connection_state
        .watch(instance_connection_state_key(&instance_id))
  }

  pub async fn set_instance_connection_state(&self, instance_id: &str, state: InstanceConnectionState) -> Result {
    self.nats
        .instance_connection_state
        .put(instance_connection_state_key(&instance_id), state)
        .await?;

    Ok(())
  }

  pub async fn set_instance_parameters(&self,
                                       instance_id: &str,
                                       request: Vec<SetInstanceParameter>)
                                       -> Result<SetInstanceParameterResponse> {
    self.nats.request(set_instance_parameters_request(&instance_id), request).await
  }

  pub fn serve_set_instance_parameters_requests(&self,
                                                instance_id: &str)
                                                -> RequestStream<Vec<SetInstanceParameter>, SetInstanceParameterResponse> {
    self.nats.serve_requests(set_instance_parameters_request(instance_id))
  }
}
