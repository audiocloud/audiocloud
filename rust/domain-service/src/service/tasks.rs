use std::collections::{HashMap, HashSet};

use anyhow::bail;

use api::auth::Auth;
use api::task::spec::TaskSpec;
use api::task::{
  CreateTaskRequest, CreateTaskResponse, DeleteTaskResponse, DesiredTaskPlayState, InstanceAllocationRequest, ModifyTaskGraphRequest,
  ModifyTaskGraphResponse, SetTaskGraphRequest, SetTaskGraphResponse, SetTaskTimeRequest, SetTaskTimeResponse,
};
use api::{BucketKey, Timestamp};

use super::{Result, Service};

impl Service {
  pub async fn create_task(&self, auth: Auth, id: String, create: CreateTaskRequest) -> Result<CreateTaskResponse> {
    let existing = self.nats.task_spec.get(BucketKey::new(&id)).await?;
    if existing.is_some() {
      bail!("Task already exists");
    }

    // get all of existing ones
    let allocated = self.allocate_instances(create.from, create.to, &create.instances, None).await?;

    // we are fine, can create
    let spec = TaskSpec { app:        create.app.clone(),
                          engine:     create.engine.clone(),
                          from:       create.from,
                          to:         create.to,
                          requests:   create.instances.clone(),
                          instances:  allocated,
                          graph_spec: Default::default(), };

    let ctrl = DesiredTaskPlayState::Idle;

    let state = (); // TODO: when state will be more defined, insert it

    self.nats.task_spec.put(BucketKey::new(&id), spec).await?;
    self.nats.task_ctrl.put(BucketKey::new(&id), ctrl).await?;
    self.nats.task_state.put(BucketKey::new(&id), state).await?;

    Ok(CreateTaskResponse::Success { app_id:  create.app.clone(),
                                     task_id: id.clone(), })
  }

  pub async fn set_task_graph(&self, auth: Auth, id: String, new_graph_spec: SetTaskGraphRequest) -> Result<SetTaskGraphResponse> {
    let Some(mut spec) = self.nats.task_spec.get(BucketKey::new(&id)).await? else { return Ok(SetTaskGraphResponse::NotFound) };
    spec.graph_spec = new_graph_spec;

    self.nats.task_spec.put(BucketKey::new(&id), spec).await?;

    Ok(SetTaskGraphResponse::Success)
  }

  pub async fn set_task_time(&self, auth: Auth, id: String, set: SetTaskTimeRequest) -> Result<SetTaskTimeResponse> {
    let Some(mut spec) = self.nats.task_spec.get(BucketKey::new(&id)).await? else { return Ok(SetTaskTimeResponse::NotFound) };

    let allocated = self.allocate_instances(set.from, set.to, &spec.requests, Some(&id)).await?;

    spec.from = set.from;
    spec.to = set.to;
    spec.instances = allocated;

    self.nats.task_spec.put(BucketKey::new(&id), spec).await?;

    Ok(SetTaskTimeResponse::Success)
  }

  pub async fn modify_task_graph(&self, auth: Auth, id: String, modify: ModifyTaskGraphRequest) -> Result<ModifyTaskGraphResponse> {
    let Some(mut spec) = self.nats.task_spec.get(BucketKey::new(&id)).await? else { return Ok(ModifyTaskGraphResponse::NotFound) };

    // TODO: apply modification

    self.nats.task_spec.put(BucketKey::new(&id), spec).await?;

    Ok(ModifyTaskGraphResponse::Success)
  }

  pub async fn delete_task(&self, auth: Auth, id: String) -> Result<DeleteTaskResponse> {
    let Some(mut spec) = self.nats.task_spec.get(BucketKey::new(&id)).await? else { return Ok(DeleteTaskResponse::NotFound) };

    self.nats.task_spec.delete(BucketKey::new(&id)).await?;
    self.nats.task_ctrl.delete(BucketKey::new(&id)).await?;
    self.nats.task_state.delete(BucketKey::new(&id)).await?;

    Ok(DeleteTaskResponse::Success)
  }

  async fn allocate_instances(&self,
                              from: Timestamp,
                              to: Timestamp,
                              requests: &HashMap<String, InstanceAllocationRequest>,
                              ignore_task_id: Option<&str>)
                              -> Result<HashMap<String, String>> {
    let all_tasks = self.nats.task_spec.scan("*").await?;
    let all_instances = self.nats.instance_spec.scan("*").await?;

    let mut available_instances = all_instances.keys().cloned().collect::<HashSet<_>>();
    for (id, task) in &all_tasks {
      if task.from >= to || task.to <= from {
        continue;
      }
      if let Some(ignore_task_id) = &ignore_task_id {
        if ignore_task_id == id {
          continue;
        }
      }

      available_instances.retain(|id| task.instances.values().all(|claimed_id| claimed_id != id));
    }

    let mut allocated = HashMap::new();

    for (alloc_id, alloc_request) in requests {
      match alloc_request {
        | InstanceAllocationRequest::Fixed { instance_id } => {
          if !available_instances.contains(instance_id) {
            bail!("Instance {} is not available", instance_id);
          }

          allocated.insert(alloc_id.clone(), instance_id.clone());
        }
        | InstanceAllocationRequest::Dynamic { model_id } => {
          let mut found = false;
          // TODO: is model instanced on the engine?

          for instance_id in &available_instances {
            let Some(instance) = all_instances.get(instance_id) else { continue };
            // TODO: check model id
          }

          if !found {
            bail!("No available instance with model {}", model_id);
          }
        }
      }
    }

    Ok(allocated)
  }
}
