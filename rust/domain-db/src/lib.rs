use surrealdb::engine::any::Any;

pub type Result<T = ()> = anyhow::Result<T>;

pub mod instances;
pub mod users;
pub mod media;
pub mod tasks;

pub struct Db {
  db: surrealdb::Surreal<Any>,
}

impl Db {
  pub async fn new_in_mem() -> Result<Self> {
    let db = surrealdb::engine::any::connect("mem://").await?;

    db.use_ns("audiocloud").use_db("domain").await?;

    Ok(Self { db })
  }
}
