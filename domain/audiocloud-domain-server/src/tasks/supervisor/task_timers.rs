/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use actix::{Actor, AsyncContext, Context};
use actix_broker::BrokerIssue;
use tracing::*;

use audiocloud_api::now;

use crate::o11y;
use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::task::TaskActor;
use crate::tasks::{NotifyTaskActivated, NotifyTaskDeactivated, NotifyTaskDeleted};

impl TasksSupervisor {
    pub(crate) fn register_task_timers(&mut self, ctx: &mut Context<Self>) {
        ctx.run_interval(Duration::from_millis(100), Self::run_task_timers);
    }

    pub(crate) fn run_task_timers(&mut self, ctx: &mut Context<Self>) {
        if self.online {
            self.update_metrics();
            self.drop_inactive_task_actors();
            self.drop_old_tasks();
            self.create_pending_task_actors();
        }
    }

    fn update_metrics(&mut self) {
        o11y::in_context(|ctx| {
            self.num_tasks.observe(ctx, self.tasks.len() as u64, &[]);
            self.num_active_tasks
                .observe(ctx, self.tasks.values().filter(|task| task.actor.is_some()).count() as u64, &[]);
        });
    }

    fn drop_inactive_task_actors(&mut self) {
        let mut deactivated = HashSet::new();
        for (id, task) in &mut self.tasks {
            if task.actor.as_ref().map(|actor| !actor.connected()).unwrap_or(false) {
                debug!(%id, "Dropping task actor due to inactivity");
                deactivated.insert(id.clone());
                task.actor = None;
            }
        }

        for task_id in deactivated {
            self.issue_system_async(NotifyTaskDeactivated { task_id });
        }
    }

    fn drop_old_tasks(&mut self) {
        let mut deleted = HashSet::new();

        let cutoff = now() + chrono::Duration::seconds(self.opts.task_grace_seconds as i64);

        self.tasks.retain(|id, task| {
                      if task.reservations.to < cutoff {
                          deleted.insert(id.clone());
                          debug!(%id, "Cleaning up task from supervisor completely");
                          false
                      } else {
                          true
                      }
                  });

        for task_id in deleted {
            self.issue_system_async(NotifyTaskDeleted { task_id });
        }
    }

    fn create_pending_task_actors(&mut self) {
        // generate an actor map to later assign
        let mut actors = HashMap::new();

        for (task_id, task) in self.tasks.iter() {
            if task.reservations.contains_now() && task.actor.is_none() {
                if let Some(engine_id) = self.allocate_engine(&task_id, &task.spec) {
                    match TaskActor::new(task_id.clone(),
                                         self.opts.clone(),
                                         task.domain_id.clone(),
                                         engine_id.clone(),
                                         task.reservations.clone(),
                                         task.spec.clone(),
                                         task.security.clone(),
                                         self.fixed_instance_routing.clone())
                    {
                        Ok(actor) => {
                            self.issue_system_async(NotifyTaskActivated { task_id: task_id.clone() });
                            actors.insert(task_id.clone(), actor.start());
                        }
                        Err(error) => {
                            warn!(%error, "Failed to start task actor");
                        }
                    }
                } else {
                    warn!(%task_id, "No available audio engines to start task");
                }
            }
        }

        for (id, task) in self.tasks.iter_mut() {
            if let Some(actor) = actors.remove(&id) {
                task.actor.replace(actor);
            }
        }
    }
}
