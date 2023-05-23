pub use instances::v1::*;
pub use media::v1::*;
pub use models::v1::*;
pub use security::v1::*;
pub use tasks::v1::*;

pub mod instances {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/io.audiocloud.instances.v1.rs"));
  }
}

pub mod models {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/io.audiocloud.models.v1.rs"));
  }
}

pub mod media {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/io.audiocloud.media.v1.rs"));
  }
}

pub mod tasks {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/io.audiocloud.tasks.v1.rs"));
  }
}

pub mod security {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/io.audiocloud.security.v1.rs"));
  }
}
