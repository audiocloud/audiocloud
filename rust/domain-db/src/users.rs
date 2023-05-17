use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use crate::{Db, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
  pub id:       Thing,
  pub password: String,
}

impl Db {
  pub async fn get_user_by_id(&self, id: &str) -> Result<Option<UserData>> {
    Ok(self.db.select(("user", id)).await?)
  }

  pub async fn register_user(&self, id: &str, password: &str) -> Result<UserData> {
    Ok(self.db.create(("user", id)).content(json!({ "password": password })).await?)
  }

  pub async fn list_users(&self, skip: usize, limit: usize, search: Option<&str>) -> Result<Vec<UserData>> {
    let mut q = "SELECT * from user ".to_string();
    if search.is_some() {
      q = q + " WHERE id CONTAINS $search ";
    }
    q = q + " ORDER BY id LIMIT $limit START $skip";

    Ok(self.db
           .query(q)
           .bind(("limit", limit))
           .bind(("skip", skip))
           .bind(("search", search))
           .await?
           .take(0)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn sanity() -> Result {
    let db = Db::new_in_mem().await?;

    assert!(db.get_user_by_id("admin@domain.com").await?.is_none(),
            "user admin@domain.com should not exist");

    Ok(())
  }

  #[tokio::test]
  async fn register_user() -> Result {
    let db = Db::new_in_mem().await?;

    let user = db.register_user("admin@domain.com", "password").await?;
    assert_eq!(user.id,
               Thing::from(("user", "admin@domain.com")),
               "user id should be user:admin@domain.com");

    let user = db.get_user_by_id("admin@domain.com")
                 .await?
                 .ok_or_else(|| anyhow::anyhow!("user admin@domain.com must exist after insertion"))?;

    assert_eq!(user.id, Thing::from(("user", "admin@domain.com")));
    assert_eq!(user.password, "password");

    Ok(())
  }

  #[tokio::test]
  async fn search_users() -> Result {
    let db = Db::new_in_mem().await?;

    db.register_user("admin@domain.com", "password").await?;
    db.register_user("test@domain.com", "password").await?;
    db.register_user("foo@bar.com", "password").await?;

    let users = db.list_users(0, 10, None).await?;
    assert_eq!(users.len(), 3, "should have 3 users in the database matching no filter");

    let users = db.list_users(0, 10, Some("admin")).await?;
    assert_eq!(users.len(), 1, "should have 1 user in the database matching search for admin");

    let users = db.list_users(0, 10, Some("domain.com")).await?;
    assert_eq!(users.len(), 2, "should have 2 users in the database matching search for domain.com");

    let users = db.list_users(0, 10, Some("foo")).await?;
    assert_eq!(users.len(), 1, "should have 1 users in the database matching search for foo");

    let users = db.list_users(0, 10, Some("bar")).await?;
    assert_eq!(users.len(), 1, "should have 1 users in the database matching search for bar");

    let users = db.list_users(0, 10, Some("baz")).await?;
    assert_eq!(users.len(), 0, "should have 0 users in the database matching search for baz");

    let users = db.list_users(0, 1, None).await?;
    assert_eq!(users.len(),
               1,
               "should have 1 users in the database matching no filter but limiting to 1 result");

    let users = db.list_users(1, 1, None).await?;
    assert_eq!(users.len(),
               1,
               "should have 1 users in the database matching no filter but limiting to 1 result and skipping 1");

    Ok(())
  }
}
