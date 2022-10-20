use actix::Handler;

use tracing::*;

use audiocloud_api::domain::streaming::DomainServerMessage;
use audiocloud_api::{AppTaskId, TaskEvent, TaskPermissions};

use crate::sockets::supervisor::SupervisedClient;
use crate::sockets::SocketsSupervisor;
use crate::tasks::messages::NotifyStreamingPacket;

impl Handler<NotifyStreamingPacket> for SocketsSupervisor {
    type Result = ();

    #[instrument(name = "handle_streaming_packet", skip(self, ctx))]
    fn handle(&mut self, msg: NotifyStreamingPacket, ctx: &mut Self::Context) -> Self::Result {
        // TODO: at some point we may want a lookup from app tasks to clients

        for (client_id, client) in &self.clients {
            if self.client_can_on_task(client, &msg.task_id, TaskPermissions::can_audio) {
                let event = TaskEvent::StreamingPacket {
                    packet: msg.packet.clone(),
                };
                let msg = DomainServerMessage::TaskEvent {
                    task_id: { msg.task_id.clone() },
                    event: { event },
                };
                if let Err(error) = self.send_to_client(client_id, msg, ctx) {
                    warn!(%error, %client_id, "Failed to send streaming packet to client");
                }
            }
        }
    }
}

impl SocketsSupervisor {
    pub fn client_can_on_task(
        &self,
        client: &SupervisedClient,
        task_id: &AppTaskId,
        predicate: impl Fn(&TaskPermissions) -> bool,
    ) -> bool {
        client
            .memberships
            .get(task_id)
            .and_then(|secure_key| {
                self.security
                    .get(task_id)
                    .and_then(|task_security| task_security.security.get(secure_key))
                    .map(predicate)
            })
            .unwrap_or_default()
    }
}
