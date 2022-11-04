/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Addr;

use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceDriverResult, InstanceParametersUpdated, InstanceWithStatus,
    InstanceWithStatusList,
};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId, InstanceParameters};

use crate::messages::{GetInstanceMsg, GetInstancesMsg, SetDesiredStateMsg, SetParametersMsg};
use crate::supervisor::DriverSupervisor;

#[derive(Debug, Clone)]
pub struct DriverClient {
    addr: Addr<DriverSupervisor>,
}

impl DriverClient {
    pub fn new(addr: Addr<DriverSupervisor>) -> Self {
        Self { addr }
    }

    pub async fn get_instances(&self) -> InstanceDriverResult<InstanceWithStatusList> {
        self.addr.send(GetInstancesMsg).await.map_err(rpc_error)?
    }

    pub async fn get_instance(&self, instance_id: &FixedInstanceId) -> InstanceDriverResult<InstanceWithStatus> {
        self.addr
            .send(GetInstanceMsg { instance_id: instance_id.clone(), })
            .await
            .map_err(rpc_error)?
    }

    pub async fn set_parameters(&self,
                                instance_id: &FixedInstanceId,
                                parameters: InstanceParameters)
                                -> InstanceDriverResult<InstanceParametersUpdated> {
        self.addr
            .send(SetParametersMsg { instance_id: { instance_id.clone() },
                                     parameters:  { parameters }, })
            .await
            .map_err(rpc_error)?
    }

    pub async fn set_desired_play_state(&self,
                                        instance_id: &FixedInstanceId,
                                        play_state: DesiredInstancePlayState)
                                        -> InstanceDriverResult<DesiredInstancePlayStateUpdated> {
        self.addr
            .send(SetDesiredStateMsg { instance_id: { instance_id.clone() },
                                       play_state:  { play_state }, })
            .await
            .map_err(rpc_error)?
    }
}

fn rpc_error(error: impl ToString) -> InstanceDriverError {
    InstanceDriverError::RPC { error: error.to_string() }
}
