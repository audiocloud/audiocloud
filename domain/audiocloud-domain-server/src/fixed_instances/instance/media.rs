use audiocloud_api::cloud::domains::DomainMediaInstanceConfig;

use audiocloud_api::{DesiredInstancePlayState, InstancePlayState, ReportInstancePlayState, Timestamped};

use crate::remote_value::RemoteValue;
use crate::tracker::RequestTracker;

pub struct Media {
    actual:   Timestamped<InstancePlayState>,
    desired:  RemoteValue<DesiredInstancePlayState>,
    position: Timestamped<Option<f64>>,
    config:   DomainMediaInstanceConfig,
    tracker:  RequestTracker,
}

impl Media {
    pub fn new(config: DomainMediaInstanceConfig) -> Self {
        let desired = DesiredInstancePlayState::Stopped { position: None };
        let actual = InstancePlayState::Stopped { position: None };

        Self { desired:  { RemoteValue::new(desired) },
               actual:   { Timestamped::new(actual) },
               position: { Timestamped::new(None) },
               tracker:  { Default::default() },
               config:   { config }, }
    }

    pub fn get_play_state(&self) -> ReportInstancePlayState {
        ReportInstancePlayState { desired: self.desired.timestamped().clone(),
                                  actual:  self.actual.clone(),
                                  media:   self.position.clone(), }
    }

    pub fn update(&mut self) -> Option<(u64, DesiredInstancePlayState)> {
        if !self.actual.get_ref().satisfies(self.desired.get_ref()) {
            if self.tracker.should_retry() {
                self.tracker.retried();
            }
        }

        return self.desired.start_update();
    }

    pub fn finish_update(&mut self, remote: u64, successful: bool) {
        self.desired.finish_update(remote, successful);
    }

    pub fn set_desired_state(&mut self, desired: DesiredInstancePlayState) {
        if self.desired.get_ref() != &desired {
            self.desired.set(desired);
        }
    }

    pub fn on_instance_play_state_changed(&mut self, state: InstancePlayState, position: Option<f64>) {
        self.actual = state.into();
        self.position = Timestamped::new(position);
    }
}
