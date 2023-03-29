
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
  desired:         Timestamped<Desired>,
  actual:          Timestamped<Actual>,
  last_request_at: Option<Timestamp>,
  next_request_at: Timestamp,
  #[serde(with = "dur_milliseconds")]
  retry_interval:  Duration,
}

mod dur_milliseconds {
  use std::fmt::Formatter;

  use chrono::Duration;
  use serde::de::Error;
  use serde::{de, Deserializer, Serializer};

  pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
  {
    serializer.serialize_i64(value.num_milliseconds() as i64)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de>
  {
    struct DurationVisitor;

    impl<'de> de::Visitor<'de> for DurationVisitor {
      type Value = Duration;

      fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an i64 representing milliseconds")
      }

      fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where E: de::Error
      {
        Ok(Duration::milliseconds(v))
      }

      fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where E: Error
      {
        Ok(Duration::milliseconds(v as i64))
      }

      fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where E: Error
      {
        Ok(Duration::milliseconds(v as i64))
      }

      fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where E: Error
      {
        Ok(Duration::milliseconds(v as i64))
      }
    }

    deserializer.deserialize_i64(DurationVisitor)
  }
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
    Self { desired:         Timestamped::now(desired),
           actual:          Timestamped::now(actual),
           retry_interval:  Duration::seconds(1),
           last_request_at: None,
           next_request_at: Utc::now(), }
  }

  pub fn update_requested_now(&mut self) {
    let now = Utc::now();
    self.last_request_at = Some(now);
    self.next_request_at = now + self.retry_interval;
  }

  pub fn set_desired(&mut self, desired: Desired) {
    self.desired = Timestamped::now(desired);
    self.last_request_at = None;
    self.next_request_at = Utc::now();
  }

  pub fn set_actual(&mut self, actual: Actual) {
    self.actual = Timestamped::now(actual);
  }

  pub fn set_retry_interval(&mut self, interval: Duration) {
    self.retry_interval = interval;
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
