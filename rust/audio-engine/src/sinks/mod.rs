pub mod monitor_sink_node;
pub mod streaming_sink_node;

pub mod reports {
  use std::collections::HashMap;

  use maplit::hashmap;

  use api::instance::model::{unit_db, ReportModel, ValueRange};

  pub const PEAK_LEVEL: &'static str = "peakLevel";
  pub const LUFS_LEVEL: &'static str = "lufsIntegratedLevel";
  pub const MEASURE_LUFS_FACTOR: f64 = 0.4; // 400ms

  fn peak_level_report(num_outputs: usize) -> ReportModel {
    ReportModel { channels: num_outputs,
                  range: ValueRange::volume(),
                  unit: unit_db(),
                  ..Default::default() }
  }

  fn lufs_level_report() -> ReportModel {
    ReportModel { channels: 1,
                  range: ValueRange::volume(),
                  unit: unit_db(),
                  ..Default::default() }
  }

  pub fn create(num_outputs: usize) -> HashMap<String, ReportModel> {
    hashmap! {
      PEAK_LEVEL.to_string() => peak_level_report(num_outputs),
      LUFS_LEVEL.to_string() => lufs_level_report(),
    }
  }
}

pub mod parameters {
  use std::collections::HashMap;

  use maplit::hashmap;

  use api::instance::model::{unit_db, ParameterModel, ValueRange};

  pub const GAIN: &'static str = "gain";

  fn gain(num_channels: usize) -> ParameterModel {
    ParameterModel { channels: num_channels,
                     range: ValueRange::volume(),
                     unit: unit_db(),
                     ..Default::default() }
  }

  pub fn create(num_channels: usize) -> HashMap<String, ParameterModel> {
    hashmap! {
      GAIN.to_string() => gain(num_channels),
    }
  }
}
