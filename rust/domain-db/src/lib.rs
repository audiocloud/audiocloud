use surrealdb::engine::any::Any;
use surrealdb::Surreal;

pub mod security;

pub struct Db {
  surreal: Surreal<Any>,
}

pub type Result<T = ()> = anyhow::Result<T>;

impl Db {
  pub async fn new_in_mem() -> Result<Self> {
    let surreal = surrealdb::engine::any::connect("mem://").await?;
    surreal.use_ns("audiocloud").use_db("audiocloud").await?;

    Ok(Self { surreal })
  }
}
