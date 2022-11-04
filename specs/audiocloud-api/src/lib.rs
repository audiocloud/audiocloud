/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

//! A crate of audiocloud API definitions and API calls
#![allow(dead_code)]

pub use api::*;
pub use common::*;

pub mod api;
pub mod audio_engine;
pub mod cloud;
pub mod common;
pub mod domain;
pub mod instance_driver;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServicePorts {
    CloudServerHttps = 7000,
    DomainServerHttps = 7100,
    InstanceDriverHttps = 7200,
    AudioEngineHttps = 7300,
}
