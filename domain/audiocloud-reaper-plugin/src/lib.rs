extern crate core;

pub mod audio_engine;
pub mod audiocloud_plugin;
pub mod events;
pub mod streaming;

reaper_low::reaper_vst_plugin!();
vst::plugin_main!(audiocloud_plugin::AudioCloudPlugin);
