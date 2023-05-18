use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use api::task::spec::TaskSpec;
use api::task::DesiredTaskPlayState;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskData {
  pub id:       Thing,
  pub spec:     TaskSpec,
  pub control:  DesiredTaskPlayState,
  pub state:    (),
  pub revision: u64,
}
