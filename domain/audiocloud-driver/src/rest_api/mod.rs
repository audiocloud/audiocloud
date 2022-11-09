/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use axum::Router;

use crate::client::DriverClient;

mod v1;

pub fn configure(router: Router, state: DriverState) -> Router {
    router.nest("/v1", v1::configure(state))
}

#[derive(Clone)]
pub struct DriverState {
    pub instances: DriverClient,
}
