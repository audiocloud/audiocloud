use tokio_stream::wrappers::WatchStream;

use api::instance::spec::InstanceSpec;

use crate::nats_utils::Nats;

pub struct InstanceService {
  watch_specs: WatchStream<InstanceSpec>,
}

impl InstanceService {
  pub fn new(buckets: &Nats) -> Self {}
}
