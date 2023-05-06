pub mod juce_source_reader_node;

pub mod reports {
  use std::collections::HashMap;

  use maplit::hashmap;

  use api::instance::model::{unit_db, ReportModel, ValueRange};

  pub const PEAK_LEVEL: &'static str = "peakLevel";
  pub const RMS_LEVEL: &'static str = "rmsLevel";

  fn peak_level_report(num_channels: usize) -> ReportModel {
    ReportModel { channels: num_channels,
                  range:    ValueRange::volume(),
                  unit:     unit_db(),
                  metadata: Default::default(), }
  }

  fn rms_level_report(num_channels: usize) -> ReportModel {
    ReportModel { channels: num_channels,
                  range:    ValueRange::volume(),
                  unit:     unit_db(),
                  metadata: Default::default(), }
  }

  pub fn create(num_channels: usize) -> HashMap<String, ReportModel> {
    hashmap! {
      PEAK_LEVEL.to_owned() => peak_level_report(num_channels),
      RMS_LEVEL.to_owned() => rms_level_report(num_channels),
    }
  }
}
