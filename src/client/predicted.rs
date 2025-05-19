use avian3d::prelude::RigidBody;
use bevy::prelude::*;
use lightyear::prelude::client::*;

use crate::protocol::component::EnemyId;
use crate::protocol::component::PlayerId;
use crate::protocol::component::ProjectileId;
use crate::shared::enemy::EnemyBundle;
use crate::shared::player::PlayerBundle;
use crate::shared::projectile::ProjectileBundle;

pub struct PredictedPlugin;

impl Plugin for PredictedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (add_character_mesh, add_projectile_mesh, add_enemy_mesh),
        );
        app.add_systems(Update, turn_billboard);
    }
}

fn add_character_mesh(
    mut commands: Commands,
    character_query: Query<
        (Entity, Has<Mesh3d>, Has<RigidBody>),
        (Added<Predicted>, With<PlayerId>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mesh, physics) in &character_query {
        info!(?entity, "Adding cosmetics to character {:?}", entity);

        let mut body = commands.entity(entity);

        if !mesh {
            body.insert((
                Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
                MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            ));
        }

        if !physics {
            body.insert(PlayerBundle::default());
        }
    }
}

fn add_projectile_mesh(
    mut commands: Commands,
    character_query: Query<
        (Entity, Has<Mesh3d>, Has<RigidBody>),
        (Added<Predicted>, With<ProjectileId>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mesh, physics) in &character_query {
        let mut body = commands.entity(entity);

        if !mesh {
            info!(?entity, "Adding cosmetics to projectile {:?}", entity);
            body.insert((
                Mesh3d(meshes.add(Sphere::new(0.75))),
                MeshMaterial3d(materials.add(Color::srgb(0.5, 0.4, 0.4))),
            ));
        }

        if !physics {
            info!(?entity, "Adding physics to projectile {:?}", entity);
            body.insert(ProjectileBundle::default());
        }
    }
}

fn add_enemy_mesh(
    mut commands: Commands,
    character_query: Query<
        (Entity, Has<Mesh3d>, Has<RigidBody>),
        (Added<Predicted>, With<EnemyId>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    for (entity, mesh, physics) in &character_query {
        let mut body = commands.entity(entity);

        let image_handle: Handle<Image> = server.load("images/enemy.png");
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(image_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            // double_sided: true, // This doesnt seem to work anyways
            ..default()
        });

        if !mesh {
            info!(?entity, "Adding cosmetics to enemy {:?}", entity);
            body.insert((
                Billboard,
                Mesh3d(meshes.add(Rectangle::new(2.0, 2.0))),
                MeshMaterial3d(material_handle),
            ));
        }

        if !physics {
            info!(?entity, "Adding physics to enemy {:?}", entity);
            body.insert(EnemyBundle { ..default() });
        }
    }
}

#[derive(Component)]
pub struct Billboard;

fn turn_billboard(
    mut boards: Query<&mut Transform, With<Billboard>>,
    player: Query<&GlobalTransform, (With<Camera3d>, Without<Billboard>)>,
) {
    if let Ok(camera_pos) = player.single() {
        for mut transform in boards.iter_mut() {
            let dir: Vec3 = transform.translation - camera_pos.translation();
            transform.look_to(dir, Vec3::Y);
        }
    } else {
        info!("none");
    }
}
