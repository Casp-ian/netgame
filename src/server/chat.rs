use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

use crate::protocol::component::ProjectileId;
use crate::protocol::message::{ChatChannel, ChatMessage};
use crate::shared::projectile::ProjectileBundle;

use super::network::REPLICATION_GROUP;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, forward_chat);
    }
}

fn forward_chat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut connection_manager: ResMut<lightyear::prelude::server::ConnectionManager>,
    mut events: EventReader<FromClients<ChatMessage>>,
) {
    for event in events.read() {
        connection_manager
            .send_message_to_target::<ChatChannel, ChatMessage>(
                &ChatMessage {
                    text: format!("[{}]: {}", event.from(), event.message().text).to_string(),
                },
                NetworkTarget::All,
            )
            .unwrap();

        let replicate = ServerReplicate {
            group: REPLICATION_GROUP,
            // target: ReplicationTarget {
            //     target: NetworkTarget::All,
            // },
            // authority: server::AuthorityPeer::Client(client_id),
            sync: server::SyncTarget {
                prediction: NetworkTarget::All,
                interpolation: NetworkTarget::All,
            },
            // relevance_mode: NetworkRelevanceMode::All,
            hierarchy: ReplicateHierarchy {
                enabled: false,
                ..default()
            },
            ..default()
        };

        commands.spawn((
            replicate,
            ProjectileId { id: 0 },
            Transform::from_xyz(0.0, 2.5, 0.0),
            ProjectileBundle { ..default() },
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
        ));
    }
}
