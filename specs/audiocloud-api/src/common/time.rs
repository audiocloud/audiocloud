/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
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

impl<T> Deref for Timestamped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

impl<T> DerefMut for Timestamped<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> From<T> for Timestamped<T> {
    fn from(t: T) -> Self {
        Self(Utc::now(), t)
    }
}

impl<T> Timestamped<Option<T>> {
    pub fn exists(&self) -> bool {
        self.get_ref().is_some()
    }
}

impl<T> Timestamped<T> {
    pub fn new(t: T) -> Self {
        t.into()
    }

    pub fn new_with_epoch(t: T) -> Self {
        Self(Utc.from_utc_datetime(&NaiveDateTime::default()), t)
    }

    pub fn elapsed(&self) -> Duration {
        Utc::now() - self.timestamp()
    }

    pub fn get_ref(&self) -> &T {
        &self.1
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.1
    }

    pub fn reset_to_now(&mut self) {
        self.0 = Utc::now();
    }

    pub fn into_inner(self) -> T {
        self.1
    }

    pub fn timestamp(&self) -> Timestamp {
        self.0
    }

    pub fn set_timestamp_if_newer(&mut self, ts: Timestamp) -> bool {
        if self.0 < ts {
            self.0 = ts;
            true
        } else {
            false
        }
    }
}

impl<T> Timestamped<T> where T: Copy
{
    pub fn value_copied(&self) -> T {
        self.1
    }
}

impl<T> Timestamped<T> where T: Clone
{
    pub fn value_cloned(&self) -> T {
        self.1.clone()
    }
}

impl<T> Default for Timestamped<T> where T: Default
{
    fn default() -> Self {
        Timestamped::new(T::default())
    }
}
