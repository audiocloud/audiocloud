use actix::Message;

use audiocloud_api::domain::{DomainCommand, DomainEvent};

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyDomainSessionCommand {
    pub command: DomainCommand,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyDomainEvent {
    pub event: DomainEvent,
}
