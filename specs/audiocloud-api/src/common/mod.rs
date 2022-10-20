use serde::de::DeserializeOwned;
use serde::Serialize;

pub use change::*;
pub use change::*;
pub use error::*;
pub use instance::*;
pub use media::*;
pub use model::*;
pub use newtypes::*;
pub use task::*;
pub use time::*;

pub mod change;
pub mod error;
pub mod instance;
pub mod media;
pub mod model;
pub mod newtypes;
pub mod task;
pub mod time;

/// A request that has an associated response type
pub trait Request: Serialize {
    type Response: DeserializeOwned;
}
