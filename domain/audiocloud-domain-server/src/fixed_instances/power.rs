use chrono::Duration;

use audiocloud_api::cloud::domains::DomainPowerInstanceConfig;
use audiocloud_api::{DesiredInstancePowerState, InstancePowerState, ReportInstancePowerState, Timestamped};
use InstancePowerState::*;

use crate::fixed_instances::{NotifyInstancePowerChannelsChanged, SetDesiredPowerChannel};
use crate::tasks::NotifyTaskSpec;
use crate::tracker::RequestTracker;

pub struct Power {
    state:   Timestamped<InstancePowerState>,
    desired: Timestamped<DesiredInstancePowerState>,
    tracker: RequestTracker,
    config:  DomainPowerInstanceConfig,
}

impl Power {
    pub fn new(config: DomainPowerInstanceConfig) -> Self {
        Self { state:   { ShutDown.into() },
               desired: { DesiredInstancePowerState::ShutDown.into() },
               tracker: { Default::default() },
               config:  { config }, }
    }

    pub fn get_power_state(&self) -> ReportInstancePowerState {
        ReportInstancePowerState { actual:  self.state.clone(),
                                   desired: self.desired.clone(), }
    }

    pub fn update(&mut self, spec: &Timestamped<Option<NotifyTaskSpec>>) -> Option<SetDesiredPowerChannel> {
        let idle_off_delay_time = Duration::milliseconds(self.config.idle_off_delay_ms as i64);

        if spec.value().is_some() {
            self.desired = DesiredInstancePowerState::PoweredUp.into();
        } else if spec.elapsed() > idle_off_delay_time {
            self.desired = DesiredInstancePowerState::ShutDown.into();
        }

        if !self.state.value().satisfies(*self.desired.value()) {
            if self.tracker.should_retry() {
                self.tracker.retried();
                let power_up = matches!(self.desired.value(), DesiredInstancePowerState::PoweredUp);

                return Some(SetDesiredPowerChannel { instance_id: { self.config.instance.clone() },
                                                     channel:     { self.config.channel },
                                                     power:       { power_up }, });
            }
        }

        let warm_up_time = Duration::milliseconds(self.config.warm_up_ms as i64);
        let cool_down_time = Duration::milliseconds(self.config.cool_down_ms as i64);

        match self.state.value() {
            PoweringUp if self.state.elapsed() > warm_up_time => self.state = PoweredUp.into(),
            ShuttingDown if self.state.elapsed() > cool_down_time => self.state = ShutDown.into(),
            _ => {}
        };

        None
    }

    pub fn set_desired_state(&mut self, desired: DesiredInstancePowerState) {
        if self.desired.value() != &desired {
            self.desired = desired.into();
        }
    }

    pub fn on_instance_power_channels_changed(&mut self, msg: NotifyInstancePowerChannelsChanged) {
        // check if message is about our power controller
        if &self.config.instance == &msg.instance_id {
            // if so, does it have information about our power channel
            if let Some(power_is_now_up) = msg.power.get(self.config.channel).copied() {
                // are we currently believing we are shut down / powered up
                let was_shut_down = matches!(self.state.value(), ShuttingDown | ShutDown);
                let was_powered_up = !was_shut_down;

                // adjust our belief to new observed facts
                if power_is_now_up && was_shut_down {
                    self.state = PoweringUp.into();
                } else if !power_is_now_up && was_powered_up {
                    self.state = ShuttingDown.into();
                }
            }
        }
    }
}
