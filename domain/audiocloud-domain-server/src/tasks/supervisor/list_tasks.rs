use actix::Handler;

use audiocloud_api::domain::tasks::{TaskSummary, TaskSummaryList};

use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::ListTasks;

impl Handler<ListTasks> for TasksSupervisor {
    type Result = TaskSummaryList;

    fn handle(&mut self, msg: ListTasks, ctx: &mut Self::Context) -> Self::Result {
        let mut rv = vec![];
        for (id, task) in &self.tasks {
            // TODO: missing `waiting_for_instances` and `waiting_for_media`
            // TODO: would be nice if TaskSummary included timestamps
            rv.push(TaskSummary { task_id:               { id.clone() },
                                  play_state:            { task.state.play_state.get_ref().clone() },
                                  waiting_for_instances: { Default::default() },
                                  waiting_for_media:     { Default::default() }, });
        }

        rv
    }
}
