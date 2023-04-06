use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use api::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Timestamped<T> {
  time:  Timestamp,
  value: T,
}

impl<T> Timestamped<T> {
  pub fn now(value: T) -> Self {
    let time = Utc::now();

    Self { time, value }
  }
}

impl<T> Default for Timestamped<T> where T: Default
{
  fn default() -> Self {
    Self { time:  Utc::now(),
           value: Default::default(), }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RequestTracker<Desired, Actual> {
  desired:           Timestamped<Desired>,
  actual:            Timestamped<Actual>,
  last_request_at:   Option<Timestamp>,
  next_request_at:   Timestamp,
  retry_interval_ms: u32,
}

impl<Desired, Actual> RequestTracker<Desired, Actual> where Desired: Clone
{
  pub fn get_desired(&self) -> Desired {
    self.desired.value.clone()
  }
}

impl<Desired, Actual> RequestTracker<Desired, Actual> where Actual: Clone
{
  pub fn get_actual(&self) -> Actual {
    self.actual.value.clone()
  }
}

impl<Desired, Actual> RequestTracker<Desired, Actual> {
  pub fn new(actual: Actual, desired: Desired) -> Self {
    Self { desired:           Timestamped::now(desired),
           actual:            Timestamped::now(actual),
           retry_interval_ms: 1000,
           last_request_at:   None,
           next_request_at:   Utc::now(), }
  }

  pub fn update_requested_now(&mut self) {
    let now = Utc::now();
    self.last_request_at = Some(now);
    self.next_request_at = now + Duration::milliseconds(self.retry_interval_ms as i64);
  }

  pub fn set_desired(&mut self, desired: Desired) {
    if self.desired.value != desired {
      self.desired = Timestamped::now(desired);
      self.last_request_at = None;
      self.next_request_at = Utc::now();
    }
  }

  pub fn set_actual(&mut self, actual: Actual) {
    self.actual = Timestamped::now(actual);
  }

  pub fn set_retry_interval(&mut self, interval_ms: u32) {
    self.retry_interval_ms = interval_ms;
  }

  pub fn actual_elapsed_ms(&self) -> u64 {
    (Utc::now() - self.actual.time).num_milliseconds().max(0) as u64
  }
}

impl<Desired, Actual> Default for RequestTracker<Desired, Actual>
  where Desired: Default,
        Actual: Default
{
  fn default() -> Self {
    Self::new(Default::default(), Default::default())
  }
}

impl<Desired, Actual> RequestTracker<Desired, Actual> where Actual: PartialEq<Desired>
{
  pub fn is_fulfilled(&self) -> bool {
    &self.actual.value == &self.desired.value
  }

  pub fn should_request_update(&self) -> bool {
    if !self.is_fulfilled() {
      Utc::now() >= self.next_request_at
    } else {
      false
    }
  }
}
