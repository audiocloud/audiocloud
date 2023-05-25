use serde::{Deserialize, Serialize};
use serde_json::Map;
use surrealdb::sql::Thing;

use api_proto::{GlobalPermission, TaskPermission};

use crate::{Db, Identified, Result, Timestamp};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUserData {
  pub id:          Thing,
  pub email:       Option<String>,
  pub password:    String,
  pub permissions: Vec<GlobalPermission>,
  pub disabled_at: Option<Timestamp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbAppData {
  pub id:          Thing,
  pub permissions: Vec<GlobalPermission>,
  pub disabled_at: Option<Timestamp>,
}

#[derive(Default)]
pub struct DbUpdateUser {
  pub set_email:       Option<Option<String>>,
  pub set_permissions: Option<Vec<GlobalPermission>>,
  pub set_password:    Option<String>,
  pub set_disabled_at: Option<Option<Timestamp>>,
}

#[derive(Default)]
pub struct DbUpdateApp {
  pub set_permissions: Option<Vec<GlobalPermission>>,
  pub set_disabled_at: Option<Option<Timestamp>>,
}

#[derive(Serialize, Deserialize)]
pub struct DbApiKeyData {
  pub id:               Thing,
  pub hash:             String,
  pub task:             Option<Thing>,
  pub user:             Option<Thing>,
  pub app:              Option<Thing>,
  pub permissions:      Vec<GlobalPermission>,
  pub task_permissions: Vec<TaskPermission>,
  pub expires_at:       Timestamp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbApiKeyDataResolveUserApp {
  pub id:               Thing,
  pub hash:             String,
  pub task:             Option<Thing>,
  pub user:             Option<DbUserData>,
  pub app:              Option<DbAppData>,
  pub permissions:      Vec<GlobalPermission>,
  pub task_permissions: Vec<TaskPermission>,
  pub expires_at:       Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct DbTokenData {
  pub id:         Thing,
  pub user:       Option<Thing>,
  pub app:        Option<Thing>,
  pub expires_at: Timestamp,
}

#[derive(Deserialize)]
pub struct DbTokenResolvedData {
  pub id:         Thing,
  pub user:       Option<DbUserData>,
  pub app:        Option<DbAppData>,
  pub expires_at: Timestamp,
}

pub enum DbPrincipal {
  User(String),
  App(String),
}

#[derive(Serialize, Debug)]
pub struct DbCreateUser {
  pub email:       Option<String>,
  pub password:    String,
  pub permissions: Vec<GlobalPermission>,
}

#[derive(Serialize, Debug)]
pub struct DbCreateApp {
  pub permissions: Vec<GlobalPermission>,
}

pub struct DbCreateApiKey {
  pub name:             String,
  pub hash:             String,
  pub task:             Option<String>,
  pub permissions:      Vec<GlobalPermission>,
  pub task_permissions: Vec<TaskPermission>,
  pub expires_at:       Timestamp,
}

pub struct DbUpdateApiKey {
  pub set_expires_at: Option<Timestamp>,
}

impl Db {
  pub async fn get_user_by_id(&self, id: &str) -> Result<Option<DbUserData>> {
    Ok(self.surreal.select(("user", id)).await?)
  }

  pub async fn list_users(&self, filter_id: Option<&str>, filter_email: Option<&str>, offset: u32, limit: u32) -> Result<Vec<DbUserData>> {
    let mut conditions = vec!["1=1"];
    if filter_id.is_some() {
      conditions.push("id = $filter_id");
    }
    if filter_email.is_some() {
      conditions.push("email = $filter_email");
    }

    let conditions = conditions.join(" AND ");

    Ok(self.surreal
           .query(format!("select * from user where {conditions} limit $limit start $offset"))
           .bind(("filter_id", filter_id))
           .bind(("filter_email", filter_email))
           .bind(("limit", limit))
           .bind(("offset", offset))
           .await?
           .take(0)?)
  }

  pub async fn get_app_by_id(&self, id: &str) -> Result<Option<DbAppData>> {
    Ok(self.surreal.select(("app", id)).await?)
  }

  pub async fn list_apps(&self, filter_id: Option<&str>, offset: u32, limit: u32) -> Result<Vec<DbAppData>> {
    let mut conditions = vec!["1=1"];
    if filter_id.is_some() {
      conditions.push("id = $filter_id");
    }

    let conditions = conditions.join(" AND ");

    Ok(self.surreal
           .query(format!("select * from app where {conditions} limit $limit start $offset"))
           .bind(("filter_id", filter_id))
           .bind(("limit", limit))
           .bind(("offset", offset))
           .await?
           .take(0)?)
  }

  pub async fn list_api_keys(&self,
                             filter_user_id: Option<&str>,
                             filter_app_id: Option<&str>,
                             offset: u32,
                             limit: u32)
                             -> Result<Vec<DbApiKeyData>> {
    let mut conditions = vec!["1=1"];
    if filter_user_id.is_some() {
      conditions.push("user = $filter_user_id");
    }
    if filter_app_id.is_some() {
      conditions.push("app = $filter_app_id");
    }

    let conditions = conditions.join(" AND ");
    Ok(self.surreal
           .query(format!("select * from api_key where {conditions} limit $limit start $offset"))
           .bind(("filter_user_id", filter_user_id.map(|id| Thing::from(("user", id)))))
           .bind(("filter_app_id", filter_app_id.map(|id| Thing::from(("app", id)))))
           .bind(("limit", limit))
           .bind(("offset", offset))
           .await?
           .take(0)?)
  }

  pub async fn create_user(&self, username: &str, create: DbCreateUser) -> Result<DbUserData> {
    Ok(self.surreal.update(("user", username)).merge(create).await?)
  }

  pub async fn update_user(&self, username: &str, db_update: DbUpdateUser) -> Result<DbUserData> {
    let mut update = Map::new();
    if let Some(set_email) = db_update.set_email {
      update.insert("email".to_owned(), serde_json::to_value(set_email)?);
    }
    if let Some(set_permissions) = db_update.set_permissions {
      update.insert("permissions".to_owned(), serde_json::to_value(set_permissions)?);
    }

    Ok(self.surreal
           .update(("user", username))
           .merge(serde_json::Value::Object(update))
           .await?)
  }

  pub async fn delete_user(&self, username: &str) -> Result {
    let id = Thing::from(("user", username));

    self.surreal
        .query("delete from user where id = $id")
        .query("delete from api_key where user = $id")
        .bind(("id", id))
        .await?;

    Ok(())
  }

  pub async fn create_app(&self, app_id: &str, create: DbCreateApp) -> Result<DbAppData> {
    Ok(self.surreal.update(("app", app_id)).merge(create).await?)
  }

  pub async fn update_app(&self, app_id: &str, db_update: DbUpdateApp) -> Result<DbAppData> {
    let mut update = Map::new();
    if let Some(set_permissions) = db_update.set_permissions {
      update.insert("permissions".to_owned(), serde_json::to_value(set_permissions)?);
    }

    Ok(self.surreal
           .update(("app", app_id))
           .merge(serde_json::Value::Object(update))
           .await?)
  }

  pub async fn get_api_key_by_id(&self, id: &str) -> Result<Option<DbApiKeyDataResolveUserApp>> {
    Ok(self.surreal
           .query("select * from $id fetch user, app")
           .bind(("id", Thing::from(("api_key", id))))
           .await?
           .take(0)?)
  }

  pub async fn get_token_by_id(&self, id: &str) -> Result<Option<DbTokenResolvedData>> {
    Ok(self.surreal
           .query("select * from $id fetch user, app")
           .bind(("id", Thing::from(("token", id))))
           .await?
           .take(0)?)
  }

  pub async fn create_token(&self, principal: DbPrincipal, expires_at: Timestamp) -> Result<DbTokenData> {
    #[derive(Serialize, Default)]
    struct DbCreateToken {
      user:       Option<Thing>,
      app:        Option<Thing>,
      expires_at: Timestamp,
    }

    let create_data = match principal {
      | DbPrincipal::User(user_id) => DbCreateToken { user: Some(Thing::from(("user", user_id.as_str()))),
                                                      expires_at,
                                                      ..Default::default() },
      | DbPrincipal::App(app_id) => DbCreateToken { app: Some(Thing::from(("app", app_id.as_str()))),
                                                    expires_at,
                                                    ..Default::default() },
    };

    Ok(self.surreal.create("token").content(create_data).await?)
  }

  pub async fn update_token(&self, id: &str, set_expires_at: Option<Timestamp>) -> Result<Identified> {
    let mut update = Map::new();
    if let Some(set_expires_at) = set_expires_at {
      update.insert("expires_at".to_owned(), serde_json::to_value(set_expires_at)?);
    }

    Ok(self.surreal.update(("token", id)).merge(serde_json::Value::Object(update)).await?)
  }

  pub async fn create_api_key(&self, id: Option<String>, principal: DbPrincipal, create: DbCreateApiKey) -> Result<DbApiKeyData> {
    #[derive(Serialize)]
    pub struct DbCreateApiKeyResolved {
      pub hash:             String,
      pub task:             Option<Thing>,
      pub user:             Option<Thing>,
      pub app:              Option<Thing>,
      pub permissions:      Vec<GlobalPermission>,
      pub task_permissions: Vec<TaskPermission>,
      pub expires_at:       Timestamp,
    }

    let (user, app) = match principal {
      | DbPrincipal::User(user_id) => (Some(Thing::from(("user", user_id.as_str()))), None),
      | DbPrincipal::App(app_id) => (None, Some(Thing::from(("app", app_id.as_str())))),
    };

    let create = DbCreateApiKeyResolved { user,
                                          app,
                                          hash: create.hash,
                                          task: create.task.as_ref().map(|task_id| Thing::from(("task", task_id.as_str()))),
                                          permissions: create.permissions,
                                          task_permissions: create.task_permissions,
                                          expires_at: create.expires_at };

    match id {
      | Some(id) => Ok(self.surreal.update(("api_key", id.as_str())).content(create).await?),
      | None => Ok(self.surreal.create("api_key").content(create).await?),
    }
  }

  pub async fn update_api_key(&self, id: &str, db_update: DbUpdateApiKey) -> Result<Option<DbApiKeyData>> {
    let mut update = serde_json::Map::new();

    if let Some(expires_at) = db_update.set_expires_at {
      update.insert("expires_at".to_owned(), serde_json::to_value(expires_at)?);
    }

    Ok(self.surreal
           .update(("api_key", id))
           .merge(serde_json::Value::Object(update))
           .await?)
  }
}
