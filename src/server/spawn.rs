use bevy::prelude::*;
use lightyear::prelude::*;

use crate::shared::player::{Head, PlayerBundle};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_connections);
    }
}

fn handle_connections(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Here we listen for the `ConnectEvent` event
    mut connections: EventReader<ServerConnectEvent>,
    // mut global: ResMut<Global>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        // on the server, the `context()` method returns the `ClientId` of the client that connected
        let client_id = connection.client_id;

        eprintln!("OOOOH!!! {}", client_id);

        // We add the `Replicate` bundle to start replicating the entity to clients
        // By default, the entity will be replicated to all clients
        let player = PlayerBundle {
            mesh3d: Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
            mesh_material3d: MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            ..Default::default()
        };

        let head = (
            Head,
            Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::Visible,
        );

        let camera = (
            Camera3d::default(),
            Transform::from_xyz(0.0, 1.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::Visible,
        );

        commands
            .spawn((player, Replicating))
            .with_children(|parent| {
                parent.spawn((head, Replicating)).with_children(|parent| {
                    parent.spawn((camera, Replicating));
                });
            });

        // Add a mapping from client id to entity id
        // global.client_id_to_entity_id.insert(client_id, entity.id());
    }
}
