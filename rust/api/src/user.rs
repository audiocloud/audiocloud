use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSummary {
  pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
  pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
  pub set_password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserResponse {
  pub id:      String,
  pub updated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserResponse {
  pub id:      String,
  pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSpec {
  pub id:       String,
  pub password: String,
}

pub mod buckets {
  use crate::BucketName;

  use super::UserSpec;

  pub const USER_SPEC: BucketName<UserSpec> = BucketName::new("audiocloud_user_spec");
}
