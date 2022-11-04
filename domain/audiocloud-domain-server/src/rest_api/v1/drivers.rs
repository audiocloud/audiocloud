/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Addr;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::web::ServiceConfig;
use actix_web::{post, web, Error, HttpRequest, Responder};
use anyhow::anyhow;
use reqwest::Url;

use audiocloud_api::cloud::domains::InstanceDriverConfig;
use audiocloud_api::{InstanceDriverId, ServicePorts};

use crate::fixed_instances::{DriversSupervisor, RegisterInstanceDriver};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(register);
}

#[post("/{driver_id}/register")]
async fn register(req: HttpRequest,
                  driver_id: web::Path<InstanceDriverId>,
                  config: web::Json<InstanceDriverConfig>,
                  driver_supervisor: web::Data<Addr<DriversSupervisor>>)
                  -> Result<web::Json<InstanceDriverConfig>, Error> {
    const PORT: u16 = ServicePorts::InstanceDriverHttps as u16;

    let driver_id = driver_id.into_inner();
    let config = config.into_inner();
    if let Some(peer) = req.peer_addr().map(|addr| addr.ip()) {
        let peer_url = Url::parse(&format!("https://{peer}:{PORT}")).map_err(ErrorInternalServerError)?;
        let config = driver_supervisor.send(RegisterInstanceDriver { driver_id: { driver_id },
                                                                     provided:  { config },
                                                                     base_url:  { peer_url }, })
                                      .await
                                      .map_err(ErrorInternalServerError)?
                                      .map_err(ErrorBadRequest)?;
        Ok(web::Json(config))
    } else {
        Err(ErrorBadRequest(anyhow!("Request is missing peer IP address, can't construct URL")))
    }
}
