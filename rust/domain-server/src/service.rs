use std::collections::HashMap;

use api::instance::control::{InstancePlayControl, InstancePowerControl};
use api::instance::driver::events::{instance_driver_events, InstanceDriverEvent};
use api::instance::driver::requests::{set_instance_parameters_request, SetInstanceParameterResponse, SetInstanceParametersRequest};
use api::instance::spec::InstanceSpec;
use api::BucketKey;

use crate::nats::{EventStream, Nats};

#[derive(Clone)]
pub struct Service {
  pub nats: Nats,
}

pub type Result<T = ()> = anyhow::Result<T>;

impl Service {
  pub async fn list_instances(&self, filter: String) -> Result<HashMap<String, InstanceSpec>> {
    Ok(self.nats.instance_spec.scan(filter.as_str()).await?)
  }

  pub async fn set_instance_power_control(&self, instance_id: &str, power: InstancePowerControl) -> Result {
    self.nats.instance_power_ctrl.put(BucketKey::new(instance_id), power).await?;
    Ok(())
  }

  pub async fn set_instance_play_control(&self, instance_id: &str, play: InstancePlayControl) -> Result {
    self.nats.instance_play_ctrl.put(BucketKey::new(instance_id), play).await?;
    Ok(())
  }

  pub async fn set_instance_parameters(&self, request: SetInstanceParametersRequest) -> Result<SetInstanceParameterResponse> {
    self.nats
        .request(set_instance_parameters_request(&request.instance_id), request.changes)
        .await
  }

  pub fn subscribe_to_instance_events(&self, instance_id: &str) -> EventStream<InstanceDriverEvent> {
    self.nats.subscribe_to_events(instance_driver_events(instance_id))
  }
}
