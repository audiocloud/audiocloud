use std::collections::HashMap;
use std::time::{Duration, Instant};

use tokio::sync::mpsc;
use tokio::{select, spawn};

use crate::audio_device::{AudioDevice, AudioDeviceInfo, DeviceClientCommand, DeviceCommand};
use crate::buffer::DeviceBuffers;

pub fn new_simulator_device(device_id: String, info: AudioDeviceInfo) -> AudioDevice {
  let (tx_cmd, rx_cmd) = mpsc::channel(0x100);
  let cycle_time = Duration::from_secs_f64(info.buffer_size as f64 / info.sample_rate as f64);

  spawn(run_simulator_device(device_id, cycle_time, info, rx_cmd));

  AudioDevice { tx_cmd, info }
}

async fn run_simulator_device(device_id: String, cycle_time: Duration, info: AudioDeviceInfo, mut rx_cmd: mpsc::Receiver<DeviceCommand>) {
  let mut clients: HashMap<String, mpsc::Sender<DeviceClientCommand>> = HashMap::new();
  let mut new_clients = HashMap::new();
  let mut waiting_for = HashMap::new();

  let mut next_time = Instant::now() + cycle_time;

  let mut dev_buffers = DeviceBuffers::allocate_and_forget(info.num_inputs, info.num_outputs, info.buffer_size as usize);
  loop {
    select! {
      Some(cmd) = rx_cmd.recv() => {
        match cmd {
          DeviceCommand::Register { client_id, tx_client } => {
            new_clients.insert(client_id, tx_client);
          }
          DeviceCommand::Unregister { client_id } => {
            waiting_for.remove(&client_id);
            clients.remove(&client_id);
          }
          DeviceCommand::FlipFinished { client_id, generation } => {
            if waiting_for.get(&client_id) == Some(&generation) {
              waiting_for.remove(&client_id);
            } else {
              // spurious flip finished
            }
          },
          DeviceCommand::Terminate => {
            for device in clients.values_mut() {
              let _  = device.try_send(DeviceClientCommand::Unregistered {device_id: device_id.clone()});
            }

            break;
          }
        }
      }
      _ = tokio::time::sleep_until(tokio::time::Instant::from_std(next_time)) => {
        next_time = next_time + cycle_time;

        // buffer flick page time
        dev_buffers.generation += 1;

        for (id, _) in waiting_for.drain() {
          clients.remove(&id);
        }

        clients.extend(new_clients.drain());

        for (id, client) in clients.iter_mut() {
          waiting_for.insert(id.clone(), dev_buffers.generation);
          client.try_send(DeviceClientCommand::Flip {
            device_id: device_id.clone(),
            generation: dev_buffers.generation,
            deadline: next_time,
            buffers: dev_buffers.clone(),
          });
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use crate::audio_device::AudioDevices;

  use super::*;

  #[tokio::test]
  async fn test_create_delete() {
    let simulator = new_simulator_device("test".to_string(), AudioDeviceInfo { latency:     0,
                                                                               buffer_size: 512,
                                                                               sample_rate: 192_000,
                                                                               num_inputs:  2,
                                                                               num_outputs: 2, });

    let mut devices = AudioDevices::default();
    devices.add_device("test".to_string(), simulator);

    let _ = tokio::time::sleep(Duration::from_secs(1));

    devices.terminate_device("test");
  }
}
