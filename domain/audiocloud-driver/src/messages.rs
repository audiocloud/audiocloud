/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Message;

use audiocloud_api::cloud::domains::{InstanceDriverConfig, TimestampedInstanceDriverConfig};
use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverResult, InstanceParametersUpdated, InstanceWithStatus, InstanceWithStatusList,
};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId, InstanceParameters, InstanceReports};

#[derive(Message, Debug, Clone)]
#[rtype(result = "InstanceDriverResult<InstanceParametersUpdated>")]
pub struct SetParametersMsg {
    pub instance_id: FixedInstanceId,
    pub parameters:  InstanceParameters,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "InstanceDriverResult<DesiredInstancePlayStateUpdated>")]
pub struct SetDesiredStateMsg {
    pub instance_id: FixedInstanceId,
    pub play_state:  DesiredInstancePlayState,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "InstanceDriverResult")]
pub struct SetInstanceDriverConfigMsg {
    pub config: TimestampedInstanceDriverConfig,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "InstanceDriverResult<InstanceWithStatusList>")]
pub struct GetInstancesMsg;

#[derive(Message, Debug, Clone)]
#[rtype(result = "InstanceDriverResult<InstanceWithStatus>")]
pub struct GetInstanceMsg {
    pub instance_id: FixedInstanceId,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct NotifyInstanceReportsMsg {
    pub instance_id: FixedInstanceId,
    pub reports:     InstanceReports,
}
