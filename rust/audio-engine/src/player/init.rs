use std::collections::{HashMap, HashSet};

use nanoid::nanoid;
use tokio::sync::mpsc;

use api::task::graph::modify::AudioGraphModification;
use api::task::graph::{AudioGraphSpec, InputId, NodeId, OutputId};
use api::task::player::{GraphPlayerEvent, PlayHead};

use crate::audio_device::AudioDevices;
use crate::buffer::NodeBuffers;
use crate::player::work_set::WorkSet;
use crate::player::{
  BoxedDeviceInstanceResolver, BoxedMediaResolver, GraphPlayer, PlayerControlCommand, PlayerNodeState, PlayerParameterCommand,
};
use crate::{Node, Result};

impl GraphPlayer {
  pub fn new(devices: AudioDevices,
             use_media_resolver: BoxedMediaResolver,
             use_device_instance_resolver: BoxedDeviceInstanceResolver,
             spec: AudioGraphSpec,
             rx_control_ch: mpsc::Receiver<PlayerControlCommand>,
             rx_params_ch: mpsc::Receiver<PlayerParameterCommand>,
             tx_events_ch: mpsc::Sender<GraphPlayerEvent>)
             -> Result<Self> {
    let (tx_device_ch, rx_device_ch) = mpsc::channel(0xff);
    let (tx_tasks_ch, rx_tasks_ch) = mpsc::channel(0xff);
    let play_head = PlayHead::default();
    let work_set = WorkSet::from(play_head);

    let mut rv = Self { client_id:                nanoid!(),
                        specs:                    Default::default(),
                        node_apis:                Default::default(),
                        connections:              Default::default(),
                        rx_control:               rx_control_ch,
                        tx_device:                tx_device_ch,
                        rx_device:                rx_device_ch,
                        rx_params:                rx_params_ch,
                        tx_tasks:                 tx_tasks_ch,
                        rx_tasks:                 rx_tasks_ch,
                        tx_events:                tx_events_ch,
                        play_head:                Default::default(),
                        node_state:               Default::default(),
                        audio_devices:            devices,
                        current_work_set:         work_set,
                        partial_work_sets:        Default::default(),
                        pending_changes:          Default::default(),
                        media_resolver:           use_media_resolver,
                        device_instance_resolver: use_device_instance_resolver, };

    for (source_id, source_spec) in spec.sources {
      rv.pending_changes
        .push_back(AudioGraphModification::AddOrReplaceSource { source_id, source_spec });
    }
    for (bus_id, bus_spec) in spec.busses {
      rv.pending_changes
        .push_back(AudioGraphModification::AddOrReplaceBus { bus_id, bus_spec });
    }
    for (insert_id, insert_spec) in spec.device_inserts {
      rv.pending_changes
        .push_back(AudioGraphModification::AddOrReplaceDeviceInsert { insert_id, insert_spec });
    }
    for (insert_id, insert_spec) in spec.virtual_inserts {
      rv.pending_changes
        .push_back(AudioGraphModification::AddOrReplaceVirtualInsert { insert_id, insert_spec });
    }

    rv.apply_pending_structure_changes()?;

    Ok(rv)
  }

  pub(crate) fn new_node_state<N, GenInputId>(node_id: NodeId,
                                              node: &N,
                                              play_head: PlayHead,
                                              device_requirements: HashSet<String>,
                                              gen_input_ids: GenInputId,
                                              inputs: Vec<Vec<OutputId>>)
                                              -> Result<PlayerNodeState>
    where N: Node,
          GenInputId: Fn(usize) -> InputId
  {
    let node_info = node.get_node_info(play_head);
    let node_buffers = NodeBuffers::allocate(&node_info, play_head.buffer_size as usize);

    let (inputs, requirements) = Self::unwrap_inputs(inputs, gen_input_ids);

    Ok(PlayerNodeState { id:                        node_id,
                         info:                      node_info,
                         processing:                None,
                         buffers:                   node_buffers,
                         audio_device_requirements: device_requirements,
                         node_requirements:         requirements,
                         node_inputs:               inputs,
                         accumulated_latency:       0, })
  }

  pub(crate) fn unwrap_inputs(inputs: Vec<Vec<OutputId>>,
                              gen_input_id: impl Fn(usize) -> InputId)
                              -> (HashMap<InputId, Vec<OutputId>>, HashSet<NodeId>) {
    let mut ret_inputs: HashMap<InputId, Vec<OutputId>> = HashMap::new();
    let mut ret_requirements = HashSet::new();

    for (i, outputs) in inputs.into_iter().enumerate() {
      let input_id = gen_input_id(i);
      let mut target = ret_inputs.entry(input_id).or_default();

      for output_id in outputs {
        target.push(output_id);
        ret_requirements.insert(output_id.into());
      }
    }

    (ret_inputs, ret_requirements)
  }
}
