/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::future::Future;

use dashmap::DashMap;
use serde_json::Map;
use tracing::*;

use audiocloud_api::cloud::domains::{FixedInstanceConfig, InstanceDriverConfig};
use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceParametersUpdated, InstanceWithStatus, InstanceWithStatusList,
};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId, InstancePlayState, Timestamped};

use crate::driver::{DriverHandle, DriverRunner};

type Result<T = ()> = std::result::Result<T, InstanceDriverError>;

pub struct Instances {
    instances: DashMap<FixedInstanceId, InstanceController>,
}

struct InstanceController {
    handle:                 Option<DriverHandle>,
    config:                 FixedInstanceConfig,
    parameters:             serde_json::Value,
    desired_instance_state: Timestamped<DesiredInstancePlayState>,
    actual_instance_state:  Timestamped<InstancePlayState>,
}

impl Instances {
    pub fn new(config: InstanceDriverConfig) -> Result<Self> {
        let drivers = DashMap::new();

        for (id, config) in config.instances {
            drivers.insert(id,
                    InstanceController { handle:                 { None },
                                         config:                 { config },
                                         parameters:             { serde_json::Value::Object(Map::new()) },
                                         desired_instance_state: { Timestamped::new(DesiredInstancePlayState::Stopped { position: None }) },
                                         actual_instance_state:  { Timestamped::new(InstancePlayState::Stopped { position: None }) }, });
        }

        Ok(Self { instances: drivers })
    }

    pub fn set_parameters(&self,
                          fixed_instance_id: &FixedInstanceId,
                          parameters: serde_json::Value)
                          -> impl Future<Output = Result<InstanceParametersUpdated>> {
        let driver = self.instances
                         .get(fixed_instance_id)
                         .and_then(|controller| controller.handle.clone());
        let fixed_instance_id = fixed_instance_id.clone();

        async move {
            driver.ok_or_else(|| InstanceDriverError::InstanceNotFound { instance: fixed_instance_id, })?
                  .set_parameters(parameters)
                  .await
        }
    }

    pub fn set_desired_play_state(&self,
                                  fixed_instance_id: &FixedInstanceId,
                                  state: DesiredInstancePlayState)
                                  -> impl Future<Output = Result<DesiredInstancePlayStateUpdated>> {
        let driver = self.instances
                         .get(fixed_instance_id)
                         .and_then(|controller| controller.handle.clone());
        let fixed_instance_id = fixed_instance_id.clone();

        async move {
            driver.ok_or_else(|| InstanceDriverError::InstanceNotFound { instance: fixed_instance_id, })?
                  .set_desired_play_state(state)
                  .await
        }
    }

    pub fn update_drivers(&mut self) {
        for mut entry in self.instances.iter_mut() {
            if entry.handle.is_none() && !entry.config.maintenance.iter().any(|m| m.time.contains_now()) {
                match create_driver(entry.key(), &entry.config) {
                    Ok(handle) => {
                        entry.handle = Some(handle);
                    }
                    Err(error) => {
                        error!(%error, id = %entry.key(), "Failed to create driver");
                    }
                }
            }
        }
    }

    pub fn get_instances(&self) -> InstanceWithStatusList {
        let mut rv = InstanceWithStatusList::new();

        for instance in &self.instances {
            rv.push(InstanceWithStatus { id:         { instance.key().clone() },
                                         play_state: { None }, });
        }

        rv
    }

    pub fn get_parameters(&self, id: &FixedInstanceId) -> Result<serde_json::Value> {
        self.instances
            .get(id)
            .map(|instance| instance.parameters.clone())
            .ok_or_else(|| InstanceDriverError::InstanceNotFound { instance: id.clone() })
    }
}

pub fn create_driver(id: &FixedInstanceId, config: &FixedInstanceConfig) -> Result<DriverHandle> {
    use audiocloud_models as m;

    let json = config.additional.clone();
    match (id.manufacturer.as_str(), id.name.as_str()) {
        (m::distopik::NAME, m::distopik::dual1084::NAME) => {
            Ok(DriverRunner::run(id.clone(), crate::distopik::dual_1084::Config::from_json(json)?.driver(id)?))
        }
        (m::netio::NAME, m::netio::power_pdu_4c::NAME) => {
            Ok(DriverRunner::run(id.clone(), crate::netio::power_pdu_4c::Config::from_json(json)?.driver(id)?))
        }
        (manufacturer, name) => Err(InstanceDriverError::DriverNotSupported { manufacturer: manufacturer.to_owned(),
                                                                              name:         name.to_owned(), }),
    }
}

pub fn init(config: InstanceDriverConfig) -> Result<Instances> {
    Instances::new(config)
}
