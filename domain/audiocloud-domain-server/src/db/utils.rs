use std::collections::BTreeMap;

use anyhow::anyhow;
use maplit::btreemap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::sql::{Id, Value};
use surrealdb::{sql, Session};

use crate::db::Db;

impl Db {
    pub fn normalize_thing(result: Value) -> Value {
        match result {
            Value::Array(array) => Value::Array(array.0.into_iter().map(Self::normalize_thing).collect::<Vec<_>>().into()),
            Value::Object(object) => Value::Object(object.0
                                                         .into_iter()
                                                         .map(|(k, v)| (k, Self::normalize_thing(v)))
                                                         .collect::<BTreeMap<_, _>>()
                                                         .into()),
            Value::Thing(id) => match id.id {
                Id::Number(num) => Value::from(num),
                Id::String(string) => Value::from(string),
                Id::Array(array) => Value::from(array),
                Id::Object(object) => Value::from(object),
            },
            agree => agree,
        }
    }

    pub async fn query_by_id<I, T>(&self, query: &str, id: &I) -> anyhow::Result<Option<T>>
        where T: DeserializeOwned,
              I: Serialize
    {
        let vars = btreemap! {"id".to_owned() => sql::json(&serde_json::to_string(id)?)?};

        self.query_one(query, Some(vars)).await
    }

    pub async fn query_one<T>(&self, query: &str, vars: Option<BTreeMap<String, Value>>) -> anyhow::Result<Option<T>>
        where T: DeserializeOwned
    {
        let result = self.query_multi(query, vars).await?;
        Ok(result.into_iter().next())
    }

    pub async fn query_multi<T>(&self, query: &str, vars: Option<BTreeMap<String, Value>>) -> anyhow::Result<Vec<T>>
        where T: DeserializeOwned
    {
        let ses = Self::session();
        let res = self.db.execute(query, &ses, vars, false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let result = Self::normalize_thing(res.result?);

        let result: Vec<T> = serde_json::from_value(serde_json::to_value(&result)?)?;
        Ok(result)
    }

    pub async fn get_model_by_id<I, T>(&self, table: &str, id: &I) -> anyhow::Result<Option<T>>
        where T: DeserializeOwned,
              I: Serialize
    {
        let ast = format!(r#"select type::thing({table}, $id)"#);
        self.query_by_id(&ast, id).await
    }

    pub async fn execute(&self, ast: &str, params: Option<BTreeMap<String, Value>>) -> anyhow::Result<()> {
        let ses = Self::session();

        let res = self.db.execute(ast, &ses, params, false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let ___ = res.result?;

        Ok(())
    }

    pub async fn execute_no_params(&self, ast: &str) -> anyhow::Result<()> {
        self.execute(ast, None).await
    }

    pub fn session() -> Session {
        Session::for_db("audiocloud", "domain")
    }
}
