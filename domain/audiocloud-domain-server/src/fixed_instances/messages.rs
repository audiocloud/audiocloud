use std::collections::{HashMap, HashSet};

use actix::Message;
use reqwest::Url;

use audiocloud_api::common::instance::{DesiredInstancePlayState, ReportInstancePlayState, ReportInstancePowerState};
use audiocloud_api::common::newtypes::FixedInstanceId;
use audiocloud_api::common::task::{InstanceParameters, InstanceReports};
use audiocloud_api::common::time::Timestamped;
use audiocloud_api::{ModelValue, ParameterId};

use crate::DomainResult;

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyInstanceDriverUrl {
    pub instance_id: FixedInstanceId,
    pub base_url:    Option<Url>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<()>")]
pub struct SetInstanceParameters {
    pub instance_id: FixedInstanceId,
    pub parameters:  InstanceParameters,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<()>")]
pub struct MergeInstanceParameters {
    pub instance_id: FixedInstanceId,
    pub parameter:   ParameterId,
    pub value:       ModelValue,
    pub channel:     usize,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<()>")]
pub struct SetInstanceDesiredPlayState {
    pub instance_id: FixedInstanceId,
    pub desired:     DesiredInstancePlayState,
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
    pub reports:     InstanceReports,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyInstanceState {
    pub instance_id: FixedInstanceId,
    pub power:       Option<ReportInstancePowerState>,
    pub play:        Option<ReportInstancePlayState>,
    pub connected:   Timestamped<bool>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyInstanceError {
    pub instance_id: FixedInstanceId,
    pub error:       String,
}
