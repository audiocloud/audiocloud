use reqwest::{Client, ClientBuilder, Response, Url};
use serde::de::DeserializeOwned;

use audiocloud_api::audio_engine::{
    EngineError, InstancesUpdated, MediaUpdated, SetInstances, SetMedia, TaskDeleted, TaskModified, TaskPlayStopped, TaskPlaying,
    TaskRenderCancelled, TaskRendering, TaskReplaced, TaskSought,
};
use audiocloud_api::{
    AppId, ModifyTaskSpec, RequestCancelRender, RequestPlay, RequestRender, RequestSeek, RequestStopPlay, TaskId, TaskSpec,
};

use crate::create_client;

#[derive(Clone)]
pub struct AudioEngineClient {
    client:   Client,
    base_url: Url,
}

impl AudioEngineClient {
    pub fn new(base_url: Url) -> Result<Self, EngineError> {
        let client = create_client().map_err(Self::rpc_err)?;

        Ok(Self { client, base_url })
    }

    pub async fn set_instances(&self, instances: &SetInstances) -> Result<InstancesUpdated, EngineError> {
        let url = self.url("/v1/instances")?;

        let response = self.client.put(url).json(instances).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn set_media(&self, media: &SetMedia) -> Result<MediaUpdated, EngineError> {
        let url = self.url("/v1/media")?;

        let response = self.client.put(url).json(media).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn delete_task(&self, app_id: &AppId, task_id: &TaskId) -> Result<TaskDeleted, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}"))?;

        let response = self.client.delete(url).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn modify_task_spec(&self, app_id: &AppId, task_id: &TaskId, spec: &ModifyTaskSpec) -> Result<TaskModified, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}"))?;

        let response = self.client.patch(url).json(spec).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn set_task_spec(&self, app_id: &AppId, task_id: &TaskId, spec: &TaskSpec) -> Result<TaskReplaced, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}"))?;

        let response = self.client.put(url).json(spec).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn render(&self, app_id: &AppId, task_id: &TaskId, render: &RequestRender) -> Result<TaskRendering, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}/render"))?;

        let response = self.client.post(url).json(render).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn cancel_render(&self,
                               app_id: &AppId,
                               task_id: &TaskId,
                               cancel: &RequestCancelRender)
                               -> Result<TaskRenderCancelled, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}/transport/cancel"))?;

        let response = self.client.post(url).json(cancel).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn play_task(&self, app_id: &AppId, task_id: &TaskId, play: &RequestPlay) -> Result<TaskPlaying, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}/transport/play"))?;

        let response = self.client.post(url).json(play).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn seek_play(&self, app_id: &AppId, task_id: &TaskId, seek: &RequestSeek) -> Result<TaskSought, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}/transport/seek"))?;

        let response = self.client.post(url).json(seek).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    pub async fn stop_playing(&self, app_id: &AppId, task_id: &TaskId, stop: &RequestStopPlay) -> Result<TaskPlayStopped, EngineError> {
        let url = self.url(format!("/v1/tasks/{app_id}/{task_id}/transport/stop"))?;

        let response = self.client.post(url).json(stop).send().await.map_err(Self::rpc_err)?;

        Self::respond(response).await
    }

    async fn respond<T: DeserializeOwned>(response: Response) -> Result<T, EngineError> {
        if response.status().is_success() {
            response.json().await.map_err(Self::rpc_err)
        } else {
            Err(response.json().await.map_err(Self::rpc_err)?)
        }
    }

    fn url<P: AsRef<str>>(&self, path: P) -> Result<Url, EngineError> {
        let path = path.as_ref();
        self.base_url.join(path).map_err(Self::rpc_err)
    }

    fn rpc_err(e: impl ToString) -> EngineError {
        EngineError::RPC { error: e.to_string() }
    }
}
