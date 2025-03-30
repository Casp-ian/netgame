use avian3d::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use serde::{Deserialize, Serialize};

use crate::protocol::{Controller, PlayerId};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    Grounded,
    Aerial,
}

#[derive(Component, PartialEq, Serialize, Deserialize)]
#[require(Transform)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Component, PartialEq, Serialize, Deserialize)]
#[require(Transform)]
pub struct Head;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, controller)
            .add_systems(Update, (move_player, move_camera));
    }
}

pub fn controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut qp: Query<&mut Controller, Without<Head>>,
    qc: Query<&Transform, With<Head>>,
) {
    if qp.is_empty() {
        info!("returned early, qp");
        return;
    }

    if qc.is_empty() {
        info!("returned early, cq");
        return;
    }

    let rotation = qc.iter().next().unwrap().rotation.to_euler(EulerRot::XYZ).1;
    let mut controller = qp.iter_mut().next().unwrap();

    let mut whish: Vec2 = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        whish -= Vec2::Y;
    }
    if keys.pressed(KeyCode::KeyA) {
        whish -= Vec2::X;
    }
    if keys.pressed(KeyCode::KeyS) {
        whish += Vec2::Y;
    }
    if keys.pressed(KeyCode::KeyD) {
        whish += Vec2::X;
    }

    if keys.pressed(KeyCode::Space) {
        info!("Jump!!");
        controller.jump = true;
    } else {
        controller.jump = false;
    }

    let sin = rotation.sin();
    let cos = rotation.cos();

    // whish = rotation.mul_vec3(whish);
    // whish.y = 0.0;

    // whish = whish.normalize_or_zero();

    let rotated: Vec2 = Vec2::new(
        (whish.x * sin) + (whish.y * cos),
        (whish.y * sin) + (whish.x * cos),
    );

    controller.whish_dir = rotated;
}

pub fn move_player(
    // time: Res<Time>,
    mut qp: Query<(&Controller, &mut LinearVelocity, &ShapeHits, &mut Player), Without<Head>>,
) {
    if qp.is_empty() {
        return;
    }

    let (controller, mut linear, hits, mut player) = qp.single_mut();

    let grounded = 0.5;
    let grounded_pad = 0.6;
    let adjustment = 2.5;
    let dampening = 0.8;

    let ground_friction = 0.9;

    let mut height = 100.0;
    if let Some(x) = hits.iter().next() {
        height = x.distance;
    }

    let speed = 1.0;
    let gravity = -9.8;
    let jump_height = 10.0;
    let max_fall = gravity * 2.0;

    linear.x += controller.whish_dir.x * speed;
    linear.z += controller.whish_dir.y * speed;

    // grounded state
    if height > grounded - grounded_pad && height < grounded + grounded_pad {
        player.state = PlayerState::Grounded;
        let diff = (height - grounded) / -grounded_pad;

        linear.y += diff * adjustment;
        linear.y *= dampening; // TODO this should be delta timed

        linear.x *= ground_friction;
        linear.z *= ground_friction;
    } else {
        player.state = PlayerState::Aerial;
    }

    if controller.jump && player.state == PlayerState::Grounded {
        linear.y = jump_height;
    }

    // linear.y += gravity * time.delta_secs();

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_id: PlayerId,
    pub controller: Controller,
    pub mesh3d: Mesh3d,
    pub mesh_material3d: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub visibility: Visibility,
    pub shape_caster: ShapeCaster,
}

// TODO i would make a scene out of this instead of a bundle
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player {
                state: PlayerState::Grounded,
            },
            player_id: PlayerId {
                id: lightyear::prelude::ClientId::Local(0),
            },
            controller: Controller {
                whish_dir: Vec2::ZERO,
                jump: false,
            },

            mesh3d: Mesh3d::default(),
            mesh_material3d: MeshMaterial3d::default(),

            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(0.25, 0.1),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            visibility: Visibility::Visible,
            shape_caster: ShapeCaster::new(
                Collider::sphere(0.2), // Shape
                Vec3::ZERO,            // Origin
                Quat::default(),       // Shape rotation
                Dir3::X,               // Direction
            )
            .with_max_hits(1)
            .with_ignore_origin_penetration(true)
            .with_max_distance(100.0)
            .with_direction(Dir3::NEG_Y),
        }
    }
}

// pub fn spawn_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let player = (
//         Player {
//             state: PlayerState::Grounded,
//         },
//         Mesh3d(meshes.add(Capsule3d::new(0.25, 0.1))),
//         MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
//         Transform::from_xyz(-2.5, 4.5, 9.0),
//         RigidBody::Dynamic,
//         // GravityScale(0.0),
//         // Friction::new(0.0)
//         //     .with_dynamic_coefficient(0.0)
//         //     .with_static_coefficient(0.0),
//         Collider::capsule(0.25, 0.1),
//         LockedAxes::new()
//             .lock_rotation_x()
//             .lock_rotation_y()
//             .lock_rotation_z(),
//         Visibility::Visible,
//         ShapeCaster::new(
//             Collider::sphere(0.2), // Shape
//             Vec3::ZERO,            // Origin
//             Quat::default(),       // Shape rotation
//             Dir3::X,               // Direction
//         )
//         .with_max_hits(1)
//         .with_ignore_origin_penetration(true)
//         .with_max_distance(100.0)
//         .with_direction(Dir3::NEG_Y),
//     );

//     let head = (
//         Head,
//         Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
//         Visibility::Visible,
//     );

//     let camera = (
//         Camera3d::default(),
//         Transform::from_xyz(0.0, 1.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
//         Visibility::Visible,
//     );

//     commands.spawn(player).with_children(|parent| {
//         parent.spawn(head).with_children(|parent| {
//             parent.spawn(camera);
//         });
//     });
// }
