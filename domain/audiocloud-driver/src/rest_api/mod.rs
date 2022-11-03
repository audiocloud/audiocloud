/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix_web::web;

mod v1;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("v1").configure(v1::configure));
}
