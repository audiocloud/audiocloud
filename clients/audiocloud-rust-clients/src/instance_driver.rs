use reqwest::{Client, Response, Url};
use serde::de::DeserializeOwned;


use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceParametersUpdated, InstanceWithStatusList,
    SetInstanceParameters,
};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId};

use crate::create_client;

type Result<T = ()> = std::result::Result<T, InstanceDriverError>;

#[derive(Clone)]
pub struct InstanceDriverClient {
    client:   Client,
    base_url: Option<Url>,
}

impl InstanceDriverClient {
    pub fn new(base_url: impl Into<Option<Url>>) -> Result<Self> {
        let client = create_client().map_err(Self::rpc_err)?;

        Ok(Self { base_url: { base_url.into() },
                  client:   { client }, })
    }

    pub fn set_url(&mut self, base_url: impl Into<Option<Url>>) {
        self.base_url = base_url.into();
    }

    pub fn is_url_set(&self) -> bool {
        self.base_url.is_some()
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

        let response = self.client.patch(url).json(params).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn set_desired_play_state(&self,
                                        instance_id: &FixedInstanceId,
                                        state: &DesiredInstancePlayState)
                                        -> Result<DesiredInstancePlayStateUpdated> {
        let url = self.url(format!("/v1/instances/{manufacturer}/{name}/{instance}/play_state",
                                   manufacturer = &instance_id.manufacturer,
                                   name = &instance_id.name,
                                   instance = &instance_id.instance))?;

        let response = self.client.put(url).json(state).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn is_healthy(&self) -> Result {
        let url = self.url("/v1/health")?;

        let response = self.client.get(url).send().await.map_err(Self::rpc_err)?;

        let _: serde_json::Value = Self::respond(response).await?;

        Ok(())
    }

    fn url(&self, path: impl AsRef<str>) -> Result<Url> {
        match self.base_url.as_ref() {
            Some(base_url) => base_url.join(path.as_ref()).map_err(Self::rpc_err),
            None => Err(InstanceDriverError::RPC { error: format!("No base URL has been set on this client"), }),
        }
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
