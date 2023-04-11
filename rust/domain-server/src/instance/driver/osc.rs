use std::io::Write;

use anyhow::bail;
use byteorder::{WriteBytesExt, LE};
use rosc::{encoder, OscBundle, OscMessage, OscPacket, OscTime, OscType};
use serde_json::json;
use tokio::sync::mpsc;
use tokio::{net, select};

use api::instance::driver::config::osc::OscDriverConfig;
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::{SetInstanceParameter, SetInstanceParameterResponse};

use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::driver::scripting::ScriptingEngine;

use super::Result;

const IMMEDIATELY: OscTime = OscTime { seconds:    0,
                                       fractional: 0, };

pub async fn run_osc_driver(instance_id: String,
                            config: OscDriverConfig,
                            rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                            tx_evt: mpsc::Sender<InstanceDriverEvent>,
                            scripting: ScriptingEngine)
                            -> Result {
  if config.use_tcp {
    run_tcp_osc_driver(instance_id, config, rx_cmd, tx_evt, scripting).await
  } else {
    run_udp_osc_driver(instance_id, config, rx_cmd, tx_evt, scripting).await
  }
}

async fn run_tcp_osc_driver(instance_id: String,
                            config: OscDriverConfig,
                            mut rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                            tx_evt: mpsc::Sender<InstanceDriverEvent>,
                            scripting: ScriptingEngine)
                            -> Result {
  use tokio::io::{AsyncReadExt, AsyncWriteExt};

  let mut tcp_stream = net::TcpStream::connect((config.host.clone(), config.port)).await?;
  let (mut tcp_rx, mut tcp_tx) = tcp_stream.split();

  loop {
    let mut buf = [0u8; 1024];
    select! {
      _ = tcp_rx.read(&mut buf[..]) => {
        // just ignore the data
      },
      Some(cmd) = rx_cmd.recv() => {
        match cmd {
          | InstanceDriverCommand::SetParameters(req, complete) => {
            let Ok(serialized) = serialize_changed_to_bundle(&config, req.changes, &scripting).await else {
              let _ = complete.send(SetInstanceParameterResponse::EncodingError);
              continue;
            };

            if let Err(err) = tcp_tx.write_all(&serialized[..]).await {
              let _ = complete.send(SetInstanceParameterResponse::ConnectionError);
              bail!("failed to send OSC bundle: {err}");
            }

            let _ = complete.send(SetInstanceParameterResponse::Success);
          },
          | InstanceDriverCommand::Terminate => {
            break;
          },
        }
      },
      else => break
    }
  }

  Ok(())
}

async fn serialize_changed_to_bundle(config: &OscDriverConfig,
                                     changes: Vec<SetInstanceParameter>,
                                     scripting: &ScriptingEngine)
                                     -> Result<Vec<u8>> {
  let timetag = IMMEDIATELY;
  let mut content = vec![];

  for change in changes {
    let Some(parameters) = config.parameters.get(&change.parameter) else { continue; };
    let Some(parameter) = parameters.get(change.channel) else { continue; };

    let value = match parameter.transform.as_ref() {
      | None => change.value,
      | Some(transform) => scripting.execute(transform.clone(),
                                             json!({"value": change.value, "parameter": &change.parameter, "channel": change.channel}))
                                    .await
                                    .as_f64()
                                    .unwrap_or(change.value),
    };

    let addr = if parameter.address.starts_with("/") {
      parameter.address.clone()
    } else {
      scripting.execute(parameter.address.clone(),
                        json!({"value": value, "parameter": &change.parameter, "channel": change.channel}))
               .await
               .to_string()
    };

    content.push(OscPacket::Message(OscMessage { addr,
                                                 args: vec![OscType::Float(value as f32)] }));
  }

  let serialized = encoder::encode(&OscPacket::Bundle(OscBundle { timetag, content }))?;

  let mut rv = vec![];

  rv.write_u16::<LE>(serialized.len() as u16)?;
  rv.write_all(&serialized[..])?;

  Ok(rv)
}

async fn run_udp_osc_driver(instance_id: String,
                            config: OscDriverConfig,
                            rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                            tx_evt: mpsc::Sender<InstanceDriverEvent>,
                            scripting: ScriptingEngine)
                            -> Result {
  Ok(())
}
