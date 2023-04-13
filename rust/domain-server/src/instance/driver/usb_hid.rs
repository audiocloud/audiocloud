use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::anyhow;
use chrono::Utc;
use hidapi::{HidApi, HidDevice};
use lazy_static::lazy_static;
use serde_json::json;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};
use tracing::{info, instrument, trace, warn};

use api::instance::driver::config::usb_hid::{UsbHidDriverConfig, UsbHidReportConfig};
use api::instance::driver::events::{InstanceDriverEvent, InstanceDriverReportEvent};
use api::instance::driver::requests::{SetInstanceParameter, SetInstanceParameterResponse, SetInstanceParametersRequest};

use crate::instance::driver::bin_page_utils::{write_binary_within_page, write_packed_value};
use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::driver::scripting::ScriptingEngine;

use super::bin_page_utils::{read_binary_within_page, read_packed_value, remap_and_rescale_value};
use super::Result;

pub struct UsbHidDriver {
  instance_id:     String,
  device:          HidDevice,
  config:          UsbHidDriverConfig,
  parameter_pages: HashMap<u8, ParameterPage>,
  report_pages:    HashMap<u8, ReportPage>,
  scripting:       ScriptingEngine,
  notifications:   Vec<oneshot::Sender<SetInstanceParameterResponse>>,
}

impl Drop for UsbHidDriver {
  fn drop(&mut self) {
    for notify in self.notifications.drain(..) {
      let _ = notify.send(SetInstanceParameterResponse::NotConnected);
    }
  }
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
  static ref HID_API: Arc<Mutex<HidApi>> = {
    let mut api = HidApi::new().expect("HID API init failed");
    api.refresh_devices().expect("HID API refresh failed");

    Arc::new(Mutex::new(api))
  };
}

fn create_api_and_dev(config: &UsbHidDriverConfig) -> Result<HidDevice> {
  let mut api = HID_API.lock().map_err(|err| anyhow!("HID API lock failed: {err}"))?;
  let mut device = None;

  if cfg!(not(target_os = "macos")) {
    trace!("Refreshing HID device list");
    let _ = api.refresh_devices();
  } else {
    trace!("Skipping refresh of HID devices on macOS because the HID scan in upstream is buggy");
  }

  for dev in api.device_list() {
    if config.product_id.map(|p| p == dev.product_id()).unwrap_or(true)
       && config.vendor_id.map(|p| p == dev.vendor_id()).unwrap_or(true)
       && config.serial_number
                .as_ref()
                .map(|p| Some(p.as_str()) == dev.serial_number())
                .unwrap_or(true)
    {
      device = Some(dev.open_device(&api)?);
      break;
    }
  }

  match device {
    | Some(device) => Ok(device),
    | None => Err(anyhow!("No device found matching config")),
  }
}

impl UsbHidDriver {
  fn new(instance_id: &str, config: UsbHidDriverConfig, scripting: ScriptingEngine) -> Result<Self> {
    let device = create_api_and_dev(&config)?;

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
                                             reports: config.reports
                                                            .iter()
                                                            .filter(|(_, reports)| reports.iter().any(|report| report.page == page.page))
                                                            .map(|(id, report)| (id.clone(), report.clone()))
                                                            .collect(), })
                             })
                             .collect();

    let instance_id = instance_id.to_owned();
    let notifications = vec![];

    Ok(Self { instance_id,
              device,
              parameter_pages,
              report_pages,
              config,
              scripting,
              notifications })
  }

  #[instrument(skip_all)]
  fn set_parameters(&mut self, parameters: SetInstanceParametersRequest, done: oneshot::Sender<SetInstanceParameterResponse>) {
    for SetInstanceParameter { parameter, channel, value } in parameters.changes {
      if let Some(parameter_configs) = self.config.parameters.get(&parameter) {
        if let Some(parameter_config) = parameter_configs.get(channel) {
          if let Some(page) = self.parameter_pages.get_mut(&parameter_config.page) {
            trace!(parameter, channel, value, "setting parameter");

            let Ok(value) = remap_and_rescale_value(value,
                                                parameter_config.remap.as_ref(),
                                                parameter_config.rescale.as_ref(),
                                                parameter_config.clamp.as_ref()) else { continue; };

            trace!(parameter, channel, remap_rescale = value, "remapped and rescaled to");

            let env = || json!({"value": value, "channel": channel, "parameter": parameter, "instance": self.instance_id});

            let value = match &parameter_config.transform {
              | None => value,
              | Some(script) => self.scripting.execute_sync(script.clone(), env()).as_f64().unwrap_or(value),
            };

            trace!(parameter, channel, transformed = value, "transformed to");

            let value = write_packed_value(value, &parameter_config.packing);
            trace!(packed = value.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "),
                   "packed to");

            write_binary_within_page(&mut page.data, value, &parameter_config.position);

            page.dirty = true;
          } else {
            warn!(parameter,
                  channel,
                  page = parameter_config.page,
                  "Parameter config references page that is not declared as parameter page")
          }
        } else {
          warn!(parameter, channel, "No parameter config for channel");
        }
      } else {
        warn!(parameter, "No parameter config for parameter");
      }
    }

    self.notifications.push(done);
  }

  #[instrument(err, skip(self, deadline))]
  fn poll(&mut self, deadline: Instant) -> Result<Vec<InstanceDriverEvent>> {
    if let Err(err) = self.send_dirty_pages() {
      self.notify(SetInstanceParameterResponse::ConnectionError);
      return Err(err);
    }

    // read the device
    let mut temp_page_buffer = [0u8; 0xff];

    let timeout_in_ms = (((deadline - Instant::now()).as_secs_f64() * 900.0).floor() as i32).min(1);
    match self.device.read_timeout(&mut temp_page_buffer, timeout_in_ms)? {
      | 0 => Ok(vec![]),
      | size => {
        let page = &temp_page_buffer[..size];
        let page_id = page[0] & self.config.frame_mask;

        self.on_page_received(page_id, &page)
      }
    }
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

    info!(page_id, "Received page");

    let mut events = vec![];
    let captured_at = Utc::now();

    for (report_id, report_configs) in rep_page.reports.iter() {
      for (channel, report_config) in report_configs.iter().enumerate() {
        trace!(report_id, channel, "Reading report");

        let value = read_binary_within_page(rep_page.data.as_slice(), &report_config.position);

        trace!(report_id, channel, "Binary value");

        let value = read_packed_value(&value, &report_config.packing);

        trace!(report_id, channel, "Unpacked value");

        let env = || json!({"value": value, "channel": channel, "report": report_id.clone(), "instance": self.instance_id.clone()});

        let value = match &report_config.transform {
          | Some(script) => self.scripting.execute_sync(script.clone(), env()).as_f64().unwrap_or(value),
          | None => value,
        };

        trace!(report_id, channel, "Transformed value");

        let value = remap_and_rescale_value(value, report_config.remap.as_ref(), report_config.rescale.as_ref(), None)?;

        trace!(report_id, channel, "Remapped value");

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
        trace!(page_id, len = page.data.len(), "sending dirty page");

        if let Err(err) = self.device.write(&page.data) {
          warn!(?err, page_id, len = page.data.len(), "Error while writing page to HID device");
          return Err(err.into());
        } else {
          trace!(page_id, "success");
        }
      }
    }

    self.notify_success();

    Ok(())
  }

  fn notify_success(&mut self) {
    self.notify(SetInstanceParameterResponse::Success);
  }

  fn notify(&mut self, response: SetInstanceParameterResponse) {
    for done in self.notifications.drain(..) {
      let _ = done.send(response.clone());
    }
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

#[instrument(skip_all, fields(instance_id))]
fn run_usb_driver_sync(instance_id: String,
                       config: UsbHidDriverConfig,
                       mut rx_cmd: Receiver<InstanceDriverCommand>,
                       tx_evt: Sender<InstanceDriverEvent>,
                       scripting_engine: ScriptingEngine)
                       -> Result {
  let mut instance = UsbHidDriver::new(&instance_id, config.clone(), scripting_engine)?;
  let read_duration = Duration::from_millis(config.read_duration_ms as u64);

  loop {
    while let Ok(cmd) = rx_cmd.try_recv() {
      match cmd {
        | InstanceDriverCommand::SetParameters(parameters, done) => {
          instance.set_parameters(parameters, done);
        }
        | InstanceDriverCommand::Terminate => {
          return Ok(());
        }
      }
    }

    for event in instance.poll(Instant::now() + read_duration)? {
      tx_evt.blocking_send(event)?;
    }
  }
}
