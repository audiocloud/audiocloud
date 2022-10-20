use std::collections::HashMap;

use actix::Message;

use audiocloud_api::cloud::domains::{DomainConfig, FixedInstanceRouting};
use audiocloud_api::{FixedInstanceId, Model, ModelId};

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyDomainConfiguration {
    pub config: DomainConfig,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyModels {
    pub models: HashMap<ModelId, Model>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyFixedInstanceRouting {
    pub routing: HashMap<FixedInstanceId, FixedInstanceRouting>,
}
