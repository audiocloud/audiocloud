/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use chrono::Duration;

use audiocloud_api::cloud::domains::DomainPowerInstanceConfig;
use audiocloud_api::{
    DesiredInstancePowerState, FixedInstanceId, InstancePowerState, ModelValue, ParameterId, ReportInstancePowerState, Timestamped,
};
use InstancePowerState::*;

use crate::fixed_instances::{MergeInstanceParameters, NotifyFixedInstanceReports};
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

    pub fn power_instance_id(&self) -> &FixedInstanceId {
        &self.config.instance
    }

    pub fn get_power_state(&self) -> ReportInstancePowerState {
        ReportInstancePowerState { actual:  self.state.clone(),
                                   desired: self.desired.clone(), }
    }

    pub fn update(&mut self, spec: &Timestamped<Option<NotifyTaskSpec>>) -> Option<MergeInstanceParameters> {
        let idle_off_delay_time = Duration::milliseconds(self.config.idle_off_delay_ms as i64);

        if spec.get_ref().is_some() {
            self.desired = DesiredInstancePowerState::PoweredUp.into();
        } else if spec.elapsed() > idle_off_delay_time {
            self.desired = DesiredInstancePowerState::ShutDown.into();
        }

        if !self.state.get_ref().satisfies(*self.desired.get_ref()) {
            if self.tracker.should_retry() {
                self.tracker.retried();
                let power_up = matches!(self.desired.get_ref(), DesiredInstancePowerState::PoweredUp);

                return Some(MergeInstanceParameters { instance_id: { self.config.instance.clone() },
                                                      channel:     { self.config.channel },
                                                      parameter:   { ParameterId::new("power".to_owned()) },
                                                      value:       { ModelValue::Bool(power_up) }, });
            }
        }

        let warm_up_time = Duration::milliseconds(self.config.warm_up_ms as i64);
        let cool_down_time = Duration::milliseconds(self.config.cool_down_ms as i64);

        match self.state.get_ref() {
            PoweringUp if self.state.elapsed() > warm_up_time => self.state = PoweredUp.into(),
            ShuttingDown if self.state.elapsed() > cool_down_time => self.state = ShutDown.into(),
            _ => {}
        };

        None
    }

    pub fn set_desired_state(&mut self, desired: DesiredInstancePowerState) {
        if self.desired.get_ref() != &desired {
            self.desired = desired.into();
        }
    }

    pub fn on_instance_power_channels_changed(&mut self, msg: NotifyFixedInstanceReports) {
        if let Some(power_is_now_up) = msg.reports
                                          .get("power")
                                          .and_then(|value| value.as_array())
                                          .and_then(|power| power.get(self.config.channel))
                                          .and_then(|power| power.as_bool())
        {
            // are we currently believing we are shut down / powered up
            let was_shut_down = matches!(self.state.get_ref(), ShuttingDown | ShutDown);
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
