use std::collections::HashMap;
use std::iter::from_fn;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail};
use dasp_sample::{FromSample, Sample};
use itertools::Itertools;
use rayon::prelude::*;
use rtrb::{Consumer, Producer};

use api::task::graph::{AudioGraphSpec, InputId, NodeId, OutputId};
use api::task::player::PlayHead;
use buffer::DeviceBuffers;

pub type Result<T = ()> = anyhow::Result<T>;

pub mod buffer;
pub mod bus_node;
pub mod juce;
pub mod juce_source_reader_node;

pub struct ConnectionEnd {
  pub consumer:          Consumer<f64>,
  pub latency:           usize,
  pub remaining_latency: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SetLatencyOutcome {
  Ok,
  ConnectionsNeedReset,
}

impl ConnectionEnd {
  pub fn set_delay(&mut self, new_latency: usize) -> SetLatencyOutcome {
    let already_delayed = self.latency - self.remaining_latency;
    let diff = new_latency as isize - already_delayed as isize;

    // If the difference is negative, this is a forward delay; we have to "catch up".
    if diff < 0 {
      for _ in 0..-diff {
        if self.consumer.pop().is_err() {
          return SetLatencyOutcome::ConnectionsNeedReset;
        }
      }
    } else {
      self.remaining_latency += diff as usize;
    }

    self.latency = new_latency;

    SetLatencyOutcome::Ok
  }

  pub fn pull(&mut self, len: usize) -> impl Iterator<Item = f64> + '_ {
    std::iter::repeat(0.0).take(self.remaining_latency)
                          .inspect(|_| self.remaining_latency -= 1)
                          .chain(from_fn(|| self.consumer.pop().ok()))
                          .take(len)
  }
}

pub struct ConnectionStart {
  pub producer: Producer<f64>,
}

#[derive(Default)]
pub struct InputPad {
  pub receives: HashMap<OutputId, ConnectionEnd>,
}

#[derive(Default)]
pub struct OutputPad {
  pub sends: HashMap<InputId, ConnectionStart>,
}

impl OutputPad {
  pub fn send<S>(&mut self, source: &[S])
    where f64: FromSample<S>,
          S: Copy
  {
    for (_, start) in self.sends.iter_mut() {
      let Ok(chunk) = start.producer.write_chunk_uninit(source.len()) else { continue };
      chunk.fill_from_iter(source.iter().map(|s| f64::from_sample(*s)));
    }
  }
}

pub type InputPads = HashMap<InputId, InputPad>;

pub type OutputPads = HashMap<OutputId, OutputPad>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct NodeInfo {
  pub latency:     usize,
  pub num_inputs:  usize,
  pub num_outputs: usize,
}

#[allow(unused_variables)]
pub trait Node: Send {
  /// Returns the linking information about the node - latency, number of inputs and outputs
  ///
  /// # Parameters
  /// * `play`: The play head position and play session information
  fn get_node_info(&self, play: PlayHead) -> NodeInfo {
    NodeInfo::default()
  }

  /// Called when the node must prepare to play.
  ///
  /// # Parameters
  /// * `play`: The play head position and play session information
  /// * `accumulated_latency`: The latency of all nodes before this one
  fn prepare_to_play(&mut self, play: PlayHead, accumulated_latency: usize) -> Result {
    Ok(())
  }

  /// Called when the node inputs are ready for reading
  ///
  /// # Parameters
  /// * `play`: The play head position and play session information
  /// * `inputs`: The input buffers, the node will read from these
  /// * `outputs`: The output buffers, the node will write to thesež
  /// * `deadline`: The time at which the node must have finished processing
  fn process(&mut self,
             play: PlayHead,
             device_buffers: DeviceBuffers,
             inputs_buffers: &[&[f64]],
             outputs: &mut [&mut [f64]],
             deadline: Instant)
             -> Result {
    Ok(())
  }

  /// Called when the node will no longer be played and a new [prepare_to_play] will be called
  ///
  /// # Parameters
  /// * `play`: The play head position and play session information
  fn stop(&mut self, play: PlayHead) -> Result {
    Ok(())
  }
}

pub type BoxedNode = Box<dyn Node>;

struct PlayerNode {
  id:                  NodeId,
  node:                BoxedNode,
  info:                NodeInfo,
  accumulated_latency: usize,
  must_prepare:        bool,
  inputs:              InputPads,
  outputs:             OutputPads,
  input_bufs:          Vec<Vec<f64>>,
  output_bufs:         Vec<Vec<f64>>,
  generation:          u64,
  rank:                i32,
}

pub struct GraphPlayer {
  nodes:            HashMap<NodeId, PlayerNode>,
  play_head:        PlayHead,
  must_prepare:     bool,
  ready_to_iterate: bool,
}

impl GraphPlayer {
  pub fn iterate_generation(&mut self, device_buffers: DeviceBuffers, deadline: Instant) -> Result {
    let play_head = self.play_head.advance_position();
    let max_duration = Instant::now() - deadline;

    let now = Instant::now();
    self.update_info_and_prepare(play_head)?;

    if now.elapsed() > max_duration / 10 {
      // our preparing phase went for way too long, we'll skip this generation
      return Ok(());
    }

    self.process(play_head, device_buffers, deadline)?;

    Ok(())
  }

  fn on_structure_change(&mut self) -> anyhow::Result<()> {
    self.assign_node_ranks();
    if self.update_latency()? == SetLatencyOutcome::ConnectionsNeedReset {
      self.reset_connections();
      self.must_prepare = true;
    }

    Ok(())
  }

  fn reset_connections(&mut self) {
    for node in self.nodes.values_mut() {
      for input in node.inputs.values_mut() {
        for ConnectionEnd { consumer, .. } in input.receives.values_mut() {
          let slots = consumer.slots();
          if slots > 0 {
            consumer.read_chunk(slots).unwrap().commit_all();
          }
        }
      }
    }
  }

  fn update_latency(&mut self) -> anyhow::Result<SetLatencyOutcome> {
    #[derive(Debug)]
    struct Edge {
      from:         OutputId,
      to:           InputId,
      previous:     Option<usize>,
      contributed:  Option<usize>,
      compensation: Option<usize>,
    }

    #[derive(Debug)]
    struct NodeLatency {
      id:      NodeId,
      latency: Option<usize>,
    }

    let mut edges = vec![];
    for node in self.nodes.values() {
      for (input_id, inputs) in node.inputs.iter() {
        for (output_id, _) in inputs.receives.iter() {
          edges.push(Edge { from:         *output_id,
                            to:           *input_id,
                            previous:     None,
                            contributed:  None,
                            compensation: None, });
        }
      }
    }

    let mut total_latencies = vec![];
    for (id, node) in self.nodes.iter_mut() {
      node.info = node.node.get_node_info(self.play_head);
      total_latencies.push(NodeLatency { id:      *id,
                                         latency: None, });
    }

    for edge in &mut edges {
      match edge.from {
        | OutputId::Source(_, _) => {
          edge.previous = Some(0);
          edge.contributed = Some(0);
        }
        | OutputId::Insert(id, _) => {
          edge.contributed = Some(self.nodes
                                      .get(&NodeId::Insert(id))
                                      .ok_or_else(|| anyhow!("insert {id} not found"))?
                                      .info
                                      .latency);
        }
        | OutputId::Bus(_, _) => {
          edge.contributed = Some(0);
        }
      }
    }

    'outer: loop {
      for next_node in total_latencies.iter_mut().filter(|n| n.latency.is_none()) {
        let id = next_node.id;

        let filter = |e: &&Edge| Into::<NodeId>::into(e.to) == id;
        let filter_mut = |e: &&mut Edge| Into::<NodeId>::into(e.to) == id;

        if edges.iter().filter(filter).any(|e| e.previous.is_none() || e.contributed.is_none()) {
          continue;
        }

        let max_latency = edges.iter()
                               .filter(filter)
                               .map(|e| e.previous.unwrap() + e.contributed.unwrap())
                               .max()
                               .ok_or_else(|| anyhow!("Node has no input edges"))?;

        for edge in edges.iter_mut().filter(filter_mut) {
          let edge_latency = edge.previous.unwrap_or_default() + edge.contributed.unwrap_or_default();
          edge.compensation = Some(max_latency - edge_latency);
        }

        next_node.latency = Some(max_latency);

        for edge in edges.iter_mut().filter(|e| Into::<NodeId>::into(e.from) == id) {
          edge.previous = Some(max_latency);
        }

        continue 'outer;
      }

      break;
    }

    for node in total_latencies {
      let latency = node.latency.unwrap_or_default();
      let node = self.nodes.get_mut(&node.id).unwrap();
      if node.accumulated_latency != latency {
        node.accumulated_latency = latency;
      }
    }

    let mut rv = SetLatencyOutcome::Ok;
    for edge in edges {
      if self.set_delay(edge.from, edge.to, edge.compensation.unwrap_or_default())? == SetLatencyOutcome::ConnectionsNeedReset {
        rv = SetLatencyOutcome::ConnectionsNeedReset;
      }
    }

    Ok(rv)
  }

  fn set_delay(&mut self, from: OutputId, to: InputId, latency: usize) -> anyhow::Result<SetLatencyOutcome> {
    let node_id: NodeId = to.into();
    let node = self.nodes.get_mut(&node_id).ok_or_else(|| anyhow!("Node {node_id} not found"))?;

    Ok(node.inputs
           .get_mut(&to)
           .ok_or_else(|| anyhow!("Input {to} not found"))?
           .receives
           .get_mut(&from)
           .ok_or_else(|| anyhow!("Output {from} not found"))?
           .set_delay(latency))
  }

  fn update_info_and_prepare(&mut self, play_head: PlayHead) -> Result {
    let mut at_least_one_changed = false;

    // get all node info
    for value in self.nodes.values_mut() {
      let info = value.node.get_node_info(play_head);
      let must_prepare = if &info != &value.info {
        value.info = info;
        true
      } else {
        false
      };

      at_least_one_changed = at_least_one_changed || must_prepare;
      value.must_prepare = self.must_prepare || must_prepare;
    }

    if at_least_one_changed {
      self.prepare_to_play(play_head)?;
    }

    Ok(())
  }

  fn prepare_to_play(&mut self, play_head: PlayHead) -> Result {
    for (id, err) in self.nodes
                         .par_iter_mut()
                         .map(|(_, n)| (n.id, n.node.prepare_to_play(play_head, n.accumulated_latency)))
                         .find_map_first(|(id, result)| result.err().map(|err| (id, err)))
    {
      bail!("Node {id} failed to prepare: {err}");
    }

    self.must_prepare = false;

    Ok(())
  }

  fn assign_node_ranks(&mut self) {
    self.nodes
        .values_mut()
        .for_each(|n| n.rank = if n.inputs.is_empty() { 0 } else { -1 });

    fn get_node_rank(nodes: &HashMap<NodeId, PlayerNode>, node: &PlayerNode) -> i32 {
      let mut rank = -1;

      for pad in node.inputs.values() {
        for out_id in pad.receives.keys() {
          if let Some(n) = nodes.get(&(*out_id).into()) {
            rank = rank.max(get_node_rank(nodes, n));
          }
        }
      }

      rank + 1
    }

    for (id, rank) in self.nodes
                          .iter()
                          .map(|(id, n)| (*id, get_node_rank(&self.nodes, n)))
                          .collect::<Vec<_>>()
                          .into_iter()
    {
      self.nodes.get_mut(&id).unwrap().rank = rank;
    }
  }

  fn process(&mut self, play_head: PlayHead, device_buffers: DeviceBuffers, deadline: Instant) -> Result {
    // we are assuming that latency and rank are already prepared here
    for (rank, groups) in self.nodes
                              .values_mut()
                              .group_by(|n| n.rank)
                              .into_iter()
                              .sorted_by_key(|(rank, _)| *rank)
    {
      let mut groups = groups.collect_vec();

      // without spawn() this is not going to be multi-threaded.
      for (id, err) in groups.par_iter_mut()
                             .map(|mut n| {
                               n.generation = play_head.generation;
                               let id = n.id.clone();

                               // TODO: collect to a SmallVec<32>

                               let inputs = n.input_bufs.iter().map(|s| s.as_slice()).collect_vec();
                               let mut outputs = n.output_bufs.iter_mut().map(|s| s.as_mut_slice()).collect_vec();

                               let result = n.node.process(play_head, device_buffers, &inputs[..], &mut outputs[..], deadline);
                               (id, result)
                             })
                             .find_map_first(|(id, res)| res.err().map(|err| (id, err)))
      {
        bail!("Rank {rank} node {id} failed to process: {err}");
      }

      if Instant::now() >= deadline {
        bail!("Player took too long to process rank {rank}");
      }
    }

    Ok(())
  }
}

/// Audio Engine handle to be shared between a device and the system controlling the players
pub struct AudioEngine {
  pub players:     RwLock<HashMap<String, GraphPlayer>>,
  pub sample_rate: u32,
}

impl AudioEngine {
  pub fn new(sample_rate: u32) -> Self {
    let players = Default::default();

    Self { players, sample_rate }
  }
}

impl AudioEngine {
  pub fn with_player<R, F>(&self, id: &str, something: F) -> anyhow::Result<R>
    where F: FnOnce(&GraphPlayer) -> anyhow::Result<R>
  {
    let players = self.players.read().expect("Failed to lock players for reading");
    match players.get(id) {
      | Some(player) => something(player),
      | None => bail!("Player {id} not found"),
    }
  }

  pub fn with_player_mut<R, F>(&self, id: &str, something: F) -> anyhow::Result<R>
    where F: FnOnce(&mut GraphPlayer) -> anyhow::Result<R>
  {
    let mut players = self.players.write().expect("Failed to lock players for writing");
    match players.get_mut(id) {
      | Some(player) => something(player),
      | None => bail!("Player {id} not found"),
    }
  }

  pub fn add_player(&self, id: &str, player: GraphPlayer) -> anyhow::Result<()> {
    let mut players = self.players.write().expect("Failed to lock players for writing");
    if players.contains_key(id) {
      bail!("Player {id} already exists");
    }

    players.insert(id.to_string(), player);

    Ok(())
  }

  pub fn remove_player(&self, id: &str) -> anyhow::Result<()> {
    let mut players = self.players.write().expect("Failed to lock players for writing");

    players.remove(id);

    Ok(())
  }

  pub fn on_device_callback(&self, buffers: DeviceBuffers) -> anyhow::Result<()> {
    let duration_in_us = (buffers.buffer_size as f64 / self.sample_rate as f64 * 750_000.0).floor() as u64;
    let deadline = Instant::now() + Duration::from_micros(duration_in_us);
    let mut players = self.players.write().expect("Failed to lock players for writing");

    for (id, err) in players.par_iter_mut()
                            .filter(|(_, player)| player.ready_to_iterate)
                            .map(|(id, player)| (id.clone(), player.iterate_generation(buffers, deadline)))
                            .find_map_first(|(id, res)| res.err().map(|err| (id, anyhow::Error::from(err))))
    {
      bail!("Player {id} failed to process: {err}");
    }

    Ok(())
  }
}

// TODO: how do we generate Nodes for Players? That's up to the user of the library
pub struct GraphPlayerHandle<NR: NodeResolver, MR: MediaResolver> {
  pub player_id:      String,
  pub graph_id:       String,
  pub app_id:         String,
  pub spec:           AudioGraphSpec,
  pub engine:         Arc<AudioEngine>,
  pub node_resolver:  NR,
  pub media_resolver: MR,
}

pub trait NodeResolver {
  fn resolve_instance(&mut self, app_id: &str, graph_id: &str, node_id: NodeId, instance_id: &str) -> anyhow::Result<BoxedNode>;
}

pub trait MediaResolver {
  fn resolve(&mut self, app_id: &str, graph_id: &str, node_id: NodeId, media_id: &str) -> anyhow::Result<MediaInfo>;
}

pub struct MediaInfo {}
