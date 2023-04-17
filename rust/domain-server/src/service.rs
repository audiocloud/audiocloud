use std::collections::HashMap;

use api::instance::control::{instance_play_control_key, instance_power_control_key, InstancePlayControl, InstancePowerControl};
use api::instance::driver::events::{instance_driver_events, InstanceDriverEvent};
use api::instance::driver::requests::{set_instance_parameters_request, SetInstanceParameter, SetInstanceParameterResponse};
use api::instance::spec::{instance_spec_key, InstanceSpec};
use api::media::buckets::{media_download_spec_key, media_upload_spec_key, media_upload_state_key};
use api::media::spec::{MediaDownloadSpec, MediaId, MediaUploadSpec};
use api::media::state::{media_download_state_key, MediaDownloadState, MediaUploadState};
use api::BucketKey;

use crate::nats::{EventStream, Nats, RequestStream, WatchStream};

#[derive(Clone)]
pub struct Service {
  pub nats: Nats,
}

pub type Result<T = ()> = anyhow::Result<T>;

impl Service {
  pub async fn list_instances(&self, filter: String) -> Result<HashMap<String, InstanceSpec>> {
    Ok(self.nats.instance_spec.scan(filter.as_str()).await?)
  }

  pub fn subscribe_to_instance_events(&self, instance_id: &str) -> EventStream<InstanceDriverEvent> {
    self.nats.subscribe_to_events(instance_driver_events(instance_id))
  }

  pub async fn publish_instance_driver_event(&self, instance_id: &str, event: InstanceDriverEvent) -> Result {
    self.nats.publish_event(instance_driver_events(instance_id), event).await
  }

  pub fn watch_instance_specs(&self, instance_id: &str) -> WatchStream<String, InstanceSpec> {
    self.nats.instance_spec.watch(instance_spec_key(&instance_id))
  }

  pub async fn set_instance_spec(&self, instance_id: &str, spec: InstanceSpec) -> Result {
    self.nats.instance_spec.put(instance_spec_key(&instance_id), spec).await?;

    Ok(())
  }

  pub fn watch_instance_power_control(&self, instance_id: &str) -> WatchStream<String, InstancePowerControl> {
    self.nats.instance_power_ctrl.watch(instance_power_control_key(&instance_id))
  }

  pub async fn set_instance_power_control(&self, instance_id: &str, power: InstancePowerControl) -> Result {
    self.nats
        .instance_power_ctrl
        .put(instance_power_control_key(&instance_id), power)
        .await?;

    Ok(())
  }

  pub async fn set_instance_play_control(&self, instance_id: &str, play: InstancePlayControl) -> Result {
    self.nats
        .instance_play_ctrl
        .put(instance_play_control_key(&instance_id), play)
        .await?;

    Ok(())
  }

  pub fn watch_instance_play_control(&self, instance_id: &str) -> WatchStream<String, InstancePlayControl> {
    self.nats.instance_play_ctrl.watch(instance_play_control_key(&instance_id))
  }

  pub async fn set_instance_parameters(&self,
                                       instance_id: &str,
                                       request: Vec<SetInstanceParameter>)
                                       -> Result<SetInstanceParameterResponse> {
    self.nats.request(set_instance_parameters_request(&instance_id), request).await
  }

  pub fn serve_instance_parameters(&self, instance_id: &str) -> RequestStream<Vec<SetInstanceParameter>, SetInstanceParameterResponse> {
    self.nats.serve_requests(set_instance_parameters_request(instance_id))
  }

  pub fn watch_all_media_download_specs(&self) -> WatchStream<MediaId, MediaDownloadSpec> {
    self.nats.media_download_spec.watch(BucketKey::all())
  }

  pub async fn set_media_download_spec(&self, media_id: &MediaId, spec: MediaDownloadSpec) -> Result {
    self.nats.media_download_spec.put(media_download_spec_key(&media_id), spec).await?;

    Ok(())
  }

  pub fn watch_media_download_state(&self, media_id: &MediaId) -> WatchStream<MediaId, MediaDownloadState> {
    self.nats.media_download_state.watch(media_download_state_key(&media_id))
  }

  pub async fn set_media_download_state(&self, media_id: &MediaId, state: MediaDownloadState) -> Result {
    self.nats
        .media_download_state
        .put(media_download_state_key(&media_id), state)
        .await?;

    Ok(())
  }

  pub fn watch_all_media_upload_specs(&self) -> WatchStream<MediaId, MediaUploadSpec> {
    self.nats.media_upload_spec.watch(BucketKey::all())
  }

  pub async fn set_media_upload_spec(&self, media_id: &MediaId, spec: MediaUploadSpec) -> Result {
    self.nats.media_upload_spec.put(media_upload_spec_key(&media_id), spec).await?;

    Ok(())
  }

  pub fn watch_media_upload_state(&self, media_id: &MediaId) -> WatchStream<MediaId, MediaUploadState> {
    self.nats.media_upload_state.watch(media_upload_state_key(&media_id))
  }

  pub async fn set_media_upload_state(&self, media_id: &MediaId, state: MediaUploadState) -> Result {
    self.nats.media_upload_state.put(media_upload_state_key(&media_id), state).await?;

    Ok(())
  }
}
