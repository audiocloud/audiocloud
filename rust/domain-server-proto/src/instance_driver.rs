use std::collections::HashMap;

use async_stream::try_stream;
use futures::Stream;
use lazy_static::lazy_static;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;

use api_proto::*;

use crate::context::ServiceContext;
use crate::error::{auth_error, not_found_error, RpcResult};

pub mod run;

lazy_static! {
  static ref SHARED_INSTANCE_EVENTS: RwLock<HashMap<String, Sender<InstanceDriverEvent>>> = RwLock::new(HashMap::new());
}

pub async fn set_parameter_handler(context: ServiceContext,
                                   request: SetInstanceParameterRequest)
                                   -> RpcResult<SetInstanceParameterResponse> {
  context.require_global_permissions([GlobalPermission::InstanceSetParameters].into_iter())?;

  api_proto::instances::v1::DomainInstanceService::

  Ok(SetInstanceParameterResponse {})
}

pub async fn subscribe_instance_events_handler(context: ServiceContext,
                                               request: SubscribeInstanceEventsRequest)
                                               -> impl Stream<Item = RpcResult<InstanceDriverEvent>> {
  try_stream! {
    // the AIO will come in early next week and I guess it would be good to see if it fits well
    if !context.permissions.contains(&GlobalPermission::InstanceRead) {
      auth_error(format!("No permission to subscribe instance events"))?;
    }

    let shared_event_bus = SHARED_INSTANCE_EVENTS.read().await;
    let Some(clone) = shared_event_bus.get(&request.instance_id) else { not_found_error("Instance event stream not found".to_owned())?; return; };
    let mut receiver = clone.subscribe();
    drop(shared_event_bus);

    while let Ok(event) = receiver.recv().await {
      yield event;
    }
  }
}
