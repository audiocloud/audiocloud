use std::str::FromStr;

use futures::StreamExt;
use lazy_static::lazy_static;
use reqwest::{Body, Url};
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::json;
use tracing::warn;

use api::instance::driver::config::http::HttpDriverConfig;
use api::instance::driver::config::http::HttpMethod;
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;
use api::instance::driver::requests::SetInstanceParameterResponse::EncodingError;

use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::Result;

use super::scripting::ScriptingEngine;

lazy_static! {
  static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn run_http_driver(instance_id: String,
                             config: HttpDriverConfig,
                             rx_cmd: flume::Receiver<InstanceDriverCommand>,
                             tx_evt: flume::Sender<InstanceDriverEvent>,
                             scripting_engine: ScriptingEngine)
                             -> Result {
  use SetInstanceParameterResponse::*;

  let _ = tx_evt.send_async(InstanceDriverEvent::Connected { connected: true }).await;

  while let Ok(cmd) = rx_cmd.recv_async().await {
    match cmd {
      | InstanceDriverCommand::SetParameters(parameters, tx_done) => {
        let base_url = &config.base_url;
        let mut error = None;

        for p in parameters.changes {
          let Some(parameter_config) = config.parameters.get(&p.parameter) else { continue; };
          let parameter_id = &p.parameter;
          let channel = p.channel;

          let env = || {
            json!({
              "value": p.value,
              "channel": p.channel,
              "instanceId": instance_id.clone(),
              "baseUrl": base_url.clone(),
            })
          };

          let url = scripting_engine.execute(parameter_config.url.clone(), env()).await.to_string();

          let url = match Url::parse(&url) {
            | Ok(url) => url,
            | Err(err) => {
              warn!(parameter_id, channel, ?err, url, "Failed to parse url: {err}");
              continue;
            }
          };

          let mut request = reqwest::Request::new(match parameter_config.method {
                                                    | HttpMethod::GET => reqwest::Method::GET,
                                                    | HttpMethod::PUT => reqwest::Method::PUT,
                                                    | HttpMethod::POST => reqwest::Method::POST,
                                                  },
                                                  url);

          if let Some(body) = parameter_config.body.as_ref() {
            let body = scripting_engine.execute(body.clone(), env()).await.to_string();
            request.body_mut().replace(Body::from(body));
          }

          for (header, value) in &parameter_config.headers {
            let Ok(value) = HeaderValue::from_str(value) else { error = Some(EncodingError); continue };
            let Ok(name) = HeaderName::from_str(header.as_str()) else { error = Some(EncodingError); continue };

            request.headers_mut().insert(name, value);
          }

          if let Err(err) = HTTP_CLIENT.execute(request).await {
            warn!(parameter_id, ?err, "Failed to execute request: {err}");
          }
        }

        let _ = tx_done.send_async(error.unwrap_or(Success)).await;
      }
      | InstanceDriverCommand::Terminate => break,
    }
  }

  Ok(())
}
