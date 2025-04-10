use bevy::prelude::*;
use lightyear::prelude::client::*;

use crate::protocol::component::PlayerId;
use crate::protocol::component::ProjectileId;
use crate::shared::player::PlayerBundle;
use crate::shared::projectile::ProjectileBundle;

pub struct PredictedPlugin;

impl Plugin for PredictedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, (add_character_mesh, add_projectile_mesh));
    }
}

fn add_character_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    character_query: Query<Entity, (Or<(Added<Predicted>, Added<Interpolated>)>, With<PlayerId>)>,
) {
    for entity in &character_query {
        info!(?entity, "Adding cosmetics to character {:?}", entity);

        let mut body = commands.entity(entity);

        body.insert((
            Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            PlayerBundle {
                ..Default::default()
            },
        ));
    }
}

fn add_projectile_mesh(
    mut commands: Commands,
    character_query: Query<Entity, (Or<(Added<Predicted>,)>, With<ProjectileId>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &character_query {
        info!(?entity, "Adding cosmetics to projectile {:?}", entity);

        let mut body = commands.entity(entity);

        body.insert((
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            ProjectileBundle { ..default() },
        ));
    }
}
