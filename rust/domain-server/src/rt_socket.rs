use futures::channel::mpsc;
use futures::stream::FusedStream;
use futures::{SinkExt, StreamExt};
use tokio::select;
use tracing::warn;

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;
use api::instance::spec::InstanceSpec;
use api::rt::{RtCommand, RtEvent, RtRequest};

use crate::nats::{EventStreamMap, WatchStreamMap};
use crate::service::Service;

type InstanceEventsMap = EventStreamMap<String, InstanceDriverEvent>;
type InstanceSpecsMap = WatchStreamMap<String, InstanceSpec>;

pub async fn run_socket(service: Service, mut rx: mpsc::Receiver<RtRequest>, mut tx: mpsc::Sender<RtEvent>) {
  let mut instance_events = InstanceEventsMap::new();
  let mut instance_specs = InstanceSpecsMap::new();

  while !rx.is_terminated() && !tx.is_closed() {
    select! {
      Some(request) = rx.next() => handle_request(&service, &mut instance_events, &mut instance_specs, &mut tx, request).await,
      Some((instance_id, (_, event))) = instance_events.next(), if !instance_events.is_empty() => send_instance_event(&mut tx, instance_id, event).await,
      Some((instance_id, (_, spec))) = instance_specs.next(), if !instance_specs.is_empty() => send_instance_spec(&mut tx, instance_id, spec).await,
      else => break,
    }
  }
}

async fn send_instance_event(tx: &mut mpsc::Sender<RtEvent>, instance_id: String, event: InstanceDriverEvent) {
  if let Err(err) = tx.send(RtEvent::InstanceDriverEvent { instance_id, event }).await {
    warn!(?err, "Failed to send instance event: {err}");
  }
}

async fn send_instance_spec(tx: &mut mpsc::Sender<RtEvent>, instance_id: String, spec: Option<InstanceSpec>) {
  if let Err(err) = tx.send(RtEvent::SetInstanceSpec { instance_id, spec }).await {
    warn!(?err, "Failed to send instance spec: {err}");
  }
}

async fn handle_request(service: &Service,
                        events: &mut InstanceEventsMap,
                        specs: &mut InstanceSpecsMap,
                        tx: &mut mpsc::Sender<RtEvent>,
                        RtRequest { request_id, command }: RtRequest) {
  let response = match command {
    | RtCommand::SetInstancePowerControl { instance_id, power } => {
      let success = service.set_instance_power_control(&instance_id, power).await.is_ok();
      RtEvent::SetInstancePowerControl { request_id, success }
    }
    | RtCommand::SetInstancePlayControl { instance_id, play } => {
      let success = service.set_instance_play_control(&instance_id, play).await.is_ok();
      RtEvent::SetInstancePlayControl { request_id, success }
    }
    | RtCommand::SetInstanceParameters(req) => {
      let response = service.set_instance_parameters(&req.instance_id, req.changes).await;
      let response = response.unwrap_or(SetInstanceParameterResponse::RpcFailure);
      RtEvent::SetInstanceParameters { request_id, response }
    }
    | RtCommand::SubscribeToInstanceEvents { instance_id } => {
      events.insert(instance_id.clone(), service.subscribe_to_instance_events(&instance_id));
      specs.insert(instance_id.clone(), service.watch_instance_specs(&instance_id));
      RtEvent::SubscribeToInstanceEvents { request_id, success: true }
    }
    | RtCommand::UnsubscribeFromInstanceEvents { instance_id } => {
      let events_removed = events.remove(&instance_id);
      let specs_removed = specs.remove(&instance_id);
      let success = events_removed.is_some() || specs_removed.is_some();
      RtEvent::UnsubscribeFromInstanceEvents { request_id, success }
    }
  };

  if let Err(err) = tx.send(response).await {
    warn!(?err, "Failed to send response: {err}");
  }
}
