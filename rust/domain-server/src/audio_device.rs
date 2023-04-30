use std::sync::mpsc;

use derive_more::Display;
use std::time::Instant;

use audio_engine::buffer::DeviceBuffers;

pub mod juce_device;

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
