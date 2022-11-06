/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Context, Handler};
use tracing::*;

use crate::config::NotifyDomainConfiguration;
use crate::fixed_instances::FixedInstancesSupervisor;

impl Handler<NotifyDomainConfiguration> for FixedInstancesSupervisor {
    type Result = ();

    #[instrument(skip_all, name = "handle_notify_domain_configuration")]
    fn handle(&mut self, msg: NotifyDomainConfiguration, _ctx: &mut Self::Context) -> Self::Result {
        self.config = msg.config;
    }
}
