use std::collections::HashMap;

use api::task::graph::{SinkId, SinkSpec};
use api::task::player::{PlayId, PlayRegion, PlayerControlCommand};

use crate::player::{GraphPlayer, PlayerCommandOutcome};
use crate::Result;

impl GraphPlayer {
  pub(crate) fn apply_pending_commands(&mut self) -> Result<PlayerCommandOutcome> {
    let mut outcome = PlayerCommandOutcome::NoAction;

    for command in self.pending_commands.drain(..).collect::<Vec<_>>().into_iter() {
      match command {
        | PlayerControlCommand::Play { play_id,
                                       sinks: outputs,
                                       region,
                                       start_from, } => {
          outcome |= self.play(play_id, outputs, region, start_from)?;
        }
        | PlayerControlCommand::Stop { play_id } =>
          if self.play_head.play_id == play_id {
            outcome |= self.stop();
          },
        | PlayerControlCommand::Seek { play_id,
                                       set_position,
                                       set_region, } =>
          if self.play_head.play_id == PlayId::default() || self.play_head.play_id == play_id {
            if let Some(position) = set_position {
              self.play_head.position = position;
            }
            if let Some(region) = set_region {
              self.play_head.play_region = region;
            }

            outcome |= PlayerCommandOutcome::Reset;
          },
        | PlayerControlCommand::ModifyGraph { modifications } => {
          for modification in modifications {
            outcome |= self.apply_graph_modification(modification)?;
          }

          if !matches!(outcome, PlayerCommandOutcome::NoAction) {
            self.sync_all_connections();
          }
        }
      }
    }

    outcome |= self.update_latency()?;

    Ok(outcome)
  }

  fn play(&mut self,
          play_id: PlayId,
          sinks: HashMap<SinkId, SinkSpec>,
          region: PlayRegion,
          start_from: u64)
          -> Result<PlayerCommandOutcome> {
    self.play_head.play_id = play_id;
    self.play_head.position = start_from;
    self.play_head.play_region = region;

    // TODO: Create outputs
    // TODO: Set desired state

    Ok(PlayerCommandOutcome::Reset)
  }

  fn stop(&mut self) -> PlayerCommandOutcome {
    PlayerCommandOutcome::NoAction
  }
}
