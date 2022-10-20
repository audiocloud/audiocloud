use actix::{Actor, Addr};
use anyhow::anyhow;
use clap::Args;
use nanoid::nanoid;
use once_cell::sync::OnceCell;
use tracing::*;

use audiocloud_api::{SecureKey, SocketId};
pub use messages::*;
pub use supervisor::SocketsSupervisor;
pub use web_sockets::configure;

mod messages;
mod supervisor;
mod web_rtc;
mod web_sockets;

static SOCKETS_SUPERVISOR: OnceCell<Addr<SocketsSupervisor>> = OnceCell::new();

#[derive(Args, Clone, Debug)]
pub struct SocketsOpts {
    #[clap(flatten)]
    web_rtc: web_rtc::WebRtcOpts,

    /// Number of milliseconds to wait between pinging sockets (RTC or WebSockets)
    #[clap(long, env, default_value = "2500")]
    socket_ping_interval: u64,

    /// If no ping reply is received after this many milliseconds, the socket is considered dead and will be dropped
    #[clap(long, env, default_value = "15000")]
    socket_drop_timeout: u64,

    /// If the socket fails to initialize (fully connect) in this many milliseconds, the socket is considered dead and will be dropped
    #[clap(long, env, default_value = "15000")]
    socket_init_timeout: u64,
}

fn get_next_socket_id() -> SocketId {
    SocketId::new(nanoid!())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SocketMembership {
    secure_key: SecureKey,
    socket_id: SocketId,
}

#[instrument(skip_all, err)]
pub fn init(cfg: SocketsOpts) -> anyhow::Result<()> {
    let web_rtc_cfg = cfg.web_rtc.clone();
    let supervisor = SocketsSupervisor::new(cfg);

    web_rtc::init(&web_rtc_cfg)?;

    SOCKETS_SUPERVISOR
        .set(supervisor.start())
        .map_err(|_| anyhow!("Sockets supervisor already initialized"))?;

    Ok(())
}

pub fn get_sockets_supervisor() -> &'static Addr<SocketsSupervisor> {
    SOCKETS_SUPERVISOR
        .get()
        .expect("Sockets supervisor not initialized")
}
