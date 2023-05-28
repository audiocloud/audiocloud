use async_stream::stream;
use consulrs::api::catalog::requests::ListServicesRequest;
use consulrs::client::ConsulClient;
use futures::Stream;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct ServiceRegistry {
  client: ConsulClient,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceKey {
  Instances,
  Tasks,
  Security,
  InstanceDriver { host: String },
  AudioEngine { host: String },
}

impl ToString for ServiceKey {
  fn to_string(&self) -> String {
    match self {
      | ServiceKey::Instances => "instances".to_owned(),
      | ServiceKey::Tasks => "tasks".to_owned(),
      | ServiceKey::Security => "security".to_owned(),
      | ServiceKey::InstanceDriver { host } => {
        format!("instance-driver-{host}")
      }
      | ServiceKey::AudioEngine { host } => {
        format!("audio-engine-{host}")
      }
    }
  }
}

impl ServiceKey {
  pub fn to_url(&self, protocol: &str, datacenter: &str) -> String {
    format!("{protocol}://{}.service.{datacenter}.consul", self.to_string())
  }

  pub fn http_url(&self, datacenter: &str) -> String {
    self.to_url("http", datacenter)
  }
}

impl ServiceRegistry {
  pub fn subscribe_service_endpoints(&self, service: &str) -> impl Stream<Item = Vec<String>> {
    stream! {
      let mut last_sent = vec![];
      let mut first = true;
      loop {
        if !first {
          sleep(Duration::from_secs(1)).await;
        }
        first = false;

        let mut all_addresses = vec![];
        let Ok(request) = ListServicesRequest::builder().features(None) else { continue };
        let Ok(response) = consulrs::catalog::services(&self.client, Some(request)).await else { continue };

        for (id, endpoints) in response {
          if id != service {
            continue;
          }
          all_addresses.extend(endpoints.into_iter());
        }

        if &all_addresses != &last_sent {
          last_sent = all_addresses;
          yield last_sent.clone();
        }
      }
    }
  }
}
