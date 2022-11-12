/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use axum::Router;

use crate::client::DriverClient;

mod v1;

pub fn configure(router: Router<DriverState>) -> Router<DriverState> {
    router.nest("/v1", v1::configure())
}

#[derive(Clone)]
pub struct DriverState {
    pub instances: DriverClient,
}
