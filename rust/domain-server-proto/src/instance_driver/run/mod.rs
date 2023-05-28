use std::collections::HashMap;

use tokio::sync::mpsc::Receiver;

use api_proto::InstanceSpec;

pub async fn run_drivers(next_config: Receiver<HashMap<String, InstanceSpec>>) {
  let drivers = HashMap::new();
}
