/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use coerce::actor::message::Message;

use audiocloud_api::domain::{DomainCommand, DomainEvent};

pub struct NotifyDomainSessionCommand {
    pub command: DomainCommand,
}

impl Message for NotifyDomainSessionCommand {
    type Result = ();
}

pub struct NotifyDomainEvent {
    pub event: DomainEvent,
}

impl Message for NotifyDomainEvent {
    type Result = ();
}
