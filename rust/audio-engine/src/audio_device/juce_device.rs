use std::collections::{HashMap, HashSet};
use std::ffi::{c_void, CString};
use std::time::{Duration, Instant};

use anyhow::bail;
use derive_more::Display;
use tokio::sync::mpsc;

use crate::audio_device::{DeviceClientCommand, DeviceCommand};
use crate::buffer::DeviceBuffers;
use crate::juce;
use crate::Result;

// TODO: Q: do we really need crossbeam_channel here? Can't we have tokio or std mpsc?
// TODO: A: yes, as it has timeouts, which are missing with tokio or std mpsc

pub struct JuceAudioDevice {
  id:            String,
  device_id:     i32,
  clients:       HashMap<String, Client>,
  generation:    u64,
  rx_cmd:        crossbeam_channel::Receiver<DeviceCommand>,
  flip_duration: Duration,
}

impl Drop for JuceAudioDevice {
  fn drop(&mut self) {
    unsafe {
      juce::delete_audio_device(self.device_id);
    }
  }
}

impl JuceAudioDevice {
  pub fn new(id: String,
             type_name: JuceAudioDeviceType,
             input_name: &str,
             output_name: &str,
             input_channel_count: usize,
             output_channel_count: usize,
             sample_rate: usize,
             buffer_size: usize)
             -> Result<(crossbeam_channel::Sender<DeviceCommand>, Box<Self>)> {
    let (tx_cmd, rx_cmd) = crossbeam_channel::bounded(0x100);
    let type_name = type_name.to_cstring();
    let input_name = CString::new(input_name)?;
    let output_name = CString::new(output_name)?;

    let device_id = unsafe {
      juce::create_audio_device(type_name.as_ptr(),
                                input_name.as_ptr(),
                                output_name.as_ptr(),
                                input_channel_count as i32,
                                output_channel_count as i32,
                                sample_rate as i32,
                                buffer_size as i32)
    };

    if device_id < 0 {
      bail!("Failed to create audio device, error code: {device_id}");
    }

    let clients = HashMap::new();
    let generation = 0;

    let flip_duration = Duration::from_secs_f64(buffer_size as f64 / sample_rate as f64);

    let rv = Box::new(Self { id,
                             device_id,
                             clients,
                             generation,
                             rx_cmd,
                             flip_duration });

    Ok((tx_cmd, rv))
  }

  pub fn start(&mut self) {
    unsafe {
      juce::start_audio_device(self.device_id, callback, self as *mut _ as *mut _);
    }
  }

  fn handle_callback(&mut self, buffers: DeviceBuffers) {
    let deadline = Instant::now() + self.flip_duration;
    let mut waiting_for = HashSet::new();
    let mut new_registrations = HashMap::new();

    while let Ok(event) = self.rx_cmd.try_recv() {
      match event {
        | DeviceCommand::Register { client_id, tx_client } => {
          let _ = tx_client.send(DeviceClientCommand::Registered { device_id: self.id.clone(), });
          new_registrations.insert(client_id.clone(), Client { tx_cmd: tx_client });
        }
        | DeviceCommand::Unregister { client_id } =>
          if let Some(client) = self.clients.remove(&client_id) {
            let _ = client.tx_cmd
                          .send(DeviceClientCommand::Unregistered { device_id: self.id.clone(), });
          },
        | DeviceCommand::FlipFinished { .. } => {
          // too late, ignore
        }
      }
    }

    for (client_id, client) in &self.clients {
      let _ = client.tx_cmd.send(DeviceClientCommand::Flip { device_id: self.id.clone(),
                                                             generation: self.generation,
                                                             deadline,
                                                             buffers });

      waiting_for.insert(client_id.clone());
    }

    let timeout = || deadline - Instant::now() - Duration::from_micros(100);

    while !waiting_for.is_empty() && Instant::now() < deadline {
      if let Ok(event) = self.rx_cmd.recv_timeout(timeout()) {
        match event {
          | DeviceCommand::Register { client_id, tx_client } => {
            let _ = tx_client.send(DeviceClientCommand::Registered { device_id: self.id.clone(), });
            new_registrations.insert(client_id.clone(), Client { tx_cmd: tx_client });
          }
          | DeviceCommand::Unregister { client_id } => {
            if let Some(client) = self.clients.remove(&client_id) {
              let _ = client.tx_cmd
                            .send(DeviceClientCommand::Unregistered { device_id: self.id.clone(), });
            }
            new_registrations.remove(&client_id);
            waiting_for.remove(&client_id);
          }
          | DeviceCommand::FlipFinished { client_id, generation } =>
            if generation == self.generation {
              waiting_for.remove(&client_id);
            },
        }
      }
    }

    for id in waiting_for {
      if let Some(client) = self.clients.remove(&id) {
        let _ = client.tx_cmd
                      .send(DeviceClientCommand::Unregistered { device_id: self.id.clone(), });
      }
    }

    self.generation += 1;
  }
}

extern "C" fn callback(data: *mut c_void,
                       inputs: *const *const f32,
                       num_inputs: i32,
                       outputs: *mut *mut f32,
                       num_outputs: i32,
                       buffer_size: i32) {
  let device = unsafe { &mut *(data as *mut JuceAudioDevice) };
  device.generation += 1;

  let buffers = DeviceBuffers { inputs,
                                outputs,
                                num_inputs: num_inputs as usize,
                                num_outputs: num_outputs as usize,
                                buffer_size: buffer_size as usize,
                                generation: device.generation };

  device.handle_callback(buffers);
}

#[derive(Debug, Display, Clone, Copy)]
#[display(fmt = "self:?")]
pub enum JuceAudioDeviceType {
  CoreAudio,
  ASIO,
}

impl JuceAudioDeviceType {
  pub fn to_cstring(&self) -> CString {
    match self {
      | Self::CoreAudio => CString::new("CoreAudio").unwrap(),
      | Self::ASIO => CString::new("ASIO").unwrap(),
    }
  }
}

struct Client {
  tx_cmd: mpsc::Sender<DeviceClientCommand>,
}
