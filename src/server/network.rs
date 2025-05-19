use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use config::ServerConfig;
use lightyear::{
    prelude::{
        ReplicationConfig,
        server::{IoConfig, NetConfig, ServerCommandsExt, ServerTransport},
    },
    server::{config::NetcodeConfig, plugin::ServerPlugins, *},
};

use crate::shared::shared_config;

pub struct ServerNetworkPlugin;
impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(build_server_plugin())
            .add_systems(Startup, start);
    }
}

fn start(mut commands: Commands) {
    commands.start_server();
}

// for oneshot
pub fn stop(mut exit: EventWriter<AppExit>, mut commands: Commands) {
    commands.stop_server();
    exit.write(AppExit::Success);
}

fn build_server_plugin() -> ServerPlugins {
    let mut port: u16 = 25565;
    if let Ok(port_string) = std::env::var("NETGAME_PORT") {
        port = port_string.parse::<u16>().unwrap_or(25565);
    }

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(server_addr),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
        // replication: None,
    };
    let config = ServerConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        // we can specify multiple net configs here, and the server will listen on all of them
        // at the same time. Here we will only use one
        net: vec![net_config],
        // TODO specify config if needed
        replication: ReplicationConfig { ..default() },
        ..default()
    };
    ServerPlugins::new(config)
}
