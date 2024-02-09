use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::task::TaskId;
use crate::user::UserSummary;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Auth {
  System,
  User(UserSummary),
  App(TaskId),
}
