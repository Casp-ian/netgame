use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
#[require(Transform)]
pub struct Player;

#[derive(Component)]
#[require(Transform, Camera3d)]
pub struct Head;

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut qp: Query<&mut Transform, (With<Player>, Without<Head>)>,
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

    thing *= time.delta_secs();

    thing = qc.single().rotation.mul_vec3(thing);

    qp.single_mut().translation += thing;
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
