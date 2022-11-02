use std::collections::HashMap;

use actix::fut::LocalBoxActorFuture;
use actix::{fut, Actor, ActorFutureExt, Addr, Context, Handler, MessageResult, WrapFuture};
use actix_broker::BrokerSubscribe;
use anyhow::anyhow;
use futures::executor::block_on;
use tracing::*;

use audiocloud_api::cloud::domains::{
    DomainConfig, DomainFixedInstanceEngine, FixedInstanceConfig, FixedInstanceRouting, FixedInstanceRoutingMap,
};
use audiocloud_api::domain::DomainError;
use audiocloud_api::{hashmap_changes, FixedInstanceId, HashMapChanges};

use crate::config::NotifyDomainConfiguration;
use crate::db::Db;
use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::{
    GetMultipleFixedInstanceState, MergeInstanceParameters, NotifyFixedInstanceReports, NotifyInstanceState, SetInstanceDesiredPlayState,
    SetInstanceParameters,
};
use crate::DomainResult;

mod forward_instance_reports;
mod forward_merge_parameters;
mod forward_set_parameters;
mod forward_set_play_state;
mod on_domain_config_change;

pub struct FixedInstancesSupervisor {
    config:    DomainConfig,
    instances: HashMap<FixedInstanceId, SupervisedInstance>,
    db:        Db,
}

struct SupervisedInstance {
    address: Addr<FixedInstanceActor>,
    config:  FixedInstanceConfig,
    state:   Option<NotifyInstanceState>,
}

impl FixedInstancesSupervisor {
    pub async fn new(boot: &DomainConfig, db: Db) -> anyhow::Result<Self> {
        Ok(Self { db:        { db },
                  instances: { HashMap::new() },
                  config:    { boot.clone() }, })
    }
}

impl Actor for FixedInstancesSupervisor {
    type Context = Context<Self>;

    #[instrument(skip_all)]
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<NotifyDomainConfiguration>(ctx);
        self.subscribe_system_async::<NotifyFixedInstanceReports>(ctx);
    }
}

impl Handler<GetMultipleFixedInstanceState> for FixedInstancesSupervisor {
    type Result = MessageResult<GetMultipleFixedInstanceState>;

    fn handle(&mut self, msg: GetMultipleFixedInstanceState, _ctx: &mut Self::Context) -> Self::Result {
        let mut rv = HashMap::new();

        for id in msg.instance_ids {
            if let Some(instance) = self.instances.get(&id) {
                if let Some(state) = instance.state.clone() {
                    rv.insert(id.clone(), state);
                }
            }
        }

        MessageResult(rv)
    }
}
