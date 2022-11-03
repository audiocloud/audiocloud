/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use audiocloud_api::Timestamped;

/// Value that will be pushed to a remote service.
pub struct RemoteValue<T: Clone> {
    local:              u64,
    remote:             u64,
    update_in_progress: bool,
    value:              Timestamped<T>,
}

impl<T: Clone> RemoteValue<T> {
    pub fn new(value: T) -> Self {
        Self { local:              { 1 },
               remote:             { 0 },
               value:              { Timestamped::new(value) },
               update_in_progress: { false }, }
    }

    pub fn set(&mut self, value: T) {
        self.value = Timestamped::new(value);
        self.mark_modified();
    }

    pub fn mark_modified(&mut self) {
        self.local += 1;
        self.remote = 0;
    }

    pub fn get_ref(&self) -> &T {
        self.value.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    pub fn timestamped(&self) -> &Timestamped<T> {
        &self.value
    }

    pub fn flush(&mut self) {
        self.local += 1;
        self.remote = 0;
    }

    pub fn start_update(&mut self) -> Option<(u64, T)> {
        if self.update_in_progress || self.local == self.remote {
            None
        } else {
            self.update_in_progress = true;
            Some((self.local, self.value.get_ref().clone()))
        }
    }

    pub fn finish_update(&mut self, remote: u64, successful: bool) {
        if successful {
            self.remote = remote;
        }
        self.update_in_progress = false;
    }
}
