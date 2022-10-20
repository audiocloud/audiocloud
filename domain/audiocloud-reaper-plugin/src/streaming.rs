use std::collections::VecDeque;
use std::ffi::c_void;
use std::ptr::slice_from_raw_parts;

use anyhow::anyhow;
use bytes::Bytes;
use libflac_sys::{
    FLAC__StreamEncoder, FLAC__StreamEncoderWriteStatus, FLAC__byte, FLAC__stream_encoder_delete, FLAC__stream_encoder_finish,
    FLAC__stream_encoder_init_stream, FLAC__stream_encoder_new, FLAC__stream_encoder_process, FLAC__stream_encoder_set_bits_per_sample,
    FLAC__stream_encoder_set_channels, FLAC__stream_encoder_set_sample_rate, FLAC__stream_encoder_set_streamable_subset,
};
use r8brain_rs::PrecisionProfile;
use tracing::*;
use vst::buffer::AudioBuffer;

use audiocloud_api::audio_engine::CompressedAudio;
use audiocloud_api::common::media::{PlayId, RequestPlay};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StreamingConfig {
    pub play_id:     PlayId,
    pub sample_rate: usize,
    pub channels:    usize,
    pub bit_depth:   usize,
}

pub struct AudioBuf {
    stream:   u64,
    timeline: f64,
    channels: Vec<Vec<f64>>,
}

struct Resampler {
    resamplers: Vec<(r8brain_rs::Resampler, Vec<f64>)>,
    timeline:   f64,
    stream:     u64,
    offset:     f64,
    from:       f64,
    to:         f64,
}

impl Resampler {
    pub fn new(num_channels: usize, from: f64, to: f64) -> Self {
        let max_src_len = 4096;
        let max_dst_len = (4096.0 * to / from * 2.0) as usize;
        let make_temp = || -> Vec<f64> {
            let mut rv = Vec::with_capacity(max_dst_len);
            rv.resize(max_dst_len, 0.0);
            rv
        };

        let resamplers =
            (0..num_channels).map(|_| (r8brain_rs::Resampler::new(from, to, max_src_len, 2.0, PrecisionProfile::Bits32), make_temp()))
                             .collect();

        let timeline = 0.0;
        let offset = 0.0;
        let stream = 0;

        Self { resamplers,
               timeline,
               stream,
               offset,
               from,
               to }
    }

    pub fn resample(&mut self, input: AudioBuf, out: &mut VecDeque<AudioBuf>) -> anyhow::Result<()> {
        let mut channels = vec![];

        self.timeline = input.timeline;
        self.offset -= input.channels[0].len() as f64 * self.to / self.from;

        for (ch, (resampler, temp)) in input.channels.into_iter().zip(self.resamplers.iter_mut()) {
            let size = resampler.process(&ch[..], &mut temp[..]);
            channels.push(Vec::from(&temp[..size]));
        }

        let len = channels.first().map(|v| v.len()).unwrap_or_default();

        out.push_back(AudioBuf { stream: self.stream,
                                 timeline: self.timeline + self.offset,
                                 channels });

        self.offset += len as f64;
        self.stream += len as u64;

        Ok(())
    }

    pub fn finish(&mut self, out: &mut VecDeque<AudioBuf>) -> anyhow::Result<()> {
        let mut channels = vec![];

        for (resampler, temp) in &mut self.resamplers {
            let size = resampler.flush(&mut temp[..]);
            channels.push(Vec::from(&temp[..size]));
        }

        let _len = channels.first().map(|v| v.len()).unwrap_or_default();

        out.push_back(AudioBuf { channels,
                                 stream: self.stream,
                                 timeline: self.timeline + self.offset });

        Ok(())
    }
}

pub struct FlacEncoder {
    encoder:         *mut FLAC__StreamEncoder,
    internals:       Box<SharedInternals>,
    tmp_buffer:      Vec<Vec<i32>>,
    bits_per_sample: usize,
    stream_pos:      u64,
    queued_len:      usize,
    play_id:         PlayId,
    timeline_pos:    f64,
    timeline_offset: f64,
    sample_rate:     f64,
}

impl Drop for FlacEncoder {
    fn drop(&mut self) {
        unsafe { FLAC__stream_encoder_delete(self.encoder) };
    }
}

impl FlacEncoder {
    #[instrument(skip_all)]
    pub fn new(play_id: PlayId, sample_rate: usize, channels: usize, bits_per_sample: usize) -> anyhow::Result<Self> {
        debug!(sample_rate, channels, bits_per_sample, "enter");

        if bits_per_sample > 16 {
            return Err(anyhow!("The reference encoder only supports 16-bit encoding"));
        }

        unsafe {
            let encoder = FLAC__stream_encoder_new();
            assert_eq!(FLAC__stream_encoder_set_channels(encoder, channels as u32), 1);
            assert_eq!(FLAC__stream_encoder_set_bits_per_sample(encoder, bits_per_sample as u32), 1);
            assert_eq!(FLAC__stream_encoder_set_streamable_subset(encoder, 1), 1);
            assert_eq!(FLAC__stream_encoder_set_sample_rate(encoder, sample_rate as u32), 1);

            let mut internals = Box::new(SharedInternals { buffer: vec![] });

            assert_eq!(FLAC__stream_encoder_init_stream(encoder,
                                                        Some(write),
                                                        None,
                                                        None,
                                                        None,
                                                        internals.as_mut() as *mut SharedInternals as *mut c_void),
                       0);

            let tmp_buffer = (0..channels).map(|_| Vec::new()).collect::<Vec<_>>();

            Ok(Self { encoder,
                      internals,
                      tmp_buffer,
                      bits_per_sample,
                      play_id,
                      queued_len: 0,
                      timeline_pos: 0.0,
                      timeline_offset: 0.0,
                      stream_pos: 0,
                      sample_rate: sample_rate as f64 })
        }
    }

    pub fn process(&mut self, data: AudioBuf, output: &mut VecDeque<CompressedAudio>) -> anyhow::Result<()> {
        let converter = match self.bits_per_sample {
            16 => |s: f64| dasp::sample::conv::f64::to_i16(s) as i32,
            24 => |s: f64| dasp::sample::conv::f64::to_i24(s).inner(),
            32 => |s: f64| dasp::sample::conv::f64::to_i32(s),
            i => {
                return Err(anyhow!("Only 16, 24 and 32 bits_per_sample supported, not {i}"));
            }
        };

        self.timeline_pos = data.timeline;
        self.timeline_offset -= data.channels[0].len() as f64 / self.sample_rate;

        let mut pointers = vec![];
        for (input, output) in data.channels.iter().zip(self.tmp_buffer.iter_mut()) {
            output.clear();
            output.extend(input.iter().copied().map(converter));
            pointers.push(output.as_ptr());
        }

        let len = self.tmp_buffer.iter().map(Vec::len).min().unwrap_or_default();

        if len > 0 {
            unsafe {
                if FLAC__stream_encoder_process(self.encoder, pointers.as_ptr(), len as u32) != 1 {
                    return Err(anyhow!("FLAC__stream_encoder_process failed"));
                }
            }

            self.queued_len += len;

            if !self.internals.buffer.is_empty() {
                output.push_back(CompressedAudio { play_id:      { self.play_id },
                                                   timeline_pos: { self.timeline_pos + self.timeline_offset },
                                                   stream_pos:   { self.stream_pos },
                                                   buffer:       { Bytes::from(self.internals.buffer.clone()) },
                                                   num_samples:  { 0 },
                                                   last:         { false }, });
                self.stream_pos += self.queued_len as u64;
                self.timeline_offset += (self.queued_len as f64) / self.sample_rate as f64;

                self.queued_len = 0;
                self.internals.buffer.clear();
            }
        }

        Ok(())
    }

    pub fn finish(&mut self, output: &mut VecDeque<CompressedAudio>) -> anyhow::Result<()> {
        unsafe {
            if FLAC__stream_encoder_finish(self.encoder) != 1 {
                return Err(anyhow!("FLAC__stream_encoder_finish failed"));
            }
        }

        output.push_back(CompressedAudio { play_id:      { self.play_id },
                                           timeline_pos: { self.timeline_pos + self.timeline_offset },
                                           stream_pos:   { self.stream_pos },
                                           buffer:       { Bytes::from(self.internals.buffer.clone()) },
                                           num_samples:  { 0 },
                                           last:         { true }, });

        self.internals.buffer.clear();

        Ok(())
    }
}

unsafe extern "C" fn write(_encoder: *const FLAC__StreamEncoder,
                           buffer: *const FLAC__byte,
                           bytes: usize,
                           _samples: u32,
                           _current_frame: u32,
                           client_data: *mut c_void)
                           -> FLAC__StreamEncoderWriteStatus {
    let internals = &mut *(client_data as *mut SharedInternals);
    internals.buffer
             .extend_from_slice(&*slice_from_raw_parts(buffer as *const u8, bytes));

    0
}

struct SharedInternals {
    buffer: Vec<u8>,
}

pub struct EncoderChain {
    resampler:      Option<Resampler>,
    encoder:        FlacEncoder,
    queue:          VecDeque<AudioBuf>,
    stream:         u64,
    pub play:       RequestPlay,
    pub compressed: VecDeque<CompressedAudio>,
}

unsafe impl Send for EncoderChain {}

impl EncoderChain {
    pub fn new(play: RequestPlay, native_channels: usize, native_sample_rate: usize) -> anyhow::Result<Self> {
        let play_sample_rate: usize = play.sample_rate.into();
        let resampler = if native_sample_rate == play_sample_rate {
            None
        } else {
            Some(Resampler::new(native_channels, play_sample_rate as f64, native_sample_rate as f64))
        };

        let encoder = FlacEncoder::new(play.play_id, native_sample_rate, native_channels, play.bit_depth.into())?;

        let queue = VecDeque::new();
        let compressed = VecDeque::new();
        let stream = 0;

        Ok(Self { play,
                  resampler,
                  encoder,
                  queue,
                  compressed,
                  stream })
    }

    pub fn process(&mut self, buf: &mut AudioBuffer<f64>, timeline: f64) -> anyhow::Result<()> {
        let (inputs, _) = buf.split();

        let buf = AudioBuf { timeline,
                             stream: self.stream,
                             channels: (0..inputs.len()).map(|i| Vec::from_iter(inputs.get(i).into_iter().copied()))
                                                        .collect() };

        self.stream += buf.channels[0].len() as u64;

        if let Some(resampler) = self.resampler.as_mut() {
            resampler.resample(buf, &mut self.queue)?;
        } else {
            self.queue.push_back(buf);
        }

        while let Some(buf) = self.queue.pop_front() {
            self.encoder.process(buf, &mut self.compressed)?;
        }

        Ok(())
    }

    pub fn finish(mut self) -> anyhow::Result<VecDeque<CompressedAudio>> {
        if let Some(resampler) = self.resampler.as_mut() {
            resampler.finish(&mut self.queue)?;
        }

        while let Some(buf) = self.queue.pop_front() {
            self.encoder.process(buf, &mut self.compressed)?;
        }

        self.encoder.finish(&mut self.compressed)?;

        Ok(self.compressed)
    }
}
