use std::future::Future;
use std::sync::Arc;
use std::time::Instant;

use dasp::sample::{FromSample, Sample};
use itertools::Itertools;
use tokio::sync::RwLock;

use api::instance::spec::SetParameterCommand;
use api::task::player::{NodeEvent, NodeInfo, PlayHead};

use crate::buffer::{DevicesBuffers, NodeBuffers};

pub type Result<T = ()> = anyhow::Result<T>;

pub mod audio_device;
pub mod buffer;
pub mod bus_node;
pub mod connection;
pub mod events;
pub mod juce;
pub mod player;
pub mod sinks;
pub mod sources;

#[allow(unused_variables)]
pub trait Node: Send + Sync {
  /// Set the parameters of the node
  fn set_parameter(&mut self, parameter: &SetParameterCommand) {}

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
  /// * `devices`: The device buffers for reading and writing
  /// * `io`: The node buffers for reading and writing
  /// * `deadline`: The time at which the node must have finished processing
  fn process(&mut self, play: PlayHead, devices: DevicesBuffers, io: NodeBuffers, deadline: Instant, events: &mut Vec<NodeEvent>) -> Result {
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
pub type SharedBoxedNode = Arc<RwLock<BoxedNode>>;

#[macro_export]
macro_rules! buffers8 {
  ($x:ident) => {
    [&mut $x.buf0[..],
     &mut $x.buf1[..],
     &mut $x.buf2[..],
     &mut $x.buf3[..],
     &mut $x.buf4[..],
     &mut $x.buf5[..],
     &mut $x.buf6[..],
     &mut $x.buf7[..]]
  };
}
