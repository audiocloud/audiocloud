use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use chrono::Utc;
use governor::{Quota, RateLimiter};
use hidapi::{HidApi, HidDevice};
use lazy_static::lazy_static;
use nonzero_ext::nonzero;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, info, instrument, trace, warn};

use api::instance::driver::config::usb_hid::{UsbHidDriverConfig, UsbHidReportConfig};
use api::instance::driver::events::{InstanceDriverEvent, InstanceDriverReportEvent};
use api::instance::driver::requests::SetInstanceParameterResponse;
use api::instance::IdAndChannel;

use crate::instance::driver::bin_page_utils::{write_binary_within_page, write_packed_value};
use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::driver::scripting::ScriptingEngine;

use super::bin_page_utils::{read_binary_within_page, read_packed_value, remap_and_rescale_value};
use super::Result;

pub struct UsbHidDriver {
  instance_id:             String,
  device:                  HidDevice,
  config:                  UsbHidDriverConfig,
  parameter_pages:         HashMap<u8, ParameterPage>,
  report_pages:            HashMap<u8, ReportPage>,
  scripting:               ScriptingEngine,
  encountered_fatal_error: bool,
}

struct ParameterPage {
  header:                  Vec<u8>,
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

impl UsbHidDriver {
  #[instrument(err, skip(config, scripting, instance_id))]
  fn new(instance_id: &str, config: UsbHidDriverConfig, scripting: ScriptingEngine) -> Result<Self> {
    let mut shared = HID_API.lock().expect("HID API lock failed");
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
                                       ParameterPage { header:                  page.header.clone(),
                                                       data:                    vec![0u8; page.size],
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
                         encountered_fatal_error,
                         scripting });
      }
    }

    Err(anyhow!("No matching HID device found"))
  }

  #[instrument(err, skip(self))]
  fn set_parameter(&mut self, parameter_id: &str, channel: usize, value: f64) -> Result<()> {
    if let Some(parameter_configs) = self.config.parameters.get(parameter_id) {
      if let Some(parameter_config) = parameter_configs.get(channel) {
        if let Some(page) = self.parameter_pages.get_mut(&parameter_config.page) {
          let value = remap_and_rescale_value(value,
                                              parameter_config.remap.as_ref(),
                                              parameter_config.rescale.as_ref(),
                                              parameter_config.clamp.as_ref())?;

          trace!(remap_rescale = value, "remapped and rescaled to");

          let env = || json!({"value": value, "channel": channel, "parameter": parameter_id.clone(), "instance": self.instance_id.clone()});

          let value = match &parameter_config.transform {
            | None => value,
            | Some(script) => self.scripting.execute_sync(script.clone(), env()).as_f64().unwrap_or(value),
          };

          trace!(transformed = value, "transformed to");

          let value = write_packed_value(value, &parameter_config.packing);
          info!(packed = value.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "),
                "packed to");

          write_binary_within_page(&mut page.data, value, &parameter_config.position);

          page.dirty = true;
        } else {
          warn!(page = parameter_config.page,
                "Parameter config references page that is not declared as parameter page")
        }
      } else {
        warn!("No parameter config for channel");
      }
    } else {
      warn!("No parameter config for parameter");
    }

    Ok(())
  }

  #[instrument(err, skip(self, deadline))]
  fn poll(&mut self, deadline: Instant) -> Result<Vec<InstanceDriverEvent>> {
    self.send_dirty_pages()?;

    // read the device
    let mut temp_page_buffer = [0u8; 0xff];

    let timeout_in_ms = (((deadline - Instant::now()).as_secs_f64() * 900.0).floor() as i32).min(1);
    match self.device.read_timeout(&mut temp_page_buffer, timeout_in_ms) {
      | Ok(0) => Ok(vec![]),
      | Ok(size) => {
        let page = &temp_page_buffer[..size];
        let page_id = page[0] & self.config.frame_mask;

        self.on_page_received(page_id, &page)
      }
      | Err(err) => {
        self.encountered_fatal_error = true;
        Err(err.into())
      }
    }
  }

  fn can_continue(&self) -> bool {
    !self.encountered_fatal_error
  }
}

impl UsbHidDriver {
  #[instrument(err, skip(self, page))]
  fn on_page_received(&mut self, page_id: u8, page: &[u8]) -> Result<Vec<InstanceDriverEvent>> {
    let mut page_found = false;
    if let Some(rep_page) = self.report_pages.get_mut(&page_id) {
      if page.len() == rep_page.data.len() {
        rep_page.data.copy_from_slice(&page);
        page_found = true;
      } else {
        warn!("Received page with wrong size");
      }

      self.maybe_update_param_page(page_id, page)
    }

    if page_found {
      return self.read_page_events(page_id);
    }

    Ok(vec![])
  }

  #[instrument(skip(self, page))]
  fn maybe_update_param_page(&mut self, page_id: u8, page: &[u8]) {
    for param_page in self.parameter_pages.values_mut() {
      if param_page.waiting_for_report_page == Some(page_id) {
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

  #[instrument(err, skip(self))]
  fn read_page_events(&mut self, page_id: u8) -> Result<Vec<InstanceDriverEvent>> {
    let Some(rep_page) = self.report_pages.get_mut(&page_id) else {
      return Err(anyhow!("Received page '{page_id}' that is not declared as report page"));
    };

    let mut events = vec![];
    let captured_at = Utc::now();

    for (report_id, report_configs) in rep_page.reports.iter() {
      for (channel, report_config) in report_configs.iter().enumerate() {
        let report_channel_id = IdAndChannel::from((report_id, channel));

        let value = read_binary_within_page(rep_page.data.as_slice(), &report_config.position);
        let value = read_packed_value(&value, &report_config.packing);

        let env = || json!({"value": value, "channel": channel, "report": report_id.clone(), "instance": self.instance_id.clone()});

        let value = match &report_config.transform {
          | Some(script) => self.scripting.execute_sync(script.clone(), env()).as_f64().unwrap_or(value),
          | None => value,
        };

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

  #[instrument(err, skip(self))]
  fn send_dirty_pages(&mut self) -> Result {
    for (page_id, page) in self.parameter_pages.iter_mut() {
      if page.dirty {
        for (pos, byte) in page.header.iter().copied().enumerate() {
          page.data[pos] = byte;
        }
        page.dirty = false;
        debug!(page_id, len = page.data.len(), "sending dirty page");

        if let Err(err) = self.device.write(&page.data) {
          warn!(?err, page_id, len = page.data.len(), "Error while writing page to HID device");
          self.encountered_fatal_error = true;
          return Err(err.into());
        } else {
          debug!(page_id, "success");
        }
      }
    }

    Ok(())
  }
}

pub async fn run_usb_driver(instance_id: String,
                            config: UsbHidDriverConfig,
                            rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                            tx_evt: mpsc::Sender<InstanceDriverEvent>,
                            scripting_engine: ScriptingEngine)
                            -> Result {
  let usb_thread = async_thread::spawn(move || run_usb_driver_sync(instance_id, config, rx_cmd, tx_evt, scripting_engine));
  match usb_thread.join().await {
    | Ok(Ok(r)) => Ok(r),
    | Ok(Err(err)) => Err(err),
    | Err(err) => Err(anyhow!("HID USB thread panicked: {:?}", err)),
  }
}

#[instrument(err, skip(config, rx_cmd, tx_evt, scripting_engine))]
fn run_usb_driver_sync(instance_id: String,
                       config: UsbHidDriverConfig,
                       mut rx_cmd: Receiver<InstanceDriverCommand>,
                       tx_evt: Sender<InstanceDriverEvent>,
                       scripting_engine: ScriptingEngine)
                       -> Result {
  let mut instance: Option<UsbHidDriver> = None;
  let respawn_governor = RateLimiter::direct(Quota::per_minute(nonzero!(5u32)));

  loop {
    let instance_ready = instance.as_ref().map(|i| i.can_continue()).unwrap_or(false);
    let start = Instant::now();

    if let Ok(cmd) = rx_cmd.try_recv() {
      match cmd {
        | InstanceDriverCommand::SetParameters(params, completed) =>
          if let Some(instance) = instance.as_mut() {
            for p in params.changes {
              if let Err(err) = instance.set_parameter(&p.parameter, p.channel, p.value) {
                warn!(?err, "Error while setting parameter: {err}");
              }
            }
          } else {
            let _ = completed.send(SetInstanceParameterResponse::NotConnected);
          },
        | InstanceDriverCommand::Terminate => return Ok(()),
      }
    }

    if !instance_ready && respawn_governor.check().is_ok() {
      drop(instance.take());

      instance = match UsbHidDriver::new(&instance_id, config.clone(), scripting_engine.clone()) {
        | Ok(instance) => {
          let _ = tx_evt.try_send(InstanceDriverEvent::Connected { connected: true });
          Some(instance)
        }
        | Err(err) => {
          let _ = tx_evt.try_send(InstanceDriverEvent::Connected { connected: false });
          warn!(?err, "Failed to create USB HID driver instance: {err}");
          None
        }
      };
    }

    if instance_ready {
      if let Some(instance) = instance.as_mut() {
        match instance.poll(Instant::now() + Duration::from_millis(20)) {
          | Ok(events) =>
            for event in events {
              let _ = tx_evt.send(event);
            },
          | Err(err) => {
            warn!(?err, "Error while polling USB HID driver instance: {err}");
          }
        }
      }
    }

    let elapsed = start.elapsed();
    if elapsed < Duration::from_millis(20) {
      sleep(Duration::from_millis(20) - elapsed);
    }
  }
}
