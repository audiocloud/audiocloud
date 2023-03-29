use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use serialport::{available_ports, SerialPort, SerialPortType};
use tracing::debug;

use api::driver::{InstanceDriverEvent, SerialDriverConfig};
use api::instance::IdAndChannel;

use crate::instance_driver::bin_page_utils::remap_and_rescale_value;
use crate::instance_driver::scripting::{Script, ScriptingEngine};

use super::{Driver, Result};

pub struct SerialDriver {
  config:               SerialDriverConfig,
  port:                 Box<dyn SerialPort>,
  changes:              HashMap<(String, usize), f64>,
  scripting:            ScriptingEngine,
  fatal_error:          bool,
  parameter_transforms: HashMap<IdAndChannel, Script>,
  parameter_to_strings: HashMap<IdAndChannel, Script>,
}

impl Driver for SerialDriver {
  type Config = SerialDriverConfig;
  type Shared = ();

  fn create_shared() -> Result<Self::Shared> {
    Ok(())
  }

  fn new(instance_id: &str, _shared: &mut Self::Shared, config: Self::Config) -> Result<Self> {
    let available = available_ports()?;

    for port in available {
      let matches = config.serial_port
                          .as_ref()
                          .map(|name| name.as_str() == port.port_name.as_str())
                          .unwrap_or(false)
                    || match port.port_type {
                      | SerialPortType::UsbPort(usb) =>
                        config.vendor_id.map(|id| id == usb.vid).unwrap_or(true)
                        && config.product_id.map(|id| id == usb.pid).unwrap_or(true)
                        && config.serial_number
                                 .as_ref()
                                 .map(|sn| Some(sn) == usb.serial_number.as_ref())
                                 .unwrap_or(true),
                      | _ => false,
                    };

      if matches {
        debug!(instance_id, port = port.port_name, "Found matching serial port");

        let mut scripting = ScriptingEngine::new()?;

        let mut parameter_transforms = HashMap::new();
        let mut parameter_to_strings = HashMap::new();

        for (parameter_id, parameter_configs) in &config.parameters {
          for (channel, parameter_config) in parameter_configs.iter().enumerate() {
            if let Some(transform) = parameter_config.transform.as_ref() {
              let script = scripting.compile(transform)?;
              parameter_transforms.insert(IdAndChannel::from((parameter_id, channel)), script);
            }

            if let Some(to_string) = parameter_config.to_string.as_ref() {
              let script = scripting.compile(to_string)?;
              parameter_to_strings.insert(IdAndChannel::from((parameter_id, channel)), script);
            }
          }
        }

        let port =
          serialport::new(Cow::Borrowed(port.port_name.as_str()), config.baud_rate).timeout(Duration::from_millis(config.receive_time_out_ms))
                                                                         .open()?;

        let changes = HashMap::new();

        let fatal_error = false;

        return Ok(SerialDriver { port,
                                 config,
                                 changes,
                                 scripting,
                                 fatal_error,
                                 parameter_transforms,
                                 parameter_to_strings });
      }
    }

    Err(anyhow!("No matching serial port found"))
  }

  fn set_parameter(&mut self, _shared: &mut Self::Shared, parameter: &str, channel: usize, value: f64) -> Result<()> {
    self.changes.insert((parameter.to_string(), channel), value);

    Ok(())
  }

  fn poll(&mut self, _shared: &mut Self::Shared, _deadline: Instant) -> Result<Vec<InstanceDriverEvent>> {
    let mut events = vec![];

    for ((parameter_id, channel), value) in self.changes.drain() {
      let Some(parameter_configs) = self.config.parameters.get(&parameter_id) else { continue; };
      let Some(parameter_config) = parameter_configs.get(channel) else { continue; };

      let Ok(value) = remap_and_rescale_value(value,
                                          parameter_config.remap.as_ref(),
                                          parameter_config.rescale.as_ref(),
                                          parameter_config.clamp.as_ref()) else { continue; };

      let entry_id = IdAndChannel::from((parameter_id.as_str(), channel));

      let value = if let Some(transform) = self.parameter_transforms.get(&entry_id) {
        self.scripting.eval_f64_to_f64(transform, value)
      } else {
        value
      };

      let Some(transform) = self.parameter_to_strings.get(&entry_id) else { continue };
      let value = self.scripting.eval_f64_to_string(transform, value)
                  + parameter_config.line_terminator
                                    .clone()
                                    .unwrap_or_else(|| self.config.send_line_terminator.clone())
                                    .as_str();

      if let Err(err) = self.port.write(value.as_bytes()) {
        self.fatal_error = true;
        return Err(anyhow!("Failed to write {parameter_id}: '{value}' to serial port").context(err));
      }
    }

    Ok(events)
  }

  fn can_continue(&self) -> bool {
    !self.fatal_error
  }
}
