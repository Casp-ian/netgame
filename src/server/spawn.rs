use bevy::prelude::*;
use lightyear::prelude::*;

use crate::{
    protocol::component::PlayerId,
    protocol::message::{ChatChannel, ChatMessage},
    shared::player::{Head, PlayerBundle},
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_connections);
    }
}

fn handle_connections(
    mut connection_manager: ResMut<lightyear::prelude::server::ConnectionManager>,
    mut connections: EventReader<ServerConnectEvent>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut commands: Commands,
) {
    for connection in connections.read() {
        // on the server, the `context()` method returns the `ClientId` of the client that connected
        let client_id = connection.client_id;

        let replicate = ServerReplicate {
            // target: ReplicationTarget {
            //     target: NetworkTarget::All,
            // },
            // authority: server::AuthorityPeer::Client(client_id),
            sync: server::SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            // relevance_mode: NetworkRelevanceMode::All,
            controlled_by: server::ControlledBy {
                target: NetworkTarget::Single(client_id),
                lifetime: server::Lifetime::SessionBased,
            },
            // group: todo!(),
            hierarchy: ReplicateHierarchy {
                enabled: true,
                recursive: true,
            },
            ..default()
        };

        connection_manager
            .send_message_to_target::<ChatChannel, ChatMessage>(
                &ChatMessage {
                    text: "[Server]: New player joined".to_string(),
                },
                NetworkTarget::All,
            )
            .unwrap();

        let player = PlayerBundle {
            player_id: PlayerId { id: client_id },
            mesh3d: Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
            mesh_material3d: MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            ..Default::default()
        };

        let head = (
            Head,
            Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::Visible,
        );

        // We add the `Replicate` bundle to start replicating the entity to clients
        // By default, the entity will be replicated to all clients
        commands.spawn((player, replicate)).with_children(|parent| {
            parent.spawn(head);
        });

        // Add a mapping from client id to entity id
        // global.client_id_to_entity_id.insert(client_id, entity.id());
    }
}
