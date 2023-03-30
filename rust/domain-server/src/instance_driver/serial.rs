use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use boa_engine::JsValue;
use byteorder::ReadBytesExt;
use regex::Regex;
use serialport::{available_ports, FlowControl, SerialPort, SerialPortType};
use tracing::debug;

use api::driver::{
  InstanceDriverEvent, InstanceDriverReportEvent, SerialDriverConfig, SerialFlowControl, SerialReportConfig, SerialReportMatcher,
};
use api::instance::IdAndChannel;

use crate::instance_driver::bin_page_utils::remap_and_rescale_value;
use crate::instance_driver::scripting::{Script, ScriptingEngine};

use super::{Driver, Result};

pub struct SerialDriver {
  instance_id:          String,
  config:               SerialDriverConfig,
  port:                 Box<dyn SerialPort>,
  changes:              HashMap<(String, usize), f64>,
  scripting:            ScriptingEngine,
  fatal_error:          bool,
  parameter_transforms: HashMap<IdAndChannel, Script>,
  parameter_to_strings: HashMap<IdAndChannel, Script>,
  line_handler:         Option<Script>,
  regex_cache:          HashMap<String, Regex>,
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

        let mut regex_cache = HashMap::new();

        let line_handler = config.line_handler
                                 .as_ref()
                                 .map(|line_handler| scripting.compile(line_handler))
                                 .transpose()?;

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

        for (report_id, report_configs) in &config.reports {
          for (channel, report_config) in report_configs.iter().enumerate() {
            if let SerialReportMatcher::Matches { regex } = &report_config.matcher {
              if !regex_cache.contains_key(regex) {
                let regex = Regex::new(regex)?;
                regex_cache.insert(regex.to_string(), regex);
              }
            }
          }
        }

        let flow_control = match config.flow_control {
          | Some(SerialFlowControl::XonXoff) => FlowControl::Software,
          | Some(SerialFlowControl::RtsCts) => FlowControl::Hardware,
          | None => FlowControl::None,
        };

        let port = serialport::new(Cow::Borrowed(port.port_name.as_str()), config.baud_rate);
        let port = port.timeout(Duration::from_millis(1)).flow_control(flow_control).open()?;

        let changes = HashMap::new();

        let fatal_error = false;

        let instance_id = instance_id.to_owned();

        return Ok(SerialDriver { port,
                                 config,
                                 changes,
                                 scripting,
                                 fatal_error,
                                 instance_id,
                                 parameter_transforms,
                                 parameter_to_strings,
                                 line_handler,
                                 regex_cache });
      }
    }

    Err(anyhow!("No matching serial port found"))
  }

  fn set_parameter(&mut self, _shared: &mut Self::Shared, parameter: &str, channel: usize, value: f64) -> Result<()> {
    self.changes.insert((parameter.to_string(), channel), value);

    Ok(())
  }

  fn poll(&mut self, _shared: &mut Self::Shared, deadline: Instant) -> Result<Vec<InstanceDriverEvent>> {
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
        self.scripting.execute(transform, JsValue::Rational(value))
      } else {
        JsValue::Rational(value)
      };

      let Some(transform) = self.parameter_to_strings.get(&entry_id) else { continue };
      let value = self.scripting.execute(transform, value);
      let value = self.scripting.convert_to_string(value)
                  + parameter_config.line_terminator
                                    .clone()
                                    .unwrap_or_else(|| self.config.send_line_terminator.clone())
                                    .as_str();

      if let Err(err) = self.port.write(value.as_bytes()) {
        self.fatal_error = true;
        return Err(anyhow!("Failed to write {parameter_id}: '{value}' to serial port").context(err));
      }

      if self.config.read_response_after_every_send {
        let _ = read_line(&mut self.port, &self.config, deadline + Duration::from_millis(10))?;
      }
    }

    // read one line if we can
    if deadline - Instant::now() > Duration::from_millis(1) {
      if let Ok(line) = read_line(&mut self.port, &self.config, Instant::now() + Duration::from_millis(1)) {
        if let Ok(Some(event)) = handle_line(&self.instance_id,
                                             line,
                                             &self.config,
                                             &mut self.scripting,
                                             &self.regex_cache,
                                             self.line_handler.as_ref())
        {
          events.push(event);
        }
      }
    }

    Ok(events)
  }

  fn can_continue(&self) -> bool {
    !self.fatal_error
  }
}

fn read_line(port: &mut Box<dyn SerialPort>, config: &SerialDriverConfig, deadline: Instant) -> Result<String> {
  let mut line = String::new();

  while let Ok(ch) = port.read_u8() {
    line.push(ch as char);
    if line.ends_with(config.receive_line_terminator.as_str()) {
      return Ok(line);
    }
  }

  Err(anyhow!("Failed to read line from serial port before deadline"))
}

fn handle_line(instance_id: &str,
               line: String,
               config: &SerialDriverConfig,
               scripting: &mut ScriptingEngine,
               regex_cache: &HashMap<String, Regex>,
               line_handler_script: Option<&Script>)
               -> Result<Option<InstanceDriverEvent>> {
  for comment in &config.comments_start_with {
    if line.starts_with(comment.as_str()) {
      return Ok(None);
    }
  }

  for error in &config.errors_start_with {
    if line.starts_with(error.as_str()) {
      return Err(anyhow!("Serial port reported error with line '{line}'"));
    }
  }

  match line_handler_script {
    | Some(line_handler) => handle_line_with_script(instance_id, line, config, scripting, line_handler),
    | None => handle_line_with_pattern(instance_id, line, config, regex_cache),
  }
}

fn handle_line_with_script(instance_id: &str,
                           line: String,
                           config: &SerialDriverConfig,
                           scripting: &mut ScriptingEngine,
                           script: &Script)
                           -> Result<Option<InstanceDriverEvent>> {
  todo!()
}

fn handle_line_with_pattern(instance_id: &str,
                            line: String,
                            config: &SerialDriverConfig,
                            regex_cache: &HashMap<String, Regex>)
                            -> Result<Option<InstanceDriverEvent>> {
  let success = |report_id: &str, report_config: &SerialReportConfig, channel: usize, value: f64| {
    let value = remap_and_rescale_value(value,
                                        report_config.remap.as_ref(),
                                        report_config.rescale.as_ref(),
                                        report_config.clamp.as_ref())?;

    Ok::<_, anyhow::Error>(Some(InstanceDriverEvent::Report(InstanceDriverReportEvent { report_id: report_id.to_owned(),
                                                                                        instance_id: instance_id.to_owned(),
                                                                                        captured_at: Instant::now(),
                                                                                        channel,
                                                                                        value })))
  };

  for (report_id, report_configs) in &config.reports {
    for (channel, report_config) in report_configs.iter().enumerate() {
      match &report_config.matcher {
        | SerialReportMatcher::StringPrefix { prefix } =>
          if line.starts_with(prefix.as_str()) {
            let value = line[prefix.len()..].trim().parse::<f64>()?;
            return success(report_id, report_config, channel, value);
          },
        | SerialReportMatcher::Matches { regex } =>
          if let Some(regex) = regex_cache.get(regex) {
            if let Some(captures) = regex.captures(line.as_str()) {
              let Some(value) = captures.name("value") else { continue; };
              let value = value.as_str().trim().parse::<f64>()?;
              return success(report_id, report_config, channel, value);
            }
          },
      }
    }
  }

  Ok(None)
}
