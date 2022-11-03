/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::time::Duration;

use actix::{AsyncContext, Context};
use nanoid::nanoid;

use tracing::*;

use audiocloud_api::domain::streaming::DomainServerMessage;

use crate::sockets::SocketsSupervisor;
use crate::ResponseMedia;

impl SocketsSupervisor {
    pub(crate) fn cleanup_stale_sockets(&mut self, ctx: &mut Context<Self>) {
        let max_init_wait_time = chrono::Duration::milliseconds(self.opts.socket_init_timeout as i64);

        for client in self.clients.values_mut() {
            client.sockets.retain(|id, socket| {
                              if socket.is_init_timed_out(self.opts.socket_init_timeout) {
                                  debug!(%id, "Supervisor cleaning up un-initialized socket");
                                  false
                              } else if !socket.is_valid(self.opts.socket_drop_timeout) {
                                  debug!(%id, "Supervisor cleaning up timed-out or disconnected socket");
                                  false
                              } else {
                                  true
                              }
                          });
        }

        self.prune_unlinked_access();
    }

    #[instrument(skip_all)]
    pub(crate) fn ping_active_sockets(&mut self, ctx: &mut Context<Self>) {
        for (client_id, client) in &self.clients {
            for (socket_id, socket) in &client.sockets {
                if !socket.is_valid(self.opts.socket_drop_timeout) {
                    debug!(%client_id, %socket_id, "skipping");
                    continue;
                }

                if let Err(error) = self.send_to_socket(socket,
                                                        DomainServerMessage::Ping { challenge: nanoid!() },
                                                        ResponseMedia::MsgPack,
                                                        ctx)
                {
                    warn!(%error, %socket_id, "Failed to ping socket");
                }
            }
        }

        self.clients.retain(|_, clients| !clients.sockets.is_empty());
    }

    pub(crate) fn prune_unlinked_access(&mut self) {
        // NOT sure we need this anymore
    }

    pub(crate) fn register_timers(&mut self, ctx: &mut Context<Self>) {
        ctx.run_interval(Duration::from_millis(20), Self::cleanup_stale_sockets);
        ctx.run_interval(Duration::from_millis(self.opts.socket_ping_interval), Self::ping_active_sockets);
    }
}
