/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::VecDeque;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{env, thread};

use once_cell::sync::OnceCell;
use reaper_low::{static_vst_plugin_context, PluginContext};
use reaper_medium::{ProjectContext, ProjectRef, Reaper, ReaperSession};
use tracing::*;
use vst::prelude::*;

use audiocloud_api::api::codec::{Codec, MsgPack};
use audiocloud_api::audio_engine::event::EngineEvent;
use audiocloud_api::audio_engine::CompressedAudio;
use audiocloud_api::common::media::PlayId;
use audiocloud_api::newtypes::AppTaskId;

use crate::audio_engine::{PluginRegistry, ReaperEngine, ReaperEngineCommand, StreamingPluginCommand};
use crate::streaming::EncoderChain;

pub struct AudioCloudPlugin {
    activation:         Option<AudioCloudPluginActivation>,
    native_sample_rate: usize,
    host:               HostCallback,
}

struct AudioCloudPluginActivation {
    id:        AppTaskId,
    rx_plugin: flume::Receiver<StreamingPluginCommand>,
    tx_engine: flume::Sender<ReaperEngineCommand>,
    chain:     Option<EncoderChain>,
    context:   ProjectContext,
}

static SESSION_WRAPPER: OnceCell<SessionWrapper> = OnceCell::new();

impl Plugin for AudioCloudPlugin {
    fn get_info(&self) -> Info {
        Info { name: "Audiocloud Plugin".to_string(),
               unique_id: 0xbad1337,
               f64_precision: true,
               inputs: 2,
               outputs: 0,
               ..Default::default() }
    }

    fn new(host: HostCallback) -> Self
        where Self: Sized
    {
        eprintln!("new");

        SESSION_WRAPPER.get_or_init(|| {
                           eprintln!("==== first time boot ====");
                           init_env();
                           SessionWrapper(init_audio_engine(&host))
                       });

        Self { activation: None,
               native_sample_rate: 192_000,
               host }
    }

    fn init(&mut self) {
        eprintln!("init");

        {
            let reaper = Reaper::get();
            let project = reaper.enum_projects(ProjectRef::Current, 0)
                                .expect("REAPER project enum success")
                                .project;

            let maybe_id = unsafe {
                let mut notes = [0i8; 1024];

                reaper.low()
                      .GetSetProjectNotes(project.as_ptr(), false, notes.as_mut_ptr(), notes.len() as i32);

                let cstr = CStr::from_ptr(notes.as_ptr());

                AppTaskId::from_str(cstr.to_string_lossy().as_ref())
            };

            debug!(?maybe_id, "plugin init");

            self.activation = maybe_id.ok().map(|id| {
                                               let (tx_plugin, rx_plugin) = flume::unbounded();
                                               let tx_engine = PluginRegistry::register(id.clone(), tx_plugin);

                                               AudioCloudPluginActivation { id,
                                                                            rx_plugin,
                                                                            tx_engine,
                                                                            chain: None,
                                                                            context: ProjectContext::CurrentProject }
                                           });
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.native_sample_rate = rate as usize;
    }

    fn resume(&mut self) {
        debug!("resume");
    }

    fn suspend(&mut self) {
        debug!("suspend");
    }

    fn process_f64(&mut self, buffer: &mut AudioBuffer<f64>) {
        trace!(samples = buffer.samples(),
               inputs = buffer.input_count(),
               outputs = buffer.output_count(),
               "process_f64");

        if let Some(activation) = self.activation.as_mut() {
            if let Err(err) = activation.process(buffer, 2, self.native_sample_rate) {
                activation.error(err.to_string());
            }
        }
    }
}

impl AudioCloudPluginActivation {
    pub fn error(&self, error: String) {
        let _ = self.tx_engine.try_send(ReaperEngineCommand::PlayError(self.id.clone(), error));
    }

    pub(crate) fn process(&mut self, buf: &mut AudioBuffer<f64>, native_channels: usize, native_sample_rate: usize) -> anyhow::Result<()> {
        let drain = self.make_drain();

        while let Ok(cmd) = self.rx_plugin.try_recv() {
            // creating an encoder may be invasive, maybe a scoped thread and a mutex is better to handle it?
            self.dispatch_cmd(cmd, native_channels, native_sample_rate)?;
        }

        if let Some(chain) = self.chain.as_mut() {
            chain.process(buf, Reaper::get().get_play_position_2_ex(self.context).get())?;
            drain(chain.play.play_id, &mut chain.compressed)?;
        }

        Ok(())
    }

    fn dispatch_cmd(&mut self, cmd: StreamingPluginCommand, native_channels: usize, native_sample_rate: usize) -> anyhow::Result<()> {
        let drain = self.make_drain();

        match cmd {
            StreamingPluginCommand::Play { context, play } => {
                if let Some(chain) = self.chain.take() {
                    let play_id = chain.play.play_id;
                    let mut compressed = chain.finish()?;
                    drain(play_id, &mut compressed)?;
                }

                let play_id = play.play_id.clone();
                self.chain = Some(EncoderChain::new(play, native_channels, native_sample_rate)?);
                self.context = context;
                let _ = self.tx_engine.send(ReaperEngineCommand::PlayReady(self.id.clone(), play_id));
            }
            StreamingPluginCommand::Flush { play_id } => {
                let is_same_play_id = self.chain.as_ref().map(|chain| chain.play.play_id == play_id).unwrap_or(false);

                if is_same_play_id {
                    if let Some(chain) = self.chain.take() {
                        let mut compressed = chain.finish()?;
                        drain(play_id, &mut compressed)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn make_drain(&self) -> impl Fn(PlayId, &mut VecDeque<CompressedAudio>) -> anyhow::Result<()> {
        let id = self.id.clone();
        let tx_engine = self.tx_engine.clone();

        move |play_id: PlayId, compressed: &mut VecDeque<CompressedAudio>| -> anyhow::Result<()> {
            while let Some(compressed_audio) = compressed.pop_front() {
                debug!(play_id = %play_id, bytes = compressed_audio.buffer.len(), stream_pos = compressed_audio.stream_pos, timeline_pos = compressed_audio.timeline_pos, "drain");
                tx_engine.send(ReaperEngineCommand::Audio(id.clone(), play_id, compressed_audio))?;
            }

            Ok(())
        }
    }
}

impl Drop for AudioCloudPlugin {
    fn drop(&mut self) {
        if let Some(activation) = &self.activation {
            PluginRegistry::unregister(&activation.id);
        }
    }
}

fn init_env() {
    let _ = dotenv::dotenv();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,audiocloud_reaper_plugin=debug,audiocloud_api=debug,vst=warn");
    }

    tracing_subscriber::fmt::init();
}

#[instrument(skip(host))]
fn init_audio_engine(host: &HostCallback) -> ReaperSession {
    debug!("initializing audio engine");
    let ctx = PluginContext::from_vst_plugin(host, static_vst_plugin_context()).expect("REAPER PluginContext init success");
    let mut session = ReaperSession::load(ctx);
    let reaper = session.reaper().clone();

    debug!("Reaper interface now available globally");
    Reaper::make_available_globally(reaper);

    let (tx_cmd, rx_cmd) = flume::unbounded();
    let (tx_evt, rx_evt) = flume::unbounded::<EngineEvent>();

    let nats_url = env::var("NATS_URL").expect("NATS_URL env var must be set");
    let subscribe_topic = env::var("NATS_CMD_TOPIC").expect("NATS_CMD_TOPIC env var must be set");
    let publish_topic = env::var("NATS_EVT_TOPIC").expect("NATS_EVT_TOPIC env var must be set");
    let shared_media_root = PathBuf::from(
        env::var("SHARED_MEDIA_ROOT").expect("SHARED_MEDIA_ROOT env var must be set"),
    )
    .canonicalize()
    .expect("SHARED_MEDIA_ROOT must be a valid path");

    info!(?shared_media_root, "Shared media located at");

    debug!("Connecting to NATS");
    let connection = nats::connect(nats_url).expect("NATS connection success");

    debug!(topic = %subscribe_topic, "Subscribing to events");
    let subscription = connection.subscribe(&subscribe_topic).expect("NATS subscription success");

    thread::spawn({
        let tx_cmd = tx_cmd.clone();
        move || {
            // crossbeam channels that are compatible with nats crate
            while let Some(msg) = subscription.next() {
                if let Ok(cmd) = MsgPack.deserialize(&msg.data[..]) {
                    let (tx, rx) = flume::unbounded::<anyhow::Result<()>>();
                    if let Ok(_) = tx_cmd.send(ReaperEngineCommand::Request((cmd, tx))) {
                        thread::spawn(move || {
                            let result = match rx.recv_timeout(Duration::from_millis(500)) {
                                Err(_) => Err(format!("Request timed out")),
                                Ok(Err(err)) => Err(err.to_string()),
                                Ok(Ok(result)) => Ok(result),
                            };

                            let result = MsgPack.serialize(&result).expect("Response serialization success");
                            msg.respond(result).expect("NATS response send");
                        });
                    }
                }
            }
        }
    });

    thread::spawn(move || {
        while let Ok(evt) = rx_evt.recv() {
            if let Ok(encoded) = MsgPack.serialize(&evt) {
                if let Err(err) = connection.publish(&publish_topic, encoded) {
                    warn!(%err, "failed to publish event");
                }
            }
        }
    });

    debug!("Init plugin registry");
    PluginRegistry::init(tx_cmd.clone());

    session.plugin_register_add_csurf_inst(Box::new(ReaperEngine::new(shared_media_root, tx_cmd, rx_cmd, tx_evt)))
           .expect("REAPER audio engine control surface register success");

    info!("init complete");

    session
}

struct SessionWrapper(ReaperSession);

impl DerefMut for SessionWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for SessionWrapper {
    type Target = ReaperSession;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for SessionWrapper {
    fn drop(&mut self) {
        debug!("SessionWrapper dropped!");
    }
}

unsafe impl Send for SessionWrapper {}

unsafe impl Sync for SessionWrapper {}
