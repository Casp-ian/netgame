use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy::prelude::*;
use lightyear::{
    client::{
        config::{ClientConfig, NetcodeConfig},
        plugin::ClientPlugins,
    },
    prelude::{
        ClientDisconnectEvent, Key,
        client::{Authentication, ClientCommandsExt, ClientTransport, IoConfig, NetConfig},
    },
};

use crate::shared::{CLIENT_ADDR, SERVER_ADDR, shared_config};

use super::ClientGameState;

pub struct ClientNetworkPlugin;
impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(build_client_plugin());

        app.add_systems(Update, disconnect);
    }
}

// oneshot
pub fn connect(mut commands: Commands, mut game_state: ResMut<NextState<ClientGameState>>) {
    commands.connect_client();
    game_state.set(ClientGameState::Game);
}

fn disconnect(
    mut connections: EventReader<ClientDisconnectEvent>,
    mut game_state: ResMut<NextState<ClientGameState>>,
) {
    for event in connections.read() {
        info!("{:?}", event);
        game_state.set(ClientGameState::MainMenu);
    }
}

fn build_client_plugin() -> ClientPlugins {
    let id = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
        % 1000)
        + 4000;

    let mut client_addr = CLIENT_ADDR.clone();
    client_addr.set_port(id as u16);
    // Authentication is where you specify how the client should connect to the server
    // This is where you provide the server address.
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: id,
        private_key: Key::default(),
        protocol_id: 0,
    };
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(client_addr),
        ..Default::default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };
    let config = ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        net: net_config,
        ..Default::default()
    };
    ClientPlugins::new(config)
}
