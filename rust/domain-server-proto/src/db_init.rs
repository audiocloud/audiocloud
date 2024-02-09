use argon2::Argon2;
use nanoid::nanoid;
use password_hash::{PasswordHash, SaltString};
use tracing::info;

use api_proto::GlobalPermission;
use domain_db::security::{DbCreateApiKey, DbCreateApp, DbPrincipal};
use domain_db::Db;

pub async fn run(db: &Db) -> anyhow::Result<()> {
  
  let all_permissions = vec![GlobalPermission::TaskCreate,
                             GlobalPermission::TaskRead,
                             GlobalPermission::TaskSeek,
                             GlobalPermission::TaskSetGraph,
                             GlobalPermission::TaskSetMedia,
                             GlobalPermission::TaskSetPlayState,
                             GlobalPermission::TaskDelete,
                             GlobalPermission::InstanceRead,
                             GlobalPermission::InstanceSetSpec,
                             GlobalPermission::InstanceSetParameters,
                             GlobalPermission::InstanceSetMedia,
                             GlobalPermission::InstanceSetPower,
                             GlobalPermission::InstanceReferenceInputOutput,];

  db.create_app("admin", DbCreateApp { permissions: all_permissions.clone(), })
    .await?;

  let raw_password = nanoid!();

  let hashed_password = PasswordHash::generate(Argon2::default(),
                                               raw_password.as_bytes(),
                                               &SaltString::generate(rand::thread_rng()))?.to_string();

  let create_api_key = DbCreateApiKey { name:             format!("Automatically generated API key"),
                                        hash:             hashed_password,
                                        task:             None,
                                        permissions:      all_permissions,
                                        task_permissions: vec![],
                                        expires_at:       chrono::Utc::now() + chrono::Duration::days(365), };

  let target_app = DbPrincipal::App("admin".to_owned());

  let api_key = db.create_api_key(None, target_app, create_api_key).await?;

  info!("built-in admin API key: {}:{}", &api_key.id.id, raw_password);

  Ok(())
}
