use std::str::FromStr;

use lazy_static::lazy_static;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Body, Url};
use serde_json::json;
use tokio::sync::mpsc;
use tracing::warn;

use api::instance::driver::config::http::HttpDriverConfig;
use api::instance::driver::config::http::HttpMethod;
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;

use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::Result;

use super::scripting::ScriptingEngine;

lazy_static! {
  static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn run_http_driver(instance_id: String,
                             config: HttpDriverConfig,
                             mut rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                             tx_evt: mpsc::Sender<InstanceDriverEvent>,
                             scripting_engine: ScriptingEngine)
                             -> Result {
  while let Some(cmd) = rx_cmd.recv().await {
    match cmd {
      | InstanceDriverCommand::SetParameters(parameters, tx_done) => {
        let base_url = &config.base_url;

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
            let Ok(value) = HeaderValue::from_str(value) else { continue; };
            request.headers_mut().insert(HeaderName::from_str(header.as_str())?, value);
          }

          if let Err(err) = HTTP_CLIENT.execute(request).await {
            warn!(parameter_id, ?err, "Failed to execute request: {err}");
          }
        }

        let _ = tx_done.send(SetInstanceParameterResponse::Success);
      }
      | InstanceDriverCommand::Terminate => return Ok(()),
    }
  }

  Ok(())
}
