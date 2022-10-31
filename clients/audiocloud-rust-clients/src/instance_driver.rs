use reqwest::{Client, Response, Url};
use serde::de::DeserializeOwned;

use audiocloud_api::audio_engine::EngineError;
use audiocloud_api::instance_driver::{
    InstanceCommandAccepted, InstanceDriverCommand, InstanceDriverError, InstanceParametersUpdated, InstanceWithStatusList,
    SetInstanceParameters,
};
use audiocloud_api::FixedInstanceId;

use crate::create_client;

type Result<T = ()> = std::result::Result<T, InstanceDriverError>;

#[derive(Clone)]
pub struct InstanceDriverClient {
    client:   Client,
    base_url: Url,
}

impl InstanceDriverClient {
    pub fn new(base_url: Url) -> Result<Self> {
        let client = create_client().map_err(Self::rpc_err)?;

        Ok(Self { base_url, client })
    }

    pub async fn get_instances(&self) -> Result<InstanceWithStatusList> {
        let url = self.url("/v1/instances")?;

        let response = self.client.get(url).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn set_parameters(&self, instance_id: &FixedInstanceId, params: &SetInstanceParameters) -> Result<InstanceParametersUpdated> {
        let url = self.url(format!("/v1/instances/{manufacturer}/{name}/{instance}/parameters",
                                   manufacturer = &instance_id.manufacturer,
                                   name = &instance_id.name,
                                   instance = &instance_id.instance))?;

        let response = self.client.put(url).json(params).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn send_command(&self, instance_id: &FixedInstanceId, cmd: &InstanceDriverCommand) -> Result<InstanceCommandAccepted> {
        let url = self.url(format!("/v1/instances/{manufacturer}/{name}/{instance}/command",
                                   manufacturer = &instance_id.manufacturer,
                                   name = &instance_id.name,
                                   instance = &instance_id.instance))?;

        let response = self.client.post(url).json(cmd).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn is_healthy(&self) -> Result {
        let url = self.url("/v1/health")?;

        let response = self.client.get(url).send().await.map_err(Self::rpc_err)?;

        let _: serde_json::Value = Self::respond(response).await?;

        Ok(())
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

    fn rpc_err(e: impl ToString) -> InstanceDriverError {
        InstanceDriverError::RPC { error: e.to_string() }
    }
}
