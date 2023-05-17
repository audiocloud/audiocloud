use anyhow::bail;

use api::task::{CreateTaskRequest, CreateTaskResponse};

use super::{Result, Service};

impl Service {
  pub async fn create_task(&self, create: CreateTaskRequest) -> Result<CreateTaskResponse> {
    bail!("Not implemented")
  }
}
