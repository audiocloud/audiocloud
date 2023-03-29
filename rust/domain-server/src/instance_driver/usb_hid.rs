use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::anyhow;
use hidapi::{HidApi, HidDevice};
use lazy_static::lazy_static;
use tracing::warn;

use api::driver::{InstanceDriverEvent, InstanceDriverReportEvent, UsbHidDriverConfig, UsbHidReportConfig};

use crate::instance_driver::bin_page_utils::{write_binary_within_page, write_packed_value};

use super::bin_page_utils::{read_binary_within_page, read_packed_value, remap_and_rescale_value};
use super::{Driver, Result};

pub struct UsbHidDriver {
  instance_id:             String,
  device:                  HidDevice,
  config:                  UsbHidDriverConfig,
  parameter_pages:         HashMap<u8, ParameterPage>,
  report_pages:            HashMap<u8, ReportPage>,
  encountered_fatal_error: bool,
}

struct ParameterPage {
  data:                    Vec<u8>,
  dirty:                   bool,
  waiting_for_report_page: Option<u8>,
}

struct ReportPage {
  data:    Vec<u8>,
  reports: HashMap<String, Vec<UsbHidReportConfig>>,
}

lazy_static! {
  static ref HID_API: Arc<Mutex<HidApi>> = Arc::new(Mutex::new(HidApi::new().expect("HID API init failed")));
}

impl Driver for UsbHidDriver {
  type Config = UsbHidDriverConfig;
  type Shared = Arc<Mutex<HidApi>>;

  fn create_shared() -> Result<Self::Shared> {
    Ok(HID_API.clone())
  }

  fn new(instance_id: &str, shared: &mut Self::Shared, config: UsbHidDriverConfig) -> Result<Self> {
    let mut shared = shared.lock().expect("HID API lock failed");
    shared.refresh_devices()?;

    for dev in shared.device_list() {
      if config.product_id.map(|p| p == dev.product_id()).unwrap_or(true)
         && config.vendor_id.map(|p| p == dev.vendor_id()).unwrap_or(true)
         && config.serial_number
                  .as_ref()
                  .map(|p| Some(p.as_str()) == dev.serial_number())
                  .unwrap_or(true)
      {
        let device = dev.open_device(&*shared)?;

        let parameter_pages = config.parameter_pages
                                    .iter()
                                    .map(|page| {
                                      (page.page,
                                       ParameterPage { data:                    vec![0u8; page.size],
                                                       dirty:                   false,
                                                       waiting_for_report_page: page.copy_from_report_page, })
                                    })
                                    .collect();

        let report_pages = config.report_pages
                                 .iter()
                                 .map(|page| {
                                   (page.page,
                                    ReportPage { data:    vec![0u8; page.size],
                                                 reports:
                                                   config.reports
                                                         .iter()
                                                         .filter(|(_, reports)| reports.iter().any(|report| report.page == page.page))
                                                         .map(|(id, report)| (id.clone(), report.clone()))
                                                         .collect(), })
                                 })
                                 .collect();

        let instance_id = instance_id.to_owned();
        let encountered_fatal_error = false;

        return Ok(Self { instance_id,
                         device,
                         parameter_pages,
                         report_pages,
                         config,
                         encountered_fatal_error });
      }
    }

    Err(anyhow!("No matching HID device found"))
  }

  fn set_parameter(&mut self, shared: &mut Self::Shared, parameter: &str, channel: usize, value: f64) -> Result<()> {
    if let Some(parameter_configs) = self.config.parameters.get(parameter) {
      if let Some(parameter_config) = parameter_configs.get(channel) {
        if let Some(page) = self.parameter_pages.get_mut(&parameter_config.page) {
          let value = remap_and_rescale_value(value,
                                              parameter_config.remap.as_ref(),
                                              parameter_config.rescale.as_ref(),
                                              parameter_config.clamp.as_ref())?;
          let value = write_packed_value(value, &parameter_config.packing);
          write_binary_within_page(&mut page.data, value, &parameter_config.position);

          page.dirty = true;
        } else {
          warn!(instance_id = &self.instance_id,
                page = parameter_config.page,
                parameter,
                channel,
                "Parameter config references page that is not declared as parameter page")
        }
      } else {
        warn!(instance_id = &self.instance_id,
              parameter, channel, "No parameter config for channel");
      }
    } else {
      warn!(instance_id = &self.instance_id, parameter, "No parameter config for parameter");
    }

    Ok(())
  }

  fn poll(&mut self, shared: &mut Self::Shared, deadline: Instant) -> Result<Vec<InstanceDriverEvent>> {
    self.send_dirty_pages()?;

    // read the device
    let mut temp_page_buffer = [0u8; 0xff];

    let timeout_in_ms = (((deadline - Instant::now()).as_secs_f64() * 900.0).floor() as i32).min(1);
    match self.device.read_timeout(&mut temp_page_buffer, timeout_in_ms) {
      | Ok(0) => Ok(vec![]),
      | Ok(size) => {
        let page = &temp_page_buffer[..size];
        let page_id = page[0];

        self.on_page_received(&page_id, &page)
      }
      | Err(err) => {
        self.encountered_fatal_error = true;
        warn!(instance_id = &self.instance_id, ?err, "Error while reading from HID device");
        Err(err.into())
      }
    }
  }

  fn can_continue(&self) -> bool {
    !self.encountered_fatal_error
  }
}

impl UsbHidDriver {
  fn on_page_received(&mut self, page_id: &u8, page: &[u8]) -> Result<Vec<InstanceDriverEvent>> {
    let mut page_found = true;
    if let Some(rep_page) = self.report_pages.get_mut(&page_id) {
      if page.len() == rep_page.data.len() {
        rep_page.data.copy_from_slice(&page);
        page_found = true;
      } else {
        warn!(instance_id = &self.instance_id, page_id, "Received page with wrong size");
      }

      self.maybe_update_param_page(page_id, page)
    }

    if page_found {
      return self.read_page_events(self.report_pages.get(page_id).unwrap());
    }

    Ok(vec![])
  }

  fn maybe_update_param_page(&mut self, page_id: &u8, page: &[u8]) {
    for param_page in self.parameter_pages.values_mut() {
      if param_page.waiting_for_report_page == Some(*page_id) {
        if param_page.data.len() == page.len() {
          param_page.data.copy_from_slice(&page);
          param_page.waiting_for_report_page = None;
        } else {
          warn!(instance_id = &self.instance_id,
                page_id,
                incoming_len = page.len(),
                param_len = param_page.data.len(),
                "Received report page page to fill parameter page with wrong size");
        }
      }
    }
  }

  fn read_page_events(&self, rep_page: &ReportPage) -> Result<Vec<InstanceDriverEvent>> {
    let mut events = vec![];
    let captured_at = Instant::now();

    for (report_id, report_configs) in rep_page.reports.iter() {
      for (channel, report_config) in report_configs.iter().enumerate() {
        let value = read_binary_within_page(rep_page.data.as_slice(), &report_config.position);
        let value = read_packed_value(&value, &report_config.packing);
        let value = remap_and_rescale_value(value, report_config.remap.as_ref(), report_config.rescale.as_ref(), None)?;

        events.push(InstanceDriverEvent::Report(InstanceDriverReportEvent { instance_id: self.instance_id.clone(),
                                                                            report_id: report_id.clone(),
                                                                            channel,
                                                                            value,
                                                                            captured_at }));
      }
    }

    Ok(events)
  }

  fn send_dirty_pages(&mut self) -> Result {
    for (page_id, page) in self.parameter_pages.iter_mut() {
      if page.dirty {
        page.data[0] = *page_id;
        page.dirty = false;

        if let Err(err) = self.device.write(&page.data) {
          self.encountered_fatal_error = true;
          warn!(instance_id = &self.instance_id,
                ?err,
                page_id,
                len = page.data.len(),
                "Error while writing page to HID device");
          return Err(err.into());
        }
      }
    }

    Ok(())
  }
}
