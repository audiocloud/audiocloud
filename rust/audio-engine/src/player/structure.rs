use std::sync::Arc;

use anyhow::anyhow;
use maplit::hashset;
use tokio::sync::RwLock;
use tokio::task::block_in_place;

use api::task::graph::modify::AudioGraphModification;
use api::task::graph::{BusId, BusSpec, DeviceInsertSpec, InputId, InsertId, NodeId, OutputId, SourceId, SourceSpec, VirtualInsertSpec};

use crate::audio_device::audio_device_insert_node::AudioDeviceInsertNode;
use crate::bus_node::BusNode;
use crate::connection::Connection;
use crate::player::GraphPlayer;
use crate::player::PlayerChangeOutcome;
use crate::sources::juce_source_reader_node::JuceSourceReaderNode;
use crate::{Node, Result};

impl GraphPlayer {
  pub(crate) fn apply_pending_structure_changes(&mut self) -> Result<PlayerChangeOutcome> {
    let mut outcome = PlayerChangeOutcome::NoAction;

    while let Some(change) = self.pending_changes.pop_front() {
      match change {
        | AudioGraphModification::AddOrReplaceSource { source_id, source_spec } => {
          outcome |= self.add_source(source_id, source_spec)?;
        }
        | AudioGraphModification::AddOrReplaceDeviceInsert { insert_id, insert_spec } => {
          outcome |= self.add_device_insert(insert_id, insert_spec)?;
        }
        | AudioGraphModification::AddOrReplaceVirtualInsert { insert_id, insert_spec } => {
          outcome |= self.add_virtual_insert(insert_id, insert_spec)?;
        }
        | AudioGraphModification::AddOrReplaceBus { bus_id, bus_spec } => {
          outcome |= self.add_bus(bus_id, bus_spec)?;
        }
        | AudioGraphModification::RemoveSource { source_id } => {
          outcome |= self.remove_source(source_id)?;
        }
        | AudioGraphModification::RemoveDeviceInsert { insert_id } => {
          outcome |= self.remove_device_insert(insert_id)?;
        }
        | AudioGraphModification::RemoveVirtualInsert { insert_id } => {
          outcome |= self.remove_virtual_insert(insert_id)?;
        }
        | AudioGraphModification::RemoveBus { bus_id } => {
          outcome |= self.remove_bus(bus_id)?;
        }
        | AudioGraphModification::Connect { component,
                                            input_channel,
                                            output, } => {
          outcome |= self.connect(component, input_channel, output)?;
        }
        | AudioGraphModification::Disconnect { component,
                                               input_channel,
                                               output, } => {
          outcome |= self.disconnect(component, input_channel, output)?;
        }
      }
    }

    outcome |= self.update_latency()?;

    Ok(outcome)
  }

  pub(crate) async fn reset(&mut self) -> Result {
    let all_devices = self.referenced_device_ids();

    self.unsubscribe_from_devices(&all_devices)?;

    // TODO: rewrite this with spawn_blocking, collect into futures unordered and
    // TODO: yield (node_id, node_info) pairs with which to update &mut self.node_state

    for (node_id, node) in &self.node_apis {
      let Some(state) = self.node_state.get_mut(node_id) else { continue };
      let mut node = node.write().await;

      state.info = node.get_node_info(self.play_head);
      block_in_place(|| node.prepare_to_play(self.play_head, state.accumulated_latency))?;
    }

    for conn in self.connections.values_mut() {
      conn.reset();
    }

    self.subscribe_to_devices(&all_devices)?;

    Ok(())
  }

  fn add_source(&mut self, source_id: SourceId, spec: SourceSpec) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::Source(source_id);
    let path = self.media_resolver.resolve(&spec.media_id)?;
    let node = JuceSourceReaderNode::new(&path, self.play_head, spec.num_channels)?;

    self.node_state.insert(node_id,
                           Self::new_node_state(node_id, &node, self.play_head, hashset! {}, |_| unreachable!(), vec![])?);

    self.node_apis.insert(node_id, Arc::new(RwLock::new(Box::new(node))));

    // adding a source always needs a reset because the player needs to buffer samples
    Ok(PlayerChangeOutcome::NeedReset)
  }

  fn add_device_insert(&mut self, insert_id: InsertId, spec: DeviceInsertSpec) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::DeviceInsert(insert_id);
    let device_attachment = self.device_instance_resolver.resolve(&spec.instance_id)?;
    let device_latency = self.audio_devices.get_latency(&device_attachment.device_id)?;
    let node = AudioDeviceInsertNode::new(&device_attachment, device_latency)?;

    self.node_state.insert(node_id,
                           Self::new_node_state(node_id,
                                                &node,
                                                self.play_head,
                                                hashset! {device_attachment.device_id},
                                                |i| InputId::DeviceInsert(insert_id, i),
                                                spec.inputs)?);
    self.node_apis.insert(node_id, Arc::new(RwLock::new(Box::new(node))));

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn add_virtual_insert(&self, insert_id: InsertId, spec: VirtualInsertSpec) -> Result<PlayerChangeOutcome> {
    Err(anyhow!("Virtual inserts are not yet implemented (ref: {insert_id}, spec: {spec:?})"))
  }

  fn add_bus(&mut self, bus_id: BusId, spec: BusSpec) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::Bus(bus_id);
    let node = BusNode::new(bus_id, spec.inputs.len(), spec.num_outputs)?;

    self.node_state.insert(node_id,
                           Self::new_node_state(node_id,
                                                &node,
                                                self.play_head,
                                                hashset! {},
                                                |i| InputId::Bus(bus_id, i),
                                                spec.inputs)?);

    self.node_apis.insert(node_id, Arc::new(RwLock::new(Box::new(node))));

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn remove_source(&mut self, source: SourceId) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::Source(source);

    self.node_apis.remove(&node_id);
    self.node_state.remove(&node_id);

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn remove_bus(&mut self, bus: BusId) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::Bus(bus);

    self.node_apis.remove(&node_id);
    self.node_state.remove(&node_id);

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn remove_device_insert(&mut self, insert: InsertId) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::DeviceInsert(insert);

    self.node_apis.remove(&node_id);
    self.node_state.remove(&node_id);

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn remove_virtual_insert(&mut self, insert: InsertId) -> Result<PlayerChangeOutcome> {
    let node_id = NodeId::VirtualInsert(insert);

    self.node_apis.remove(&node_id);
    self.node_state.remove(&node_id);

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn connect(&mut self, component: NodeId, input_channel: usize, output_id: OutputId) -> Result<PlayerChangeOutcome> {
    let input_id = component.input(input_channel)?;

    let state = self.node_state
                    .get_mut(&component)
                    .ok_or_else(|| anyhow!("Node not found: {:?}", component))?;

    let inputs = state.node_inputs.entry(input_id).or_default();

    if !inputs.contains(&output_id) {
      inputs.push(output_id);
    }

    state.node_requirements.insert(input_id.into());

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  fn disconnect(&mut self, component: NodeId, input_channels: usize, output_id: OutputId) -> Result<PlayerChangeOutcome> {
    let input_id = component.input(input_channels)?;

    let state = self.node_state
                    .get_mut(&component)
                    .ok_or_else(|| anyhow!("Node not found: {:?}", component))?;

    let inputs = state.node_inputs.entry(input_id).or_default();

    inputs.retain(|&id| id != output_id);

    state.node_requirements.retain(|&required_node_id| {
                             inputs.iter()
                                   .any(|output_id| <OutputId as Into<NodeId>>::into(*output_id) == required_node_id)
                           });

    Ok(PlayerChangeOutcome::ConnectionSync)
  }

  pub(crate) fn sync_all_connections(&mut self) {
    for node_id in self.node_state.keys().copied().collect::<Vec<_>>().into_iter() {
      self.sync_connections(node_id);
    }
  }

  pub fn sync_connections(&mut self, node_id: NodeId) -> PlayerChangeOutcome {
    let state = self.node_state.get(&node_id);

    if let Some(state) = state.as_ref() {
      for (input, outputs) in &state.node_inputs {
        for output in outputs {
          if !self.connections.contains_key(&(*output, *input)) {
            self.connections.insert((*output, *input), Connection::default());
          }
        }
      }
    }

    let num_inputs = state.as_ref().map(|state| state.info.num_inputs).unwrap_or(0);
    let num_outputs = state.as_ref().map(|state| state.info.num_outputs).unwrap_or(0);

    self.connections.retain(|(output, input), _| {
                      let input_node: NodeId = (*input).into();
                      let output_node: NodeId = (*output).into();
                      let input_index = input.channel_index();
                      let output_index = output.channel_index();

                      if input_node == node_id {
                        if input_index >= num_inputs {
                          return false;
                        }

                        // if node is deleted, state is None and the unwrap will remove all connections to it
                        state.as_ref()
                             .map(|state| state.node_inputs.iter().any(|(_, outputs)| outputs.contains(output)))
                             .unwrap_or(false)
                      } else if output_node == node_id {
                        if output_index >= num_outputs {
                          return false;
                        }

                        // if node is deleted, remove all connections from it
                        state.is_some()
                      } else {
                        true
                      }
                    });

    PlayerChangeOutcome::NoAction
  }
}
