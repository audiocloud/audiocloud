use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;
use futures::StreamExt;
use tokio::task::JoinHandle;
use tokio::time::Interval;
use tokio::{select, spawn};

use api::task::spec::TaskSpec;
use api::task::subjects::{get_task_list, set_task_graph};
use api::task::{GetTaskListRequest, GetTaskListResponse, SetTaskGraphRequest, SetTaskGraphResponse, TaskSummary};

use crate::nats::{Nats, RequestStream, WatchStream};
use crate::tasks::run::RunDomainTask;
use crate::tasks::Result;

pub struct TasksServer {
  host_id:        String,
  get_task_list:  RequestStream<GetTaskListRequest, GetTaskListResponse>,
  set_task_graph: RequestStream<SetTaskGraphRequest, SetTaskGraphResponse>,
  watch_specs:    WatchStream<TaskSpec>,
  tasks:          HashMap<String, Task>,
  timer:          Interval,
  nats:           Nats,
}

impl TasksServer {
  pub fn new(nats: Nats, host_id: String) -> Self {
    let watch_specs = nats.task_spec.watch_all();
    let timer = tokio::time::interval(Duration::from_secs(1));

    let get_task_list = nats.serve_requests(get_task_list());
    let set_task_graph = nats.serve_requests(set_task_graph());

    let tasks = HashMap::new();

    Self { host_id,
           get_task_list,
           set_task_graph,
           watch_specs,
           tasks,
           timer,
           nats }
  }

  pub async fn run(mut self) -> Result {
    loop {
      select! {
        Some((task_id, maybe_task_spec)) = self.watch_specs.next() => {
          self.task_spec_changed(task_id, maybe_task_spec);
        },
        Some((_, request, reply)) = self.get_task_list.next() => {
          let _ = reply.send(self.get_task_list(request));
        },
        Some((_, request, reply)) = self.set_task_graph.next() => {
          let _ = reply.send(self.set_task_graph(request));
        },
        _ = self.timer.tick() => {
          self.update_tasks();
        }
      }
    }

    Ok(())
  }

  fn task_spec_changed(&mut self, task_id: String, maybe_task_spec: Option<TaskSpec>) {
    self.tasks.entry(task_id).or_default().spec = maybe_task_spec;
    self.update_tasks();
  }

  fn update_tasks(&mut self) {
    self.cleanup_stale_tasks();
    self.start_pending_tasks();
  }

  fn start_pending_tasks(&mut self) {
    for (task_id, task) in &mut self.tasks {
      let Some(spec) = task.spec.as_ref() else { continue; };
      if spec.from > Utc::now() {
        continue;
      }
      if spec.host_id != self.host_id {
        continue;
      }

      if task.handle.as_ref().map(|task| task.is_finished()).unwrap_or(true) {
        let mut domain_task = RunDomainTask::new(task_id.clone(), spec.clone(), self.nats.clone());
        task.handle = Some(spawn(async move { domain_task.run().await }));
      }
    }
  }

  fn cleanup_stale_tasks(&mut self) {
    self.tasks.retain(|_, task| match task.spec.as_ref() {
                | None => false,
                | Some(spec) => spec.to > Utc::now(),
              });
  }

  fn get_task_list(&mut self, request: GetTaskListRequest) -> GetTaskListResponse {
    GetTaskListResponse { tasks: self.tasks.iter().map(|(id, spec)| (id.clone(), TaskSummary {})).collect(), }
  }

  fn set_task_graph(&mut self, request: SetTaskGraphRequest) -> SetTaskGraphResponse {
    SetTaskGraphResponse::Success
  }
}

#[derive(Default)]
struct Task {
  spec:   Option<TaskSpec>,
  handle: Option<JoinHandle<Result>>,
}
