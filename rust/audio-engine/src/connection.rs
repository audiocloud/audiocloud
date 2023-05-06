use std::collections::VecDeque;

use anyhow::{anyhow, bail};

use api::task::graph::{InputId, NodeId, OutputId};

use crate::player::GraphPlayer;
use crate::Result;

#[derive(Debug, Default)]
pub struct Connection {
  pub samples:           VecDeque<f64>,
  pub remaining_latency: usize,
  pub latency:           usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SetLatencyOutcome {
  Ok,
  ConnectionsNeedReset,
}

impl Connection {
  pub fn set_latency(&mut self, new_latency: usize) -> SetLatencyOutcome {
    let already_delayed = self.latency - self.remaining_latency;
    let diff = new_latency as isize - already_delayed as isize;

    // If the difference is negative, this is a forward delay; we have to "catch up".
    if diff < 0 {
      for _ in 0..-diff {
        if self.samples.pop_front().is_none() {
          return SetLatencyOutcome::ConnectionsNeedReset;
        }
      }
    } else {
      self.remaining_latency += diff as usize;
    }

    self.latency = new_latency;

    SetLatencyOutcome::Ok
  }
}

impl GraphPlayer {
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
    for node in self.node_state.values() {
      for (input_id, inputs) in node.node_inputs.iter() {
        for output_id in inputs.iter() {
          edges.push(Edge { from:         *output_id,
                            to:           *input_id,
                            previous:     None,
                            contributed:  None,
                            compensation: None, });
        }
      }
    }

    let mut total_latencies = vec![];
    for (id, node) in self.node_state.iter() {
      total_latencies.push(NodeLatency { id:      *id,
                                         latency: Some(node.info.latency), });
    }

    for edge in &mut edges {
      // TODO: maybe more than edge contributed
      let node_id = edge.from.into();
      edge.contributed = Some(self.node_state
                                  .get(&node_id)
                                  .ok_or_else(|| anyhow!("Node {node_id} not found"))?
                                  .info
                                  .latency).filter(|latency| *latency != 0);
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
      let node = self.node_state.get_mut(&node.id).unwrap();
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

  fn set_delay(&mut self, from: OutputId, to: InputId, latency: usize) -> Result<SetLatencyOutcome> {
    let Some(connection) = self.connections.get_mut(&(from, to)) else { bail!("Connection {from} -> {to} does not exist") };
    Ok(connection.set_latency(latency))
  }
}
