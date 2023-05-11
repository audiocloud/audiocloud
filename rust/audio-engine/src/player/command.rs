use api::task::player::{PlayId, PlayRegion, PlayerControlCommand};
use api::task::{DesiredTaskPlayState, PlayRequest};

use crate::player::{GraphPlayer, PlayerCommandOutcome};
use crate::Result;

impl GraphPlayer {
  pub(crate) fn apply_pending_commands(&mut self) -> Result<PlayerCommandOutcome> {
    let mut outcome = PlayerCommandOutcome::NoAction;

    for command in self.pending_commands.drain(..).collect::<Vec<_>>().into_iter() {
      match command {
        | PlayerControlCommand::SetDesiredPlaybackState { desired } => match desired {
          | DesiredTaskPlayState::Idle => {
            outcome |= self.stop();
          }
          | DesiredTaskPlayState::Play(play) => {
            outcome |= self.start(play);
          }
        },
        | PlayerControlCommand::Seek { play_id, seek_to } =>
          if self.play_head.play_id == PlayId::default() || self.play_head.play_id == play_id {
            self.play_head.position = seek_to;
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

  fn start(&mut self, play: PlayRequest) -> PlayerCommandOutcome {
    self.play_head.play_id = play.play_id;
    self.play_head.position = play.start_from;
    self.play_head.play_region = PlayRegion { start:   play.start,
                                              end:     play.end,
                                              looping: play.looping, };
    PlayerCommandOutcome::Reset
  }

  fn stop(&mut self) -> PlayerCommandOutcome {
    PlayerCommandOutcome::NoAction
  }
}
