use std::path::Path;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

const SUPPORTED_FORMATS: [&'static str; 3] = ["wav", "flac", "mp3"];
const SUPPORTED_CODECS: [&'static str; 12] = ["pcm_s16le",
                                              "pcm_s24le",
                                              "pcm_s32le",
                                              "pcm_f32le",
                                              "pcm_f64le",
                                              "pcm_s16be",
                                              "pcm_s24be",
                                              "pcm_s32be",
                                              "pcm_f32be",
                                              "pcm_f64be",
                                              "mp3",
                                              "flac"];

pub async fn get_sample_rate(path: impl AsRef<Path>) -> anyhow::Result<u32> {
  let path = path.as_ref().to_string_lossy();
  let read_out = Command::new("ffprobe").args(["-v",
                                               "quiet",
                                               "-print_format",
                                               "json",
                                               "-show_format",
                                               "-show_streams",
                                               path.as_ref()])
                                        .kill_on_drop(true)
                                        .output()
                                        .await?;

  let data = serde_json::from_slice::<Metadata>(&read_out.stdout)?;

  if SUPPORTED_FORMATS.iter().find(|f| f == &&data.format.format_name).is_none() {
    return Err(anyhow!("unsupported format: {}", data.format.format_name));
  }

  for stream in data.streams {
    if SUPPORTED_CODECS.iter().find(|c| c == &&stream.codec_name).is_none() {
      continue;
    }

    if stream.codec_type == "audio" {
      if let Some(sample_rate) = stream.sample_rate {
        return Ok(sample_rate.parse::<u32>()?);
      }
    }
  }

  Err(anyhow!("no audio streams found with supported codecs"))
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
  streams: Vec<Stream>,
  format:  Format,
}

#[derive(Serialize, Deserialize, Debug)]
struct Format {
  format_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
  codec_type:  String,
  codec_name:  String,
  #[serde(default)]
  sample_rate: Option<String>,
}
