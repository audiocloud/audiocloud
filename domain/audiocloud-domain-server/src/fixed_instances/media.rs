use audiocloud_api::cloud::domains::DomainMediaInstanceConfig;
use audiocloud_api::instance_driver::InstanceDriverCommand;
use audiocloud_api::{
    DesiredInstancePlayState, InstancePlayState, ReportInstancePlayState, Timestamped,
};

use crate::tracker::RequestTracker;

pub struct Media {
    state: Timestamped<InstancePlayState>,
    desired: Timestamped<DesiredInstancePlayState>,
    position: Timestamped<Option<f64>>,
    config: DomainMediaInstanceConfig,
    tracker: RequestTracker,
}

impl Media {
    pub fn new(config: DomainMediaInstanceConfig) -> Self {
        Self {
            desired: { Timestamped::new(DesiredInstancePlayState::Stopped) },
            state: { Timestamped::new(InstancePlayState::Stopped) },
            position: { Timestamped::new(None) },
            tracker: { Default::default() },
            config: { config },
        }
    }

    pub fn get_play_state(&self) -> ReportInstancePlayState {
        ReportInstancePlayState {
            desired: self.desired.clone(),
            actual: self.state.clone(),
            media: self.position.clone(),
        }
    }

    pub fn update(&mut self) -> Option<InstanceDriverCommand> {
        if !self.state.value().satisfies(self.desired.value()) {
            if self.tracker.should_retry() {
                self.tracker.retried();
                return Some(self.desired.value().clone().into());
            }
        }

        None
    }

    pub fn set_desired_state(&mut self, desired: DesiredInstancePlayState) {
        if self.desired.value() != &desired {
            self.desired = Timestamped::new(desired);
        }
    }

    pub fn on_instance_play_state_changed(
        &mut self,
        state: InstancePlayState,
        position: Option<f64>,
    ) {
        self.state = state.into();
        self.position = Timestamped::new(position);
    }
}
