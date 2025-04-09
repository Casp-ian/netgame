use bevy::prelude::*;
use lightyear::prelude::*;

use crate::{
    protocol::component::PlayerId,
    protocol::message::{ChatChannel, ChatMessage},
    shared::player::PlayerBundle,
};

use super::network::REPLICATION_GROUP;

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
            group: REPLICATION_GROUP,
            // target: ReplicationTarget {
            //     target: NetworkTarget::All,
            // },
            sync: server::SyncTarget {
                // prediction: NetworkTarget::Single(client_id),
                // interpolation: NetworkTarget::AllExceptSingle(client_id),
                prediction: NetworkTarget::All,
                interpolation: NetworkTarget::None,
            },
            // relevance_mode: NetworkRelevanceMode::All,
            // authority: server::AuthorityPeer::Server,
            controlled_by: server::ControlledBy {
                target: NetworkTarget::Single(client_id),
                lifetime: server::Lifetime::SessionBased,
            },
            hierarchy: ReplicateHierarchy {
                enabled: false,
                ..default()
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

        let player = (
            Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            PlayerId { id: client_id },
            PlayerBundle {
                ..Default::default()
            },
        );

        // We add the `Replicate` bundle to start replicating the entity to clients
        // By default, the entity will be replicated to all clients
        commands.spawn((player, replicate));

        // Add a mapping from client id to entity id
        // global.client_id_to_entity_id.insert(client_id, entity.id());
    }
}
