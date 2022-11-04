/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix_web::web;

mod streaming;
mod tasks;
mod drivers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/streams").configure(streaming::configure))
       .service(web::scope("/tasks").configure(tasks::configure))
       .service(web::scope("/drivers").configure(drivers::configure));
}
