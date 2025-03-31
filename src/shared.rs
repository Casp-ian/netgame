use bevy::prelude::*;
use lightyear::prelude::{SharedConfig, TickConfig};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

pub mod map;
pub mod player;

pub const REPLICATION_INTERVAL: Duration = Duration::from_millis(33);
pub const TICK_DURATION: Duration = Duration::from_millis(33);
pub const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

pub struct SharedPlugins;

impl Plugin for SharedPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((map::MapPlugin, player::PlayerPlugin));
    }
}

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: REPLICATION_INTERVAL,
        client_replication_send_interval: REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: TICK_DURATION,
        },
    }
}
