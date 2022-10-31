use reqwest::{Client, Response, Url};
use serde::de::DeserializeOwned;

use audiocloud_api::cloud::domains::{EngineConfig, InstanceDriverConfig};
use audiocloud_api::domain::DomainError;
use audiocloud_api::instance_driver::InstanceDriverError;
use audiocloud_api::{EngineId, FixedInstanceId};

use crate::create_client;

type Result<T = ()> = std::result::Result<T, DomainError>;

#[derive(Clone)]
pub struct DomainServerClient {
    client:   Client,
    base_url: Url,
}

impl DomainServerClient {
    pub fn new(base_url: Url) -> Result<Self> {
        let client = create_client().map_err(Self::rpc_err)?;

        Ok(Self { base_url, client })
    }

    pub async fn register_instance_driver(&self,
                                          instance_id: &FixedInstanceId,
                                          config: &InstanceDriverConfig)
                                          -> Result<InstanceDriverConfig> {
        let url = self.url(format!("/v1/instances/{manufacturer}/{name}/{instance}/register",
                                   manufacturer = &instance_id.manufacturer,
                                   name = &instance_id.name,
                                   instance = &instance_id.instance))?;

        let response = self.client.post(url).json(config).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn register_audio_engine(&self, engine_id: &EngineId, config: &EngineConfig) -> Result<EngineConfig> {
        let url = self.url(format!("/v1/engines/{engine_id}/register"))?;

        let response = self.client.post(url).json(config).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    fn url(&self, path: impl AsRef<str>) -> Result<Url> {
        let url = self.base_url.join(path.as_ref()).map_err(Self::rpc_err)?;

        Ok(url)
    }

    async fn respond<T: DeserializeOwned>(response: Response) -> Result<T> {
        if response.status().is_success() {
            response.json().await.map_err(Self::rpc_err)
        } else {
            Err(response.json().await.map_err(Self::rpc_err)?)
        }
    }

    fn rpc_err(e: impl ToString) -> DomainError {
        DomainError::RPC { error: e.to_string() }
    }
}
