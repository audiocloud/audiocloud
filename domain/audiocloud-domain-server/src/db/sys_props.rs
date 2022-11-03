use anyhow::anyhow;
use maplit::btreemap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::db::prisma::sys_prop::Data;
use crate::db::Db;

use super::prisma::sys_prop;

impl Db {
    pub async fn get_sys_prop<T: DeserializeOwned>(&self, prop_id: &str) -> anyhow::Result<Option<T>> {
        Ok(match self.db
                     .sys_prop()
                     .find_unique(sys_prop::id::equals(prop_id.to_owned()))
                     .exec()
                     .await?
           {
               None => None,
               Some(sys_prop) => serde_json::from_str(&sys_prop.value)?,
           })
    }

    pub async fn set_sys_prop<T: Serialize>(&self, prop_id: &str, value: &T) -> anyhow::Result<()> {
        let value_as_string = serde_json::to_string_pretty(value)?;

        self.db
            .sys_prop()
            .upsert(sys_prop::id::equals(prop_id.to_owned()),
                    sys_prop::create(prop_id.to_owned(), value_as_string.clone(), vec![]),
                    vec![sys_prop::value::set(value_as_string),])
            .exec()
            .await?;

        Ok(())
    }
}
