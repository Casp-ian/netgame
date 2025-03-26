use lightyear::prelude::{Mode, SharedConfig, TickConfig};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

pub mod map;
pub mod player;

pub const REPLICATION_INTERVAL: Duration = Duration::from_millis(100);
pub const TICK_DURATION: Duration = Duration::from_millis(1000 / 64);
pub const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: TICK_DURATION,
        },
        mode: Mode::Separate,
    }
}
