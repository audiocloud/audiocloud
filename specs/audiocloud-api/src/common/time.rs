use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Timestamp = DateTime<Utc>;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, JsonSchema)]
pub struct TimeRange {
    pub from: Timestamp,
    pub to:   Timestamp,
}

pub fn now() -> Timestamp {
    Utc::now()
}

impl TimeRange {
    pub fn new(from: Timestamp, to: Timestamp) -> Self {
        Self { from, to }
    }

    pub fn new_with_length(from: Timestamp, duration: Duration) -> Self {
        Self::new(from, from + duration)
    }

    pub fn shifted(self, shift: Duration) -> Self {
        Self::new(self.from + shift, self.to + shift)
    }

    pub fn valid(&self) -> bool {
        self.from < self.to
    }

    pub fn len(&self) -> Duration {
        self.to - self.from
    }

    pub fn intersects(&self, other: &TimeRange) -> bool {
        self.to > other.from && self.from < other.to
    }

    pub fn contains(&self, ts: Timestamp) -> bool {
        match ts {
            ts if ts > self.to => false,
            ts if ts < self.from => false,
            _ => true,
        }
    }

    pub fn contains_now(&self) -> bool {
        self.contains(Utc::now())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, PartialEq, JsonSchema)]
pub struct Timestamped<T>(Timestamp, T);

impl<T> From<T> for Timestamped<T> {
    fn from(t: T) -> Self {
        Self(Utc::now(), t)
    }
}

impl<T> Timestamped<Option<T>> {
    pub fn exists(&self) -> bool {
        self.value().is_some()
    }
}

impl<T> Timestamped<T> {
    pub fn new(t: T) -> Self {
        t.into()
    }

    pub fn elapsed(&self) -> Duration {
        Utc::now() - self.0
    }

    pub fn value(&self) -> &T {
        &self.1
    }

    pub fn reset_to_now(&mut self) {
        self.0 = Utc::now();
    }

    pub fn into_inner(self) -> T {
        self.1
    }
}

impl<T> Timestamped<T> where T: Copy
{
    pub fn value_copy(&self) -> T {
        self.1
    }
}

impl<T> Default for Timestamped<T> where T: Default
{
    fn default() -> Self {
        Timestamped::new(T::default())
    }
}
