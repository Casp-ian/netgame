use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::prelude::*;
use lightyear::{
    client::{
        config::{ClientConfig, NetcodeConfig},
        plugin::ClientPlugins,
    },
    prelude::{
        ClientConnectEvent, ClientDisconnectEvent, Key,
        client::{Authentication, ClientCommandsExt, ClientTransport, IoConfig, NetConfig},
    },
};

use crate::{
    protocol::message::{RegisterChannel, RegisterMessage},
    shared::shared_config,
};

use super::ClientGameState;

pub struct ClientNetworkPlugin;
impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(build_client_plugin());

        app.add_systems(Update, (disconnect, register));
    }
}

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

// oneshot
pub fn connect(
    mut commands: Commands,
    ip_text: Query<&Text, With<super::menu::IpBox>>,
    port_text: Query<&Text, With<super::menu::PortBox>>,
    mut client_config: ResMut<ClientConfig>,
    mut game_state: ResMut<NextState<ClientGameState>>,
) {
    // TODO text parsing, for port as well
    let ip: Result<Ipv4Addr, _> = ip_text.single().unwrap().0.clone().parse();
    let port: u16 = port_text.single().unwrap().0.parse::<u16>().unwrap();

    if let Err(e) = ip {
        error!("{:?}", e);
        return;
    }

    client_config.net = netconfig(SocketAddr::new(IpAddr::V4(ip.unwrap()), port));

    commands.connect_client();
    game_state.set(ClientGameState::Game);
}

fn register(
    name_text: Query<&Text, With<super::menu::NameBox>>,

    mut connection_manager: ResMut<lightyear::prelude::client::ConnectionManager>,
    mut connection_events: EventReader<ClientConnectEvent>,
) {
    for _ in connection_events.read() {
        let name = name_text.single().unwrap().0.clone();
        connection_manager
            .send_message::<RegisterChannel, RegisterMessage>(&RegisterMessage { name })
            .unwrap();
    }
}

fn disconnect(
    mut disconnections: EventReader<ClientDisconnectEvent>,
    mut game_state: ResMut<NextState<ClientGameState>>,
) {
    for event in disconnections.read() {
        info!("{:?}", event);
        game_state.set(ClientGameState::MainMenu);
    }
}

fn build_client_plugin() -> ClientPlugins {
    // NOTE this cant be ClientConfig::default(), because some things cant change
    // luckily the server ip changes just fine
    ClientPlugins::new(ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        ..Default::default()
    })
}

fn netconfig(server: SocketAddr) -> NetConfig {
    let id = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
        % 1000)
        + 4000;

    // client_addr.set_port(id as u16);

    let auth = Authentication::Manual {
        server_addr: server,
        client_id: id,
        private_key: Key::default(),
        protocol_id: 0,
    };

    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
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
    return net_config;
}
