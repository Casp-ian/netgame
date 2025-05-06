use avian3d::prelude::*;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

/// set up a simple 3D scene
pub fn setup_map(
    mut commands: Commands,
    #[cfg(feature = "client")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "client")] mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor
    commands.spawn((
        #[cfg(feature = "client")]
        Mesh3d(meshes.add(Cuboid::new(100.0, 1.0, 100.0))),
        #[cfg(feature = "client")]
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -0.5, 0.0),
        RigidBody::Static,
        Collider::cuboid(100.0, 1.0, 100.0),
    ));
    // cube
    commands.spawn((
        #[cfg(feature = "client")]
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        #[cfg(feature = "client")]
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_xyz(1.0, 0.5, 2.0),
    ));
    // light
    #[cfg(feature = "client")]
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // TODO disable rendering on server
    #[cfg(feature = "client")]
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Visibility::Visible,
    ));
}
