use std::collections::{HashMap, HashSet};

use audiocloud_api::common::instance::DesiredInstancePlayState;
use audiocloud_api::common::task::TaskSpec;
use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_api::Timestamped;

use crate::fixed_instances::{get_instance_supervisor, NotifyInstanceState, SetInstanceDesiredPlayState};
use crate::tracker::RequestTracker;

pub struct TaskFixedInstance {
    state:   Timestamped<NotifyInstanceState>,
    tracker: RequestTracker,
}

impl TaskFixedInstance {
    pub fn new(state_spec: NotifyInstanceState) -> Self {
        Self { state:   Timestamped::new(state_spec),
               tracker: RequestTracker::new(), }
    }

    pub fn reset_request_tracker(&mut self) {
        self.tracker.reset();
    }

    pub fn set_instance_state(&mut self, state: NotifyInstanceState) {
        self.state = Timestamped::new(state);
        self.reset_request_tracker();
    }

    pub fn update(&mut self, instance_id: &FixedInstanceId, play: &DesiredInstancePlayState) -> bool {
        return self.check_power() && self.check_play(instance_id, play);
    }

    pub fn check_power(&self) -> bool {
        if let Some(power) = &self.state.value().power {
            power.actual.value().is_powered_on()
        } else {
            true
        }
    }

    pub fn check_play(&mut self, instance_id: &FixedInstanceId, play: &DesiredInstancePlayState) -> bool {
        if let Some(media) = &self.state.value().play {
            if media.desired.value() != play {
                if self.tracker.should_retry() {
                    get_instance_supervisor().do_send(SetInstanceDesiredPlayState { instance_id: instance_id.clone(),
                                                                                    desired:     play.clone(), });

                    self.tracker.retried();
                }

                false
            } else {
                media.actual.value().satisfies(play)
            }
        } else {
            true
        }
    }
}

pub struct TaskFixedInstances {
    instances: HashMap<FixedInstanceId, TaskFixedInstance>,
    play:      DesiredInstancePlayState,
}

impl Default for TaskFixedInstances {
    fn default() -> Self {
        Self { instances: Default::default(),
               play:      DesiredInstancePlayState::Stopped, }
    }
}

impl TaskFixedInstances {
    pub fn notify_instance_state_changed(&mut self, notify: NotifyInstanceState) {
        match self.instances.get_mut(&notify.instance_id) {
            None => {
                self.instances.insert(notify.instance_id.clone(), TaskFixedInstance::new(notify));
            }
            Some(task_instance) => {
                task_instance.set_instance_state(notify);
            }
        }
    }

    pub fn set_desired_state(&mut self, play: DesiredInstancePlayState) {
        self.play = play;
        for instance in self.instances.values_mut() {
            instance.reset_request_tracker();
        }
    }

    pub fn update(&mut self, session: &TaskSpec) -> bool {
        let mut rv = true;
        for instance_id in session.get_fixed_instance_ids() {
            if let Some(instance) = self.instances.get_mut(&instance_id) {
                rv &= instance.update(instance_id, &self.play);
            } else {
                rv = false;
            }
        }

        rv
    }

    pub fn waiting_for_instances(&self) -> HashSet<FixedInstanceId> {
        let mut rv = HashSet::new();

        for (instance_id, instance) in self.instances.iter() {
            let (power_satisfied, play_satisfied) = is_satisfied(instance);

            if !power_satisfied || !play_satisfied {
                rv.insert(instance_id.clone());
            }
        }

        rv
    }

    pub fn any_waiting(&self) -> bool {
        self.instances.values().any(|i| {
                                   let (power_satisfied, play_satisfied) = is_satisfied(i);
                                   !power_satisfied || !play_satisfied
                               })
    }
}

fn is_satisfied(i: &TaskFixedInstance) -> (bool, bool) {
    let power_satisfied = i.state
                           .value()
                           .power
                           .as_ref()
                           .map(|power| power.actual.value().satisfies(power.desired.value().clone()))
                           .unwrap_or(true);

    let play_satisfied = i.state
                          .value()
                          .play
                          .as_ref()
                          .map(|play| play.actual.value().satisfies(play.desired.value()))
                          .unwrap_or(true);

    (power_satisfied, play_satisfied)
}
