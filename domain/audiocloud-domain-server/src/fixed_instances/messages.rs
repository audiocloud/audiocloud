use std::collections::{HashMap, HashSet};

use actix::Message;

use audiocloud_api::common::instance::{
    DesiredInstancePlayState, ReportInstancePlayState, ReportInstancePowerState,
};
use audiocloud_api::common::newtypes::FixedInstanceId;
use audiocloud_api::common::task::{InstanceParameters, InstanceReports};
use audiocloud_api::common::time::Timestamped;

use crate::DomainResult;

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<()>")]
pub struct SetInstanceParameters {
    pub instance_id: FixedInstanceId,
    pub parameters: InstanceParameters,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<()>")]
pub struct SetDesiredPowerChannel {
    pub instance_id: FixedInstanceId,
    pub channel: usize,
    pub power: bool,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<()>")]
pub struct SetInstanceDesiredPlayState {
    pub instance_id: FixedInstanceId,
    pub desired: DesiredInstancePlayState,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "HashMap<FixedInstanceId, NotifyInstanceState>")]
pub struct GetMultipleFixedInstanceState {
    pub instance_ids: HashSet<FixedInstanceId>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyFixedInstanceReports {
    pub instance_id: FixedInstanceId,
    pub reports: InstanceReports,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyInstancePowerChannelsChanged {
    pub instance_id: FixedInstanceId,
    pub power: Vec<bool>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyInstanceState {
    pub instance_id: FixedInstanceId,
    pub power: Option<ReportInstancePowerState>,
    pub play: Option<ReportInstancePlayState>,
    pub connected: Timestamped<bool>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyInstanceError {
    pub instance_id: FixedInstanceId,
    pub error: String,
}
