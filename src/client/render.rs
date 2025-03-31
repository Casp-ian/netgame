use avian3d::prelude::*;
use bevy::{color::palettes::css::PURPLE, prelude::*};
use leafwing_input_manager::prelude::InputMap;
use leafwing_input_manager::prelude::MouseMove;
use leafwing_input_manager::prelude::VirtualDPad;
use lightyear::prelude::client::*;

use crate::protocol::component::PlayerId;
use crate::protocol::component::ProjectileId;
use crate::protocol::input::NetworkedInput;
use crate::shared::player::Head;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (add_character_mesh, add_projectile_mesh));
    }
}

fn add_character_mesh(
    mut commands: Commands,
    character_query: Query<
        Entity,
        (
            Or<(
                // Added<Predicted>,
                // Added<ReplicationTarget>,
                Added<Interpolated>,
            )>,
            With<PlayerId>,
        ),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &character_query {
        info!(?entity, "Adding cosmetics to character {:?}", entity);

        let head = commands.spawn((Head, Transform::default())).id();

        let mut body = commands.entity(entity);

        body.add_child(head);

        body.insert((
            Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            InputMap::<NetworkedInput>::default()
                .with_dual_axis(NetworkedInput::Move, VirtualDPad::wasd()),
            RigidBody::Dynamic, // Dont know why rigid body is needed to show the mesh??
        ));
    }
}

fn add_projectile_mesh(
    mut commands: Commands,
    character_query: Query<
        Entity,
        (
            Or<(
                Added<Predicted>,
                // Added<ReplicationTarget>,
                // Added<Interpolated>,
            )>,
            With<ProjectileId>,
        ),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &character_query {
        info!(?entity, "Adding cosmetics to projectile {:?}", entity);

        let mut body = commands.entity(entity);

        body.insert((
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            RigidBody::Dynamic, // Dont know why rigid body is needed to show the mesh??
        ));
    }
}
