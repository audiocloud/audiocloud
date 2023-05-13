use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserResponse {
  pub token: String,
}

pub mod buckets {
  use crate::BucketName;

  use super::UserSpec;

  pub const USER_SPEC: BucketName<UserSpec> = BucketName::new("audiocloud_user_spec");
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(UserSummary),
                 schema_for!(CreateUserRequest),
                 schema_for!(CreateUserResponse),
                 schema_for!(DeleteUserResponse),
                 schema_for!(UserSpec),
                 schema_for!(LoginUserResponse)].into_iter())
}
