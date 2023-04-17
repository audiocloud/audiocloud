use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use byteorder::ReadBytesExt;
use chrono::Utc;
use futures::channel::{mpsc, oneshot};
use futures::SinkExt;
use regex::Regex;
use serde_json::json;
use serialport::{available_ports, FlowControl, SerialPort, SerialPortType};
use tokio::spawn;
use tracing::{debug, warn};

use api::instance::driver::config::serial::{SerialDriverConfig, SerialFlowControl, SerialReportConfig, SerialReportMatcher};
use api::instance::driver::events::{InstanceDriverEvent, InstanceDriverReportEvent};
use api::instance::driver::requests::{SetInstanceParameter, SetInstanceParameterResponse, SetInstanceParametersRequest};
use api::instance::IdAndChannel;

use crate::instance::driver::bin_page_utils::remap_and_rescale_value;
use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::driver::scripting::ScriptingEngine;

use super::Result;

pub struct SerialDriver {
  instance_id: String,
  config:      SerialDriverConfig,
  port:        Box<dyn SerialPort>,
  changes:     HashMap<(String, usize), f64>,
  notify_done: Vec<oneshot::Sender<SetInstanceParameterResponse>>,
  scripting:   ScriptingEngine,
  regex_cache: HashMap<String, Regex>,
}

impl Drop for SerialDriver {
  fn drop(&mut self) {
    for notify in self.notify_done.drain(..) {
      let _ = notify.send(SetInstanceParameterResponse::NotConnected);
    }
  }
}

type Config = SerialDriverConfig;

impl SerialDriver {
  fn new(instance_id: &str, config: Config, scripting: ScriptingEngine) -> Result<Self> {
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

        let mut regex_cache = HashMap::new();

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

        let instance_id = instance_id.to_owned();

        let notify_done = vec![];

        return Ok(SerialDriver { port,
                                 config,
                                 changes,
                                 notify_done,
                                 scripting,
                                 instance_id,
                                 regex_cache });
      }
    }

    Err(anyhow!("No matching serial port found"))
  }

  fn set_parameters(&mut self,
                    parameters: SetInstanceParametersRequest,
                    notify: oneshot::Sender<SetInstanceParameterResponse>)
                    -> Result<()> {
    for parameter in parameters.changes {
      let SetInstanceParameter { parameter, channel, value } = parameter;
      self.changes.insert((parameter, channel), value);
    }

    self.notify_done.push(notify);

    Ok(())
  }

  fn poll(&mut self, deadline: Instant) -> Result<Vec<InstanceDriverEvent>> {
    let mut events = vec![];

    for ((parameter_id, channel), value) in self.changes.drain() {
      let Some(parameter_configs) = self.config.parameters.get(&parameter_id) else { continue; };
      let Some(parameter_config) = parameter_configs.get(channel) else { continue; };

      let Ok(value) = remap_and_rescale_value(value,
                                              parameter_config.remap.as_ref(),
                                              parameter_config.rescale.as_ref(),
                                              parameter_config.clamp.as_ref()) else { continue; };

      let entry_id = IdAndChannel::from((parameter_id.as_str(), channel));

      let value = if let Some(transform) = &parameter_config.transform {
        self.scripting
            .execute_sync(transform.clone(), json!({ "value": value }))
            .to_string()
      } else {
        value.to_string()
      };

      let value = value
                  + parameter_config.line_terminator
                                    .clone()
                                    .unwrap_or_else(|| self.config.send_line_terminator.clone())
                                    .as_str();

      if let Err(err) = self.port.write(value.as_bytes()) {
        return Err(anyhow!("Failed to write {parameter_id}: '{value}' to serial port").context(err));
      }

      if self.config.read_response_after_every_send {
        let _ = read_line(&mut self.port, &self.config, deadline + Duration::from_millis(10))?;
      }
    }

    // read one line if we can
    if deadline - Instant::now() > Duration::from_millis(1) {
      if let Ok(line) = read_line(&mut self.port, &self.config, Instant::now() + Duration::from_millis(1)) {
        if let Ok(Some(event)) = handle_line(&self.instance_id, line, &self.config, self.scripting.clone(), &self.regex_cache) {
          events.push(event);
        }
      }
    }

    for notify in self.notify_done.drain(..) {
      let _ = notify.send(SetInstanceParameterResponse::Success);
    }

    Ok(events)
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
               scripting: ScriptingEngine,
               regex_cache: &HashMap<String, Regex>)
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

  match &config.line_handler {
    | Some(line_handler) => handle_line_with_script(instance_id, line, config, scripting, line_handler),
    | None => handle_line_with_pattern(instance_id, line, config, regex_cache),
  }
}

fn handle_line_with_script(instance_id: &str,
                           line: String,
                           config: &SerialDriverConfig,
                           scripting: ScriptingEngine,
                           script: &str)
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
                                                                                        captured_at: Utc::now(),
                                                                                        channel,
                                                                                        value })))
  };

  for (report_id, report_configs) in &config.reports {
    for (channel, report_config) in report_configs.iter().enumerate() {
      match &report_config.matcher {
        | SerialReportMatcher::StringPrefix { prefix, skip, take } =>
          if line.starts_with(prefix.as_str()) {
            // TODO: take
            let value = line[(prefix.len() + skip.unwrap_or_default())..].trim().parse::<f64>()?;
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

pub async fn run_serial_driver(instance_id: String,
                               config: SerialDriverConfig,
                               rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                               tx_evt: mpsc::Sender<InstanceDriverEvent>,
                               scripting_engine: ScriptingEngine)
                               -> Result {
  let handle = async_thread::spawn(move || run_serial_driver_sync(instance_id, config, rx_cmd, tx_evt, scripting_engine));
  match handle.join().await {
    | Ok(Ok(r)) => Ok(r),
    | Ok(Err(err)) => Err(err),
    | Err(_) => Err(anyhow!("Serial driver panicked")),
  }
}

fn run_serial_driver_sync(instance_id: String,
                          config: SerialDriverConfig,
                          mut rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                          mut tx_evt: mpsc::Sender<InstanceDriverEvent>,
                          scripting_engine: ScriptingEngine)
                          -> Result {
  let mut driver = SerialDriver::new(&instance_id, config.clone(), scripting_engine.clone())?;
  spawn({
    let mut tx_evt = tx_evt.clone();
    async move { tx_evt.send(InstanceDriverEvent::Connected { connected: true }).await }
  });

  loop {
    let mut start = Instant::now();

    // TODO: fix this
    // while let Ok(cmd) = rx_cmd.() {
    //   match cmd {
    //     | InstanceDriverCommand::SetParameters(parameters, tx_one) => driver.set_parameters(parameters, tx_one)?,
    //     | InstanceDriverCommand::Terminate => return Ok(()),
    //   }
    // }

    match driver.poll(Instant::now() + Duration::from_millis(25)) {
      | Ok(events) =>
        for event in events {
          let _ = tx_evt.try_send(event);
        },
      | Err(err) => {
        warn!(?err, "Failed to poll serial driver: {err}");
      }
    }

    let elapsed = start.elapsed();
    if elapsed < Duration::from_millis(25) {
      sleep(Duration::from_millis(25) - elapsed);
    }
  }
}
