use std::collections::HashMap;

use anyhow::{anyhow, bail};
use async_nats::jetstream::{kv, Context};
use bytes::Bytes;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::{select, spawn};
use tokio_stream::StreamMap;
use tracing::warn;

use api::driver::{buckets, control_keys, status_keys, InstanceDriverConfig, InstanceDriverEvent, SetInstanceParameterRequest};

use crate::instance_driver::run::{run_driver_server, InstanceDriverCommand};
use crate::instance_driver::usb_hid::UsbHidDriver;
use crate::nats_utils::{ExtractValue, InstanceParameterId};

use super::Result;

enum Message {
  Terminated {
    instance_id: String,
    next:        Option<InstanceDriverConfig>,
  },
}

type Handle = (JoinHandle<Result>, mpsc::Sender<InstanceDriverCommand>, InstanceDriverConfig);

pub async fn server(driver_id: String, ctx: Context) -> Result {
  let config_bucket = ctx.get_key_value(buckets::instance_specs(&driver_id))
                         .await
                         .map_err(|e| anyhow!("Failed to get bucket driver config bucket").context(e))?;

  let mut config_watch = config_bucket.watch_all()
                                      .await
                                      .map_err(|e| anyhow!("Failed to watch bucket").context(e))?;

  let state_bucket = ctx.get_key_value(buckets::INSTANCE_STATE)
                        .await
                        .map_err(|e| anyhow!("Failed to get bucket driver state bucket").context(e))?;

  let control_bucket = ctx.get_key_value(buckets::INSTANCE_CONTROL)
                          .await
                          .map_err(|e| anyhow!("Failed to get bucket driver control bucket").context(e))?;

  let mut servers = HashMap::<String, Handle>::new();
  let mut watches = StreamMap::<String, kv::Watch>::new();

  let (tx_msg, mut rx_msg) = mpsc::channel(0x02);
  let (tx_evt, mut rx_evt) = mpsc::channel(0xff);

  loop {
    select! {
      Some(Ok(kv::Entry { operation, value, key: instance_id, .. })) = config_watch.next() => {
        match operation {
          | kv::Operation::Put  => {
            let config = match serde_json::from_slice(value.as_slice()) {
              Ok(value) => value,
              Err(err) => {
                warn!(%err, instance_id, "Failed to parse instance config");
                continue;
              }
            };

            terminate_and_requeue(instance_id.clone(), &mut servers, config, tx_msg.clone()).await;
          }
          | kv::Operation::Delete | kv::Operation::Purge => {
            terminate_and_requeue(instance_id.clone(), &mut servers, None, tx_msg.clone()).await;
          },
        }
      },
      Some((instance_id, Ok(entry))) = watches.next() => {
        if let Ok((InstanceParameterId {parameter, channel, ..}, value)) = entry.extract::<InstanceParameterId, f64>() {
          if let Some((_, tx_cmd, _)) = servers.get(&instance_id) {
            let parameter = parameter.to_string();
            let _ = tx_cmd.send(InstanceDriverCommand::SetParameters(SetInstanceParameterRequest {
                                                                        parameter,
                                                                        channel,
                                                                        value
                                                                      })).await;
          }
        }
      },
      Some(msg) = rx_msg.recv() => match msg {
        | Message::Terminated { instance_id, next } => {
          servers.remove(&instance_id);
          watches.remove(&instance_id);

          if let Some(next_cfg) = next {
            if let Ok(handle) = new_driver_server_from_config(instance_id.clone(), next_cfg, tx_evt.clone()).await {
              let watch = control_bucket.watch(&control_keys::instance_desired_parameter_value_wildcard(&instance_id))
                                        .await
                                        .map_err(|e| anyhow!("Failed to watch instance control").context(e))?;
              watches.insert(instance_id.clone(), watch);
              servers.insert(instance_id, handle);
            }
          }
        }
      },
      Some(event) = rx_evt.recv() => {
        match event {
          | InstanceDriverEvent::Report(report) => {
            let Ok(value) = serde_json::to_string(&report.value) else { continue };
            let value = Bytes::from(value);

            // TODO: update report key
            // state_bucket.put(status_keys::instance_report_value(&report.instance_id, &report.report_id, report.channel), value)
            //             .await
            //             .map_err(|e| anyhow!("Failed to put report update").context(e))?;
          }
        }
      },
      else => break,
    }
  }

  Ok(())
}

async fn terminate_and_requeue<'a>(instance_id: String,
                                   instance_server: &mut HashMap<String, Handle>,
                                   next: Option<InstanceDriverConfig>,
                                   tx_msg: mpsc::Sender<Message>) {
  match instance_server.remove(&instance_id) {
    | Some((handle, tx, _)) => {
      spawn(async move {
        let _ = tx.send(InstanceDriverCommand::Terminate).await;
        let _ = handle.await;
        let _ = tx_msg.send(Message::Terminated { instance_id, next }).await;
      });
    }
    | None =>
      if let Some(next) = next {
        let _ = tx_msg.send(Message::Terminated { instance_id,
                                                  next: Some(next) })
                      .await;
      },
  }
}

async fn new_driver_server_from_config(instance_id: String,
                                       cfg: InstanceDriverConfig,
                                       tx_evt: mpsc::Sender<InstanceDriverEvent>)
                                       -> Result<Handle> {
  let (tx_cmd, rx_cmd) = mpsc::channel(0xff);

  match cfg {
    | InstanceDriverConfig::USBHID(cfg) => {
      let handle = spawn(run_driver_server::<UsbHidDriver>(instance_id, cfg.clone(), rx_cmd, tx_evt));
      Ok((handle, tx_cmd, InstanceDriverConfig::USBHID(cfg)))
    }
    | InstanceDriverConfig::Serial(_) => {
      bail!("Serial driver not implemented yet")
    }
    | InstanceDriverConfig::OSC(_) => {
      bail!("OSC driver not implemented yet")
    }
  }
}
