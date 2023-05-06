use api::instance::model::{unit_db, ReportModel, ValueRange};

use crate::NodeEvent;

pub fn slice_peak_level_db(samples: &[f64]) -> f64 {
  gain_factor_to_db(samples.iter().map(|x| x.abs()).max_by(|a, b| a.total_cmp(b)).unwrap_or_default()).max(-100.0)
}

pub fn slice_rms_level_db(samples: &[f64]) -> f64 {
  let sum: f64 = samples.iter().map(|x| x.powi(2)).sum();
  let mean = sum / samples.len() as f64;

  gain_factor_to_db(mean.sqrt()).max(-100.0)
}

pub fn gain_factor_to_db(factor: f64) -> f64 {
  20.0 * factor.log10()
}

pub fn make_report(name: &str, channel_offset: usize) -> impl Fn((usize, f64)) -> NodeEvent + '_ {
  move |(channel, value)| {
    let name = name.to_owned();
    let channel = channel_offset + channel;

    NodeEvent::Report { name, channel, value }
  }
}

pub fn volume_level_report(num_channels: usize) -> ReportModel {
  ReportModel { range:    ValueRange::volume(),
                unit:     unit_db(),
                channels: num_channels,
                metadata: Default::default(), }
}
