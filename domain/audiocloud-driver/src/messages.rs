/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use coerce::actor::message::Message;

use audiocloud_api::cloud::domains::{TimestampedInstanceDriverConfig};
use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverResult, InstanceParametersUpdated, InstanceWithStatus, InstanceWithStatusList,
};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId, InstanceParameters, InstanceReports};

pub struct SetParametersMsg {
    pub instance_id: FixedInstanceId,
    pub parameters:  InstanceParameters,
}

impl Message for SetParametersMsg {
    type Result = InstanceDriverResult<InstanceParametersUpdated>;
}

pub struct SetDesiredStateMsg {
    pub instance_id: FixedInstanceId,
    pub play_state:  DesiredInstancePlayState,
}

impl Message for SetDesiredStateMsg {
    type Result = InstanceDriverResult<DesiredInstancePlayStateUpdated>;
}

pub struct SetInstanceDriverConfigMsg {
    pub config: TimestampedInstanceDriverConfig,
}

impl Message for SetInstanceDriverConfigMsg {
    type Result = InstanceDriverResult;
}

pub struct GetInstancesMsg;

impl Message for GetInstancesMsg {
    type Result = InstanceDriverResult<InstanceWithStatusList>;
}

pub struct GetInstanceMsg {
    pub instance_id: FixedInstanceId,
}

impl Message for GetInstanceMsg {
    type Result = InstanceDriverResult<InstanceWithStatus>;
}

#[derive(Clone)]
pub struct NotifyInstanceReportsMsg {
    pub instance_id: FixedInstanceId,
    pub reports:     InstanceReports,
}

impl Message for NotifyInstanceReportsMsg {
    type Result = ();
}
