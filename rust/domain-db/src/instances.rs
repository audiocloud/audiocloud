use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use api_proto::{InstanceMediaState, InstancePowerState, InstanceSpec, Timestamped};

use crate::{Db, Result};

#[derive(Serialize, Deserialize)]
pub struct DbInstanceData {
  pub id:          Thing,
  pub spec:        InstanceSpec,
  pub power_state: InstancePowerState,
  pub media_state: InstanceMediaState,
  pub claimed_by:  Option<Timestamped<String>>,
}

impl Db {
  pub async fn list_instances(&self) -> Result<Vec<DbInstanceData>> {
    Ok(self.surreal.select("instance").await?)
  }

  pub async fn get_instance_by_id(&self, id: &Thing) -> Result<Option<DbInstanceData>> {
    Ok(self.surreal.select(("instance", "id")).await?)
  }

  pub async fn set_instance_spec(&self, id: &str, spec: &InstanceSpec) -> Result<Option<DbInstanceData>> {
    Ok(self.surreal.update(("instance", id)).merge(json!({"spec": spec})).await?)
  }

  pub async fn set_instance_power_state(&self, id: &str, power_state: &InstancePowerState) -> Result<Option<DbInstanceData>> {
    Ok(self.surreal
           .update(("instance", id))
           .merge(json!({"power_state": power_state}))
           .await?)
  }

  pub async fn set_instance_media_state(&self, id: &str, media_state: &InstanceMediaState) -> Result<Option<DbInstanceData>> {
    Ok(self.surreal
           .update(("instance", id))
           .merge(json!({"media_state": media_state}))
           .await?)
  }

  pub async fn set_instance_claim(&self, id: &str, claimed_by: Option<Timestamped<String>>) -> Result<Option<DbInstanceData>> {
    Ok(self.surreal
           .update(("instance", id))
           .merge(json!({"claimed_by": claimed_by}))
           .await?)
  }
}
