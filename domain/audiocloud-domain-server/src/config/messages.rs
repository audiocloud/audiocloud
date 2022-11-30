/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;

use coerce::actor::message::Message;

use audiocloud_api::cloud::domains::{DomainConfig, FixedInstanceRouting};
use audiocloud_api::{FixedInstanceId, Model, ModelId};

pub struct NotifyDomainConfiguration {
    pub config: DomainConfig,
}

impl Message for NotifyDomainConfiguration {
    type Result = ();
}

pub struct NotifyModels {
    pub models: HashMap<ModelId, Model>,
}

impl Message for NotifyModels {
    type Result = ();
}

pub struct NotifyFixedInstanceRouting {
    pub routing: HashMap<FixedInstanceId, FixedInstanceRouting>,
}

impl Message for NotifyFixedInstanceRouting {
    type Result = ();
}
