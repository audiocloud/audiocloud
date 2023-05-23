use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{Db, Result};

#[derive(Serialize, Deserialize)]
pub struct UserData {
  pub id:       Thing,
  pub email:    String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeyData {
  pub id:   Thing,
  pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiTokenData {
  pub id: Thing,
}

impl Db {
  pub async fn get_user_by_id(&self, id: &str) -> Result<Option<UserData>> {
    Ok(self.surreal.select(("user", id)).await?)
  }

  pub async fn get_api_key_by_id(&self, id: &str) -> Result<Option<UserData>> {
    Ok(self.surreal.select(("api_key", id)).await?)
  }
}
