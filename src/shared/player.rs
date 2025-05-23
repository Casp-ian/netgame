use std::f32::consts::PI;

use avian3d::prelude::{
    Collider, LinearVelocity, LockedAxes, Restitution, RigidBody, ShapeCaster, ShapeHits,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::shared::replication::components::Controlled;
use serde::{Deserialize, Serialize};

use crate::protocol::input::NetworkedInput;

use super::casting::Caster;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (look_player, float_player, move_player).chain(),
        )
        .add_systems(FixedPostUpdate, die)
        .add_systems(Update, move_camera);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PlayerState {
    Grounded,
    Aerial,
}

// NOTE required components do not seem to work perfectly
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player {
    pub state: PlayerState,
    pub look_dir: Vec2,
    pub hp: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Grounded,
            look_dir: Vec2::default(),
            hp: 1,
        }
    }
}

fn die(mut qp: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut pos) in qp.iter_mut() {
        if player.hp <= 0 {
            player.hp = 1;
            *pos = Transform::from_xyz(0.0, 5.0, 0.0);
        }

        if Vec3::ZERO.distance(pos.translation) > 50.0 {
            player.hp = 1;
            *pos = Transform::from_xyz(0.0, 5.0, 0.0);
        }
    }
}

// TODO this probably should be in update, to make camera movement smooth to framerate, but it breaks determinism
fn look_player(mut qp: Query<(&ActionState<NetworkedInput>, &mut Player)>) {
    for (action, mut player) in qp.iter_mut() {
        let sens = Vec2 {
            x: -0.003,
            y: 0.003,
        };
        let input = action.axis_pair(&NetworkedInput::Look);

        let newlook = player.look_dir + (input * sens);
        let clamped = Vec2 {
            x: newlook.x,
            y: newlook.y.clamp(-PI * 0.1, PI * 0.4),
        };

        player.look_dir = clamped;
    }
}

fn float_player(
    // comment to keep rust format from doing this
    mut qp: Query<(&mut LinearVelocity, &ShapeHits, &mut Player)>,
) {
    for (mut linear, hits, mut player) in qp.iter_mut() {
        let grounded = 0.5;
        let grounded_pad = 0.6;
        let adjustment = 2.5;
        let dampening = 0.8;

        let ground_friction = 0.9;

        let mut height = 100.0;
        if let Some(x) = hits.iter().next() {
            height = x.distance;
        }

        let gravity = -9.8;
        let max_fall = gravity * 2.0;

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

        if linear.y < max_fall {
            linear.y = max_fall;
        }
    }
}

fn move_player(mut qp: Query<(&ActionState<NetworkedInput>, &mut LinearVelocity, &Player)>) {
    for (action, mut linear, player) in qp.iter_mut() {
        let speed = 1.0;
        let jump_height = 10.0;

        let axis = action.axis_pair(&NetworkedInput::Move);
        let jump = action.pressed(&NetworkedInput::Jump);

        let quat_x = Quat::from_axis_angle(Vec3::Y, player.look_dir.x);
        let mut movement: Vec3 = Vec3 {
            x: -axis.x,
            y: 0.,
            z: axis.y,
        };

        movement = quat_x.mul_vec3(movement);
        movement = movement.normalize_or_zero() * speed;

        linear.0 += movement;

        if jump && player.state == PlayerState::Grounded {
            linear.y = jump_height;
        }
    }
}

pub fn move_camera(
    qp: Query<(&Transform, &mut Player), With<Controlled>>,
    mut qc: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if qp.is_empty() || qc.is_empty() {
        return;
    }
    let distance = 5.;

    let (player_transform, player) = qp.iter().last().unwrap();
    let mut camera_transform = qc.single_mut().unwrap();

    let player_pos = player_transform.translation;

    let quat_x = Quat::from_axis_angle(Vec3::Y, player.look_dir.x);
    let quat_y = Quat::from_axis_angle(quat_x.mul_vec3(Vec3::X), player.look_dir.y);

    let diff = quat_y.mul_vec3(quat_x.mul_vec3(Vec3::NEG_Z * distance));

    camera_transform.translation = player_pos + diff;
    camera_transform.look_at(player_pos, Vec3::Y);
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub caster: Caster,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub visibility: Visibility,
    pub shape_caster: ShapeCaster,
}

// TODO i would make a scene out of this instead of a bundle
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::default(),
            caster: Caster::new(),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::new(0.1),
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
