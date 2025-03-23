use std::{
    hash::{DefaultHasher, Hash, Hasher},
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{
    prelude::*,
    winit::{UpdateMode::Continuous, WinitSettings},
};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RepliconRenetPlugins,
    netcode::{
        ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
        ServerConfig,
    },
    renet::{ConnectionConfig, RenetClient, RenetServer},
};
use serde::{Deserialize, Serialize};

// pub mod shared;
// use shared::map::MapPlugin;
// use shared::player::PlayerPlugin;

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    eprintln!("Cant have both client and server features!!!");
    // compile_error!("cant have both client and server features at the same time");

    App::new()
        // .init_resource::<Cli>() // Parse CLI before creating window.
        // Makes the server/client update continuously even while unfocused.
        .insert_resource(WinitSettings {
            focused_mode: Continuous,
            unfocused_mode: Continuous,
        })
        .add_plugins((
            DefaultPlugins,
            RepliconPlugins.build().set(ServerPlugin {
                tick_policy: TickPolicy::MaxTickRate(120),
                ..Default::default()
            }),
            RepliconRenetPlugins,
            SimpleBoxPlugin,
        ))
        .run();
}

struct SimpleBoxPlugin;

impl Plugin for SimpleBoxPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<BoxPosition>()
            .replicate::<PlayerBox>()
            .add_client_trigger::<MoveBox>(ChannelKind::Ordered)
            .add_observer(spawn_clients)
            .add_observer(despawn_clients)
            .add_observer(apply_movement)
            .add_systems(Startup, (setup, spawn_camera))
            // .add_systems(Startup, (read_cli.map(Result::unwrap), spawn_camera))
            .add_systems(Update, (read_input, draw_boxes));
    }
}

const PORT: u16 = 5000;
const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

fn setup(mut commands: Commands, channels: Res<RepliconChannels>) {
    const PROTOCOL_ID: u64 = 0;

    // NOTE SINGLEPLAYER is done like this

    // {
    //     info!("starting single-player game");
    //     commands.spawn((
    //         PlayerBox {
    //             color: GREEN.into(),
    //         },
    //         BoxOwner(SERVER),
    //     ));
    // }

    #[cfg(feature = "server")]
    {
        info!("starting server at port {PORT}");
        let server_channels_config = channels.get_server_configs();
        let client_channels_config = channels.get_client_configs();

        let server = RenetServer::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT)).unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 10,
            protocol_id: PROTOCOL_ID,
            authentication: ServerAuthentication::Unsecure,
            public_addresses: Default::default(),
        };
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        commands.insert_resource(server);
        commands.insert_resource(transport);

        commands.spawn((
            Text::new("Server"),
            TextFont {
                font_size: 30.0,
                ..Default::default()
            },
            TextColor::WHITE,
        ));
        // commands.spawn((
        //     PlayerBox {
        //         color: GREEN.into(),
        //     },
        //     BoxOwner(SERVER),
        // ));
    }

    #[cfg(feature = "client")]
    {
        info!("connecting to {IP}:{PORT}");
        let server_channels_config = channels.get_server_configs();
        let client_channels_config = channels.get_client_configs();

        let client = RenetClient::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let server_addr = SocketAddr::new(IP, PORT);
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        commands.insert_resource(client);
        commands.insert_resource(transport);

        commands.spawn((
            Text(format!("Client: {client_id}")),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor::WHITE,
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Spawns a new box whenever a client connects.
fn spawn_clients(trigger: Trigger<OnAdd, ConnectedClient>, mut commands: Commands) {
    // Hash index to generate visually distinctive color.
    let mut hasher = DefaultHasher::new();
    trigger.entity().index().hash(&mut hasher);
    let hash = hasher.finish();

    // Use the lower 24 bits.
    // Divide by 255 to convert bytes into 0..1 floats.
    let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = (hash & 0xFF) as f32 / 255.0;

    // Generate pseudo random color from client entity.
    info!("spawning box for `{}`", trigger.entity());
    commands.spawn((
        PlayerBox {
            color: Color::srgb(r, g, b),
        },
        BoxOwner(trigger.entity()),
    ));
}

/// Despawns a box whenever a client disconnects.
fn despawn_clients(
    trigger: Trigger<OnRemove, ConnectedClient>,
    mut commands: Commands,
    boxes: Query<(Entity, &BoxOwner)>,
) {
    let (entity, _) = boxes
        .iter()
        .find(|(_, owner)| owner.0 == trigger.entity())
        .expect("all clients should have entities");
    commands.entity(entity).despawn();
}

/// Reads player inputs and sends [`MoveDirection`] events.
fn read_input(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        commands.client_trigger(MoveBox(direction.normalize_or_zero()));
    }
}

/// Mutates [`BoxPosition`] based on [`MoveBox`] events.
///
/// Fast-paced games usually you don't want to wait until server send a position back because of the latency.
/// But this example just demonstrates simple replication concept.
fn apply_movement(
    trigger: Trigger<FromClient<MoveBox>>,
    time: Res<Time>,
    mut boxes: Query<(&BoxOwner, &mut BoxPosition)>,
) {
    const MOVE_SPEED: f32 = 300.0;
    // info!("received movement from `{}`", trigger.client_entity);

    // Find the sender entity. We don't include the entity as a trigger target to save traffic, since the server knows
    // which entity to apply the input to. We could have a resource that maps connected clients to controlled entities,
    // but we didn't implement it for the sake of simplicity.
    let (_, mut position) = boxes
        .iter_mut()
        .find(|(owner, _)| owner.0 == trigger.client_entity)
        .unwrap_or_else(|| panic!("`{}` should be connected", trigger.client_entity));

    **position += *trigger.event * time.delta_secs() * MOVE_SPEED;
}

fn draw_boxes(mut gizmos: Gizmos, boxes: Query<(&BoxPosition, &PlayerBox)>) {
    for (position, player) in &boxes {
        gizmos.rect(
            Vec3::new(position.x, position.y, 0.0),
            Vec2::ONE * 50.0,
            player.color,
        );
    }
}

/// Player-controlled box.
///
/// We want to replicate all boxes, so we just set [`Replicated`] as a required component.
#[derive(Component, Deref, Deserialize, Serialize, Default)]
#[require(BoxPosition, Replicated)]
struct PlayerBox {
    /// Color to visually distinguish boxes.
    color: Color,
}

/// Position of a player-controlled box.
///
/// This is a separate component from [`PlayerBox`] because, when the position
/// changes, we only want to send this component (and it changes often!).
#[derive(Component, Deserialize, Serialize, Deref, DerefMut, Default)]
struct BoxPosition(Vec2);

/// Identifies which player controls the box.
///
/// Points to client entity. Used to apply movement to the correct box.
///
/// It's not replicated and present only on server or singleplayer.
#[derive(Component, Clone, Copy, Deref)]
struct BoxOwner(Entity);

/// A movement event for the controlled box.
#[derive(Deserialize, Deref, Event, Serialize)]
struct MoveBox(Vec2);
