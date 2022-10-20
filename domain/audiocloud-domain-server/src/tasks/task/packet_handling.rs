use std::collections::HashMap;
use std::mem;

use actix_broker::BrokerIssue;

use audiocloud_api::audio_engine::CompressedAudio;
use audiocloud_api::domain::streaming::DiffStamped;
use audiocloud_api::{now, NodePadId, PadMetering};

use crate::tasks::messages::NotifyStreamingPacket;
use crate::tasks::task::TaskActor;

impl TaskActor {
    pub(crate) fn merge_peak_meters(&mut self, peak_meters: HashMap<NodePadId, PadMetering>) {
        for (pad_id, metering) in peak_meters {
            self.packet
                .pad_metering
                .entry(pad_id)
                .or_default()
                .push(DiffStamped::new(self.packet.created_at, metering));
        }
    }

    pub(crate) fn push_compressed_audio(&mut self, audio: CompressedAudio) {
        if self.engine.should_be_playing(&audio.play_id) {
            self.packet.audio.push(DiffStamped::new(self.packet.created_at, audio));
        }
    }

    pub(crate) fn maybe_send_packet(&mut self) {
        let packet_age = now() - self.packet.created_at;
        let packet_num_audio_frames = self.packet.audio.len();
        let max_packet_age = chrono::Duration::milliseconds(self.opts.max_packet_age_ms as i64);

        if packet_age >= max_packet_age || packet_num_audio_frames >= self.opts.max_packet_audio_frames {
            let packet = mem::take(&mut self.packet);
            self.issue_system_async(NotifyStreamingPacket { task_id: { self.id.clone() },
                                                            packet:  { packet }, });
        }
    }
}
