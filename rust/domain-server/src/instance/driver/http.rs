use std::convert::identity;
use std::str::FromStr;
use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;
use maplit::hashmap;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{blocking::Body, Url};
use tracing::warn;

use api::instance::driver::config::http::HttpMethod;
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::{driver::config::http::HttpDriverConfig, IdAndChannel};

use crate::instance::Result;

use super::{
  scripting::{Script, ScriptingEngine},
  Driver,
};

pub struct HttpDriver {
  pub client:           Arc<reqwest::blocking::Client>,
  pub config:           HttpDriverConfig,
  pub instance_id:      String,
  pub outgoing:         HashMap<IdAndChannel, f64>,
  pub scripting:        ScriptingEngine,
  pub parameter_paths:  HashMap<String, Script>,
  pub parameter_bodies: HashMap<String, Script>,
}

impl Driver for HttpDriver {
  type Config = HttpDriverConfig;
  type Shared = Arc<reqwest::blocking::Client>;

  fn create_shared() -> Result<Self::Shared> {
    Ok(Arc::new(reqwest::blocking::Client::new()))
  }

  fn new(instance_id: &str, shared: &mut Self::Shared, config: Self::Config) -> Result<Self> {
    let mut scripting = ScriptingEngine::new()?;

    let parameter_paths = config.parameters
                                .iter()
                                .map(|(k, v)| Ok((k.clone(), scripting.compile(&v.path)?)))
                                .collect::<Result<_>>()?;

    let parameter_bodies = config.parameters
                                 .iter()
                                 .map(|(k, v)| match v.body.as_ref() {
                                   | Some(body) => Ok(Some((k.clone(), scripting.compile(&body)?))),
                                   | None => Ok(None),
                                 })
                                 .map(Result::transpose)
                                 .filter_map(identity)
                                 .collect::<Result<_>>()?;

    let instance_id = instance_id.to_string();
    let outgoing = HashMap::new();
    let client = shared.clone();

    Ok(Self { client,
              config,
              instance_id,
              outgoing,
              scripting,
              parameter_paths,
              parameter_bodies })
  }

  fn set_parameter(&mut self, shared: &mut Self::Shared, parameter: &str, channel: usize, value: f64) -> Result<()> {
    self.outgoing.insert(IdAndChannel { id: parameter.to_string(),
                                        channel },
                         value);
    Ok(())
  }

  fn poll(&mut self, shared: &mut Self::Shared, deadline: std::time::Instant) -> Result<Vec<InstanceDriverEvent>> {
    let mut events = Vec::new();

    for (id_and_channel, value) in self.outgoing.drain() {
      let parameter_id = &id_and_channel.id;
      let channel = id_and_channel.channel;

      let Some(parameter_config) = self.config.parameters.get(parameter_id) else { continue; };
      let Some(parameter_path_script) = self.parameter_paths.get(parameter_id) else { continue; };
      let body_script = self.parameter_bodies.get(parameter_id);

      let env = || {
        hashmap! {
          "value".to_owned() => value.into(),
          "channel".to_owned() => id_and_channel.channel.into(),
          "instanceId".to_owned() => self.instance_id.clone().into(),
          "baseUrl".to_owned() => self.config.base_url.clone().into(),
        }
      };

      let url = self.scripting.execute_with_env(parameter_path_script, env().into_iter());
      let url = self.scripting.convert_to_string(url);

      let url = match Url::parse(&url) {
        | Ok(url) => url,
        | Err(err) => {
          warn!(parameter_id, channel, ?err, url, "Failed to parse url: {err}");
          continue;
        }
      };

      let mut request = reqwest::blocking::Request::new(match parameter_config.method {
                                                          | HttpMethod::GET => reqwest::Method::GET,
                                                          | HttpMethod::PUT => reqwest::Method::PUT,
                                                          | HttpMethod::POST => reqwest::Method::POST,
                                                        },
                                                        url);

      if let Some(body) = body_script {
        let body = self.scripting.execute_with_env(body, env().into_iter());
        let body = self.scripting.convert_to_string(body);

        request.body_mut().replace(Body::from(body));
      }

      for (header, value) in &parameter_config.headers {
        let Ok(value) = HeaderValue::from_str(value) else { continue; };
        request.headers_mut().insert(HeaderName::from_str(header.as_str())?, value);
      }

      if let Err(err) = self.client.execute(request) {
        warn!(parameter_id, ?err, "Failed to execute request: {err}");
      }
    }

    Ok(events)
  }

  fn can_continue(&self) -> bool {
    true
  }
}
