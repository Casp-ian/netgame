use bevy::{ecs::system::SystemId, prelude::*};
use lightyear::prelude::{NetworkTarget, ServerReplicate, server};
use std::collections::HashMap;

use crate::{
    protocol::{
        REPLICATION_GROUP,
        component::{EnemyId, ProjectileId},
    },
    shared::{enemy::EnemyBundle, projectile::ProjectileBundle},
};

#[derive(Resource)]
pub struct ServerOneshotSystems {
    pub list: HashMap<String, SystemId>,
}

impl FromWorld for ServerOneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = ServerOneshotSystems {
            list: HashMap::new(),
        };

        systems
            .list
            .insert("stop".into(), world.register_system(super::network::stop));

        systems
            .list
            .insert("ball".into(), world.register_system(spawn_ball));

        systems
            .list
            .insert("kanye".into(), world.register_system(spawn_enemy));

        systems
    }
}

fn spawn_ball(
    // a
    mut commands: Commands,
) {
    let replicate = ServerReplicate {
        group: REPLICATION_GROUP,
        sync: server::SyncTarget {
            prediction: NetworkTarget::All,
            interpolation: NetworkTarget::None,
        },
        ..default()
    };

    // the default hashing algorithm uses the tick and component list. in order to disambiguate
    // between two players spawning a bullet on the same tick, we add client_id to the mix.
    commands.spawn((
        replicate,
        ProjectileId { id: 0 },
        Transform::from_xyz(0.0, 5.0, 0.0),
        ProjectileBundle::default(),
        // NOTE could make gui feature
        // Mesh3d(meshes.add(Sphere::new(0.25))),
        // MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
    ));
}

fn spawn_enemy(
    // a
    mut commands: Commands,
) {
    let replicate = ServerReplicate {
        group: REPLICATION_GROUP,
        sync: server::SyncTarget {
            prediction: NetworkTarget::All,
            interpolation: NetworkTarget::None,
        },
        ..default()
    };

    // the default hashing algorithm uses the tick and component list. in order to disambiguate
    // between two players spawning a bullet on the same tick, we add client_id to the mix.
    commands.spawn((
        // TODO enemy
        replicate,
        EnemyId { id: 0 },
        Transform::from_xyz(0.0, 5.0, 0.0),
        EnemyBundle { ..default() },
        // NOTE could make gui feature
        // Mesh3d(meshes.add(Sphere::new(0.25))),
        // MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
    ));
}
