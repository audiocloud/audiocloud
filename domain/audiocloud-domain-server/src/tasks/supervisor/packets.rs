use std::time::Duration;

use actix::{AsyncContext, Context, Handler};

use audiocloud_api::domain::streaming::StreamStats;
use audiocloud_api::domain::DomainError;
use audiocloud_api::{StreamingPacket, Timestamped};

use crate::tasks::messages::NotifyStreamingPacket;
use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::{GenerateStreamStats, GetStreamPacket};
use crate::DomainResult;

impl TasksSupervisor {
    pub(crate) fn update_packet_cache(&mut self, ctx: &mut Context<Self>) {
        let packet_cache_max_retention = chrono::Duration::milliseconds(self.opts.packet_cache_max_retention_ms as i64);

        self.tasks.values_mut().for_each(|task| {
                                   task.packet_cache.values_mut().for_each(|play_id_cache| {
                                                                     play_id_cache.retain(|_, packet| {
                                                                                      packet.elapsed() < packet_cache_max_retention
                                                                                  });
                                                                 });
                                   task.packet_cache.retain(|_, play_id_cache| !play_id_cache.is_empty());
                               });
    }

    pub(crate) fn register_packet_cache_cleanup(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(Duration::from_millis(250), Self::update_packet_cache);
    }
}

impl Handler<NotifyStreamingPacket> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyStreamingPacket, ctx: &mut Self::Context) -> Self::Result {
        if let Some(task) = self.tasks.get_mut(&msg.task_id) {
            task.packet_cache
                .entry(msg.packet.play_id.clone())
                .or_default()
                .insert(msg.packet.serial, Timestamped::new(msg.packet.clone()));
        }
    }
}

impl Handler<GenerateStreamStats> for TasksSupervisor {
    type Result = DomainResult<StreamStats>;

    fn handle(&mut self, msg: GenerateStreamStats, ctx: &mut Self::Context) -> Self::Result {
        let task_id = msg.task_id;
        let play_id = msg.play_id;

        match self.tasks.get(&task_id) {
            None => Err(DomainError::TaskNotFound { task_id }),
            Some(task) => match task.packet_cache.get(&play_id) {
                None => Err(DomainError::TaskStreamNotFound { task_id, play_id }),
                Some(packet_cache) => {
                    let low = packet_cache.keys().min();
                    let high = packet_cache.keys().max();

                    Ok(StreamStats { id:      { task_id },
                                     play_id: { play_id },
                                     state:   { task.state.play_state.value().clone() },
                                     low:     { low.cloned() },
                                     high:    { high.cloned() }, })
                }
            },
        }
    }
}

impl Handler<GetStreamPacket> for TasksSupervisor {
    type Result = DomainResult<StreamingPacket>;

    fn handle(&mut self, msg: GetStreamPacket, ctx: &mut Self::Context) -> Self::Result {
        let task_id = msg.task_id;
        let play_id = msg.play_id;
        let serial = msg.serial;

        match self.tasks.get(&task_id) {
            None => Err(DomainError::TaskNotFound { task_id }),
            Some(task) => match task.packet_cache.get(&play_id) {
                None => Err(DomainError::TaskStreamNotFound { task_id, play_id }),
                Some(packet_cache) => match packet_cache.get(&serial) {
                    None => Err(DomainError::TaskPacketNotFound { task_id, play_id, serial }),
                    Some(packet) => Ok(packet.value().clone()),
                },
            },
        }
    }
}
