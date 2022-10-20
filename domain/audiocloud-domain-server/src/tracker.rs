use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use audiocloud_api::common::time::Timestamp;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum RequestTracker {
    Completed,
    Pending { next_retry: Timestamp },
}

impl RequestTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn complete(&mut self) {
        *self = Self::Completed;
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }

    pub fn should_retry(&self) -> bool {
        matches!(self, Self::Pending { next_retry } if *next_retry >= Utc::now())
    }

    pub fn retried(&mut self) {
        *self = match self {
            RequestTracker::Completed => Self::Completed,
            RequestTracker::Pending { next_retry } => RequestTracker::Pending {
                next_retry: *next_retry + Duration::seconds(1),
            },
        };
    }
}

impl Default for RequestTracker {
    fn default() -> Self {
        Self::Pending {
            next_retry: Utc::now(),
        }
    }
}
