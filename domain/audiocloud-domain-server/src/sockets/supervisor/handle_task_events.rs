use actix::Handler;

use crate::sockets::SocketsSupervisor;
use crate::tasks::{NotifyTaskDeleted, NotifyTaskSecurity};

impl Handler<NotifyTaskDeleted> for SocketsSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskDeleted, ctx: &mut Self::Context) -> Self::Result {
        for clients in self.clients.values_mut() {
            clients.memberships.remove(&msg.task_id);
        }

        self.prune_unlinked_access();
    }
}

impl Handler<NotifyTaskSecurity> for SocketsSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskSecurity, ctx: &mut Self::Context) -> Self::Result {
        self.security
            .insert(msg.task_id.clone(), msg.security.clone());
    }
}
