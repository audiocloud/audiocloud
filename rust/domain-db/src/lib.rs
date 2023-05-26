use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::{Namespace, Root};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub mod security;

#[derive(Debug, Clone)]
pub struct Db {
  surreal: Surreal<Any>,
}

#[derive(Serialize, Deserialize)]
pub struct Identified {
  id: Thing,
}

pub type Timestamp = DateTime<Utc>;

pub type Result<T = ()> = anyhow::Result<T>;

impl Db {
  pub async fn new_in_mem() -> Result<Self> {
    let surreal = surrealdb::engine::any::connect("mem://").await?;
    surreal.use_ns("audiocloud").use_db("audiocloud").await?;

    Ok(Self { surreal })
  }

  pub async fn new(db_url: &str, use_root: bool, namespace: &str, username: &str, password: &str) -> Result<Self> {
    let surreal = surrealdb::engine::any::connect(db_url).await?;

    if use_root {
      surreal.signin(Root { username, password }).await?;
    } else {
      surreal.signin(Namespace { namespace,
                                 username,
                                 password })
             .await?;
    }

    surreal.use_ns(namespace).use_db(namespace).await?;

    Ok(Self { surreal })
  }
}
