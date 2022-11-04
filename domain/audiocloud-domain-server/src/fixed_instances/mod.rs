/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Actor, Addr};
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use tracing::*;

use audiocloud_api::cloud::domains::{DomainConfig, FixedInstanceRoutingMap};
pub use drivers::DriversSupervisor;
pub use messages::*;
pub use supervisor::FixedInstancesSupervisor;

use crate::db::Db;

mod drivers;
mod instance;
mod messages;
mod supervisor;
mod values;

static INSTANCE_SUPERVISOR: OnceCell<Addr<FixedInstancesSupervisor>> = OnceCell::new();

pub fn get_instance_supervisor() -> &'static Addr<FixedInstancesSupervisor> {
    INSTANCE_SUPERVISOR.get().expect("Instance supervisor not initialized")
}

#[instrument(skip_all, err)]
pub async fn init(cfg: &DomainConfig, db: Db) -> anyhow::Result<FixedInstanceRoutingMap> {
    let supervisor = FixedInstancesSupervisor::new(cfg, db).await?;
    INSTANCE_SUPERVISOR.set(supervisor.start())
                       .map_err(|_| anyhow!("INSTANCE_SUPERVISOR already initialized"))?;

    Ok(Default::default())
}

#[derive(Clone)]
pub struct Instances {
    supervisor: Addr<FixedInstancesSupervisor>,
}
