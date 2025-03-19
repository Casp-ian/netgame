use avian3d::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
#[require(Transform)]
pub struct Player;

#[derive(Component)]
#[require(Transform)]
pub struct Head;

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut qp: Query<(&mut LinearVelocity, &mut AngularVelocity), (With<Player>, Without<Head>)>,
    qc: Query<&Transform, (With<Head>, Without<Player>)>,
) {
    let mut thing: Vec3 = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        thing -= Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyA) {
        thing -= Vec3::X;
    }
    if keys.pressed(KeyCode::KeyS) {
        thing += Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyD) {
        thing += Vec3::X;
    }

    // thing *= time.delta_secs();
    //
    let (mut linear, mut angular) = qp.single_mut();

    thing = qc.single().rotation.mul_vec3(thing);

    linear.x = thing.x;
    linear.y = thing.y;
    linear.z = thing.z;

    angular.x = 0.0;
    angular.y = 0.0;
    angular.z = 0.0;
}

pub fn move_camera(mut motion: EventReader<MouseMotion>, mut q: Query<&mut Transform, With<Head>>) {
    let delta: Vec2 = motion
        .read()
        .map(|event| event.delta)
        .reduce(|acc, e| acc + e)
        .unwrap_or(Vec2::ZERO);

    let sens = 0.01;

    for mut t in &mut q {
        t.rotate_axis(Dir3::Y, delta.x * -sens);
        // as this is approximate, it might fuck up
        let left = t.left().fast_renormalize();
        t.rotate_axis(left, delta.y * sens);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        Player,
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
        Transform::from_xyz(-2.5, 4.5, 9.0),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Visibility::Visible,
    );

    let head = (
        Head,
        Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    );

    let camera = (Camera3d::default(), Transform::from_xyz(0.0, 0.0, 2.0));

    commands.spawn(player).with_children(|parent| {
        parent.spawn(head).with_children(|parent| {
            parent.spawn(camera);
        });
    });
}
