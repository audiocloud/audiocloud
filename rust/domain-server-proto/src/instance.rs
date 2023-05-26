use async_stream::try_stream;
use axum_connect::pbjson_types::Empty;
use futures::{pin_mut, Stream, StreamExt};

use api_proto::*;

use crate::context::ServiceContext;
use crate::error::{auth_error, not_implemented_error, RpcResult};
use crate::nats::nats_subscribe;

pub async fn set_instance_spec_handler(context: ServiceContext, request: SetInstanceSpecRequest) -> RpcResult<SetInstanceSpecResponse> {
  if context.permissions.contains(&GlobalPermission::InstanceSetSpec) {
    return auth_error("not authorized".to_owned());
  }

  not_implemented_error("SetInstanceSpec")
}

pub async fn subscribe_instance_events_handler(context: ServiceContext,
                                               request: SubscribeInstanceEventsRequest)
                                               -> impl Stream<Item = RpcResult<InstanceEvent>> {
  try_stream! {
    if !context.permissions.contains(&GlobalPermission::InstanceRead) {
      auth_error(format!("No permission to subscribe instance events"))?;
    }

    let stream = nats_subscribe::<InstanceEvent>(context.nats.clone(), format!("instance.{}.events", request.instance_id));

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
      yield event?;
    }
  }
}

pub async fn set_parameter_handler(context: ServiceContext,
                                   request: SetInstanceParameterRequest)
                                   -> RpcResult<SetInstanceParameterResponse> {
  context.require_global_permissions([GlobalPermission::InstanceSetParameters].into_iter())?;

  Ok(SetInstanceParameterResponse {})
}

pub async fn set_instance_desired_media_state_handler(context: ServiceContext,
                                                      request: SetInstanceDesiredMediaStateRequest)
                                                      -> RpcResult<Empty> {
  context.require_global_permissions([GlobalPermission::InstanceSetMedia].into_iter())?;

  Ok(Empty {})
}

pub async fn list_instances_handler(context: ServiceContext, request: ListInstancesRequest) -> RpcResult<ListInstancesResponse> {
  context.require_global_permissions([GlobalPermission::InstanceRead].into_iter())?;

  not_implemented_error("ListInstances")
}

pub async fn describe_instance_handler(context: ServiceContext, request: DescribeInstanceRequest) -> RpcResult<DescribeInstanceResponse> {
  context.require_global_permissions([GlobalPermission::InstanceRead].into_iter())?;

  // TODO: ... let instance = context.db.get_instance_by_id(&request.id).await?;

  Ok(DescribeInstanceResponse { id:            "".to_string(),
                                host:          "".to_string(),
                                spec:          None,
                                current_power: None,
                                current_media: None,
                                maintenance:   vec![], })
}

pub async fn claim_instance_handler(context: ServiceContext, request: ClaimInstanceRequest) -> RpcResult<Empty> {
  context.require_global_permissions([GlobalPermission::InstanceClaim].into_iter())?;

  not_implemented_error("ClaimInstance")
}

pub async fn release_instance_handler(context: ServiceContext, request: ReleaseInstanceRequest) -> RpcResult<Empty> {
  context.require_global_permissions([GlobalPermission::InstanceClaim].into_iter())?;

  not_implemented_error("ReleaseInstance")
}

pub async fn add_instance_maintenance_handler(context: ServiceContext,
                                              request: AddInstanceMaintenanceRequest)
                                              -> RpcResult<InstanceMaintenanceDetails> {
  context.require_global_permissions([GlobalPermission::InstanceSetMaintenance].into_iter())?;

  not_implemented_error("AddInstanceMaintenance")
}

pub async fn update_instance_maintenance_handler(context: ServiceContext,
                                                 request: UpdateInstanceMaintenanceRequest)
                                                 -> RpcResult<InstanceMaintenanceDetails> {
  context.require_global_permissions([GlobalPermission::InstanceSetMaintenance].into_iter())?;

  not_implemented_error("UpdateInstanceMaintenance")
}

