use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use api::instance::spec::InstanceSpec;
use api::instance::{InstancePlayState, InstancePowerState};

use super::{Db, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceData {
  pub id:          Thing,
  pub spec:        InstanceSpec,
  #[serde(default)]
  pub power_state: Option<InstancePowerState>,
  #[serde(default)]
  pub play_state:  Option<InstancePlayState>,
  pub revision:    u64,
}

impl Db {
  pub async fn get_instances(&self, limit_to_host_id: Option<String>) -> Result<Vec<InstanceData>> {
    if let Some(host_id) = limit_to_host_id {
      Ok(self.db
             .query("SELECT * from instance WHERE spec.host = $host")
             .bind(("host", host_id))
             .await?
             .take(0)?)
    } else {
      Ok(self.db.select("instance").await?)
    }
  }

  pub async fn get_instance_by_id(&self, id: &str) -> Result<Option<InstanceData>> {
    Ok(self.db.select(("instance", id)).await?)
  }

  pub async fn register_instance(&self, id: &str, spec: InstanceSpec) -> Result<InstanceData> {
    Ok(self.db
           .create(("instance", id))
           .content(json!({
                      "spec": spec, "revision": 0
                    }))
           .await?
           .unwrap())
  }

  pub async fn set_instance_power_state(&self, id: &str, power_state: Option<InstancePowerState>) -> Result<Option<InstanceData>> {
    Ok(self.db
           .query("update $instance set power_state = $power_state, revision = revision + 1")
           .bind(("instance", Thing::from(("instance", id))))
           .bind(("power_state", power_state))
           .await?
           .take(0)?)
  }

  pub async fn set_instance_play_state(&self, id: &str, play_state: Option<InstancePlayState>) -> Result<Option<InstanceData>> {
    Ok(self.db
           .query("update $instance set play_state = $play_state, revision = revision + 1")
           .bind(("instance", Thing::from(("instance", id))))
           .bind(("play_state", play_state))
           .await?
           .take(0)?)
  }
}

#[cfg(test)]
mod test {
  use anyhow::anyhow;

  use api::instance::driver::config::InstanceDriverConfig;
  use api::instance::model::InstanceModel;

  use super::*;

  #[tokio::test]
  async fn sanity() -> Result {
    let db = Db::new_in_mem().await?;
    assert!(db.get_instances(None).await?.is_empty(), "initial instances should be empty");
    assert!(db.get_instances(Some("host".to_string())).await?.is_empty(),
            "initial instances belonging to a host should be empty");

    Ok(())
  }

  #[tokio::test]
  async fn create_and_query() -> Result {
    let spec = InstanceSpec { model:      InstanceModel { parameters:    Default::default(),
                                                          reports:       Default::default(),
                                                          audio_inputs:  0,
                                                          audio_outputs: 0,
                                                          supports:      Default::default(), },
                              host:       "host".to_string(),
                              power:      None,
                              media:      None,
                              attachment: None,
                              driver:     InstanceDriverConfig::Mock, };

    let db = Db::new_in_mem().await?;

    let instance = db.register_instance("id", spec.clone()).await?;
    assert_eq!(instance.spec, spec, "returned instance spec should match after insertion");
    assert_eq!(instance.revision, 0, "returned instance revision should be 0 after insertion");

    let instances = db.get_instances(None).await?;
    assert_eq!(instances.len(), 1, "instances should have 1 entry after insertion");
    assert_eq!(instances[0].spec, spec, "returned instance spec should match after insertion");

    let instance = db.get_instance_by_id("id").await?;
    assert!(instance.is_some(), "instance should be found by id after insertion");
    assert_eq!(instance.unwrap().spec, spec, "returned instance spec should match after insertion");

    let instances = db.get_instances(Some("host".to_string())).await?;
    assert_eq!(instances.len(), 1, "instances hosted by host should have 1 entry after insertion");
    assert_eq!(instances[0].spec, spec,
               "returned instances spec hosted by host should match after insertion");

    Ok(())
  }

  #[tokio::test]
  async fn modify_and_query() -> Result {
    let spec = InstanceSpec { model:      InstanceModel { parameters:    Default::default(),
                                                          reports:       Default::default(),
                                                          audio_inputs:  0,
                                                          audio_outputs: 0,
                                                          supports:      Default::default(), },
                              host:       "host".to_string(),
                              power:      None,
                              media:      None,
                              attachment: None,
                              driver:     InstanceDriverConfig::Mock, };

    let db = Db::new_in_mem().await?;

    db.register_instance("id", spec.clone()).await?;

    let instance = db.set_instance_play_state("id", Some(InstancePlayState::Idle))
                     .await?
                     .ok_or_else(|| anyhow!("instance should be returned after update"))?;

    assert_eq!(instance.play_state,
               Some(InstancePlayState::Idle),
               "returned instance play state should match after insertion");
    assert_eq!(instance.revision, 1, "returned instance revision should be 1 after i[date");

    let instance = db.get_instance_by_id("id")
                     .await?
                     .ok_or_else(|| anyhow!("instance should be found by id after update"))?;

    assert_eq!(instance.play_state,
               Some(InstancePlayState::Idle),
               "returned instance play state should match after update");
    assert_eq!(instance.revision, 1, "returned instance revision should be 1 after update");

    let instance = db.set_instance_power_state("id", Some(InstancePowerState::On))
                     .await?
                     .ok_or_else(|| anyhow!("instance should be returned after update"))?;

    assert_eq!(instance.power_state,
               Some(InstancePowerState::On),
               "returned instance power state should match after update");
    assert_eq!(instance.revision, 2, "returned instance revision should be 2 after update");

    let instance = db.get_instance_by_id("id")
                     .await?
                     .ok_or_else(|| anyhow!("instance should be found by id after update"))?;

    assert_eq!(instance.power_state,
               Some(InstancePowerState::On),
               "returned instance power state should match after update");

    assert_eq!(instance.revision, 2, "returned instance revision should be 2 after update");

    Ok(())
  }
}
