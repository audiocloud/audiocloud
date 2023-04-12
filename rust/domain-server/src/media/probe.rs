use std::path::Path;

use anyhow::bail;

pub fn get_sample_rate(path: impl AsRef<Path>) -> anyhow::Result<u32> {
  let format = ffprobe::ffprobe(path)?;
  let Some(stream_sample_rate) = format.streams.iter().find_map(|s| s.sample_rate.as_ref()) else { bail!("No audio streams found"); };
  let Ok(parsed) = stream_sample_rate.parse::<u32>() else { bail!("Failed to parse sample rate"); };

  Ok(parsed)
}
