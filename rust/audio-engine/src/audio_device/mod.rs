use std::collections::HashMap;
use std::time::Instant;

use anyhow::anyhow;
use derive_more::Display;
use tokio::sync::mpsc;

use crate::buffer::DeviceBuffers;

use super::Result;

pub mod audio_device_insert_node;
pub mod juce_device;
pub mod simulator_device;

/// Command sent to the device
#[derive(Debug, Clone, Display)]
pub enum DeviceCommand {
  /// Register a device client with the device
  #[display(fmt = "Register client {client_id}")]
  Register {
    /// Client ID. Must be unique.
    client_id: String,
    /// Sender to send commands to the client
    tx_client: mpsc::Sender<DeviceClientCommand>,
  },
  /// Unregister a device client from the device
  #[display(fmt = "Unregister client {client_id}")]
  Unregister {
    /// Client ID. Must be unique and successfully registered
    client_id: String,
  },
  /// Sent from client when it's done flipping buffers
  #[display(fmt = "Flip finished for client {client_id} generation {generation}")]
  FlipFinished {
    /// Client ID. Must be unique and successfully registered
    client_id:  String,
    /// Generation number of the buffers that were flipped
    generation: u64,
  },
}

/// Command sent to a device client
#[derive(Debug, Clone, Display)]
pub enum DeviceClientCommand {
  /// Sent from device when it's ready to flip buffers.
  ///
  /// The clients should submit [DeviceCommand::FlipFinished] as soon as possible, no later
  /// than the [deadline](DeviceClientCommand::Flip::deadline).
  #[display(fmt = "Flip device buffers for device {device_id} generation {generation}")]
  Flip {
    /// Device ID.
    device_id:  String,
    /// Device buffers to flip.
    buffers:    DeviceBuffers,
    /// Generation number of the buffers to flip.
    generation: u64,
    /// Deadline to submit [DeviceCommand::FlipFinished] by.
    deadline:   Instant,
  },
  /// Sent from device as a response to [DeviceCommand::Register].
  #[display(fmt = "Registered with device {device_id}")]
  Registered {
    /// Device ID.
    device_id: String,
  },
  /// Sent from device as a response to [DeviceCommand::Unregister].
  #[display(fmt = "Unregistered from device {device_id}")]
  Unregistered {
    /// Device ID.
    device_id: String,
  },
}

#[derive(Clone, Debug)]
pub struct AudioDevices {
  devices: HashMap<String, AudioDevice>,
}

#[derive(Clone, Debug)]
pub struct AudioDevice {
  tx_cmd: mpsc::Sender<DeviceCommand>,
  info:   AudioDeviceInfo,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct AudioDeviceInfo {
  pub latency:     u32,
  pub buffer_size: u32,
  pub sample_rate: u32,
  pub num_inputs:  usize,
  pub num_outputs: usize,
}

impl AudioDevices {
  pub fn send_command(&self, device_id: &str, cmd: DeviceCommand) -> Result {
    self.devices
        .get(device_id)
        .ok_or_else(|| anyhow!("Device {device_id} not found"))?
        .tx_cmd
        .try_send(cmd)
        .map_err(|e| anyhow!("Failed to send command to device {device_id}: {e}"))?;

    Ok(())
  }

  pub fn get_info(&self, device_id: &str) -> Result<AudioDeviceInfo> {
    Ok(self.devices
           .get(device_id)
           .ok_or_else(|| anyhow!("Device {device_id} not found"))?
           .info)
  }

  pub fn add_device(&mut self, device_id: String, device: AudioDevice) {
    self.devices.insert(device_id, device);
  }
}
