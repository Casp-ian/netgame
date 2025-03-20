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
    mut qp: Query<(&mut LinearVelocity, &ShapeHits), (With<Player>, Without<Head>)>,
    qc: Query<&Transform, (With<Head>, Without<Player>)>,
) {
    let mut whish: Vec3 = Vec3::ZERO;
    let mut jump = false;

    if keys.pressed(KeyCode::KeyW) {
        whish -= Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyA) {
        whish -= Vec3::X;
    }
    if keys.pressed(KeyCode::KeyS) {
        whish += Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyD) {
        whish += Vec3::X;
    }

    if keys.pressed(KeyCode::Space) {
        jump = true;
    }

    let (mut linear, hits) = qp.single_mut();
    let rotation = qc.single().rotation;

    let float = 20.0;

    for hit in hits.iter() {
        println!("{:?}", hit);
        linear.y += float * time.delta_secs();
    }

    whish = rotation.mul_vec3(whish);
    whish.y = 0.0;

    whish = whish.normalize_or_zero();

    let speed = 1.0;
    let gravity = -9.8;
    let jump_height = 10.0;
    let max_fall = gravity * 2.0;

    linear.x += whish.x * speed;
    linear.z += whish.z * speed;

    if jump {
        linear.y = jump_height;
    }

    linear.y += gravity * time.delta_secs();

    if linear.y < max_fall {
        linear.y = max_fall;
    }
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
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
        Transform::from_xyz(-2.5, 4.5, 9.0),
        RigidBody::Dynamic,
        Collider::sphere(0.25),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        Visibility::Visible,
        ShapeCaster::new(
            Collider::sphere(0.25),    // Shape
            Vec3::new(0.0, -0.5, 0.0), // Origin
            Quat::default(),           // Shape rotation
            Dir3::X,                   // Direction
        ),
    );

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

    commands.spawn(player).with_children(|parent| {
        parent.spawn(head).with_children(|parent| {
            parent.spawn(camera);
        });
    });
}
