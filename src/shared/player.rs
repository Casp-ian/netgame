use std::f32::consts::PI;

use avian3d::prelude::{Collider, LinearVelocity, LockedAxes, RigidBody, ShapeCaster, ShapeHits};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap, MouseMove, VirtualDPad};
use lightyear::shared::replication::components::Controlled;
use serde::{Deserialize, Serialize};

use crate::protocol::{component::PlayerId, input::NetworkedInput};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    Grounded,
    Aerial,
}

#[derive(Component, PartialEq, Serialize, Deserialize)]
#[require(Transform)]
pub struct Player {
    pub state: PlayerState,
    pub look_dir: Vec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player)
            .add_systems(Update, (look_player, move_camera));
    }
}

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
            y: newlook.y.clamp(-PI * 0.4, PI * 0.4),
        };

        player.look_dir = clamped;
    }
}

fn move_player(
    mut qp: Query<(
        &ActionState<NetworkedInput>,
        &mut LinearVelocity,
        &ShapeHits,
        &mut Player,
    )>,
) {
    for (action, mut linear, hits, mut player) in qp.iter_mut() {
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

        let mut axis = action.axis_pair(&NetworkedInput::Move);
        let jump = action.pressed(&NetworkedInput::Jump);

        axis *= speed;

        // let sin = ops::sin(player.look_dir.x);
        // let cos = ops::cos(player.look_dir.x);

        // linear.x += (axis.x * cos) + (axis.y * sin);
        // linear.z += (axis.y * cos) + (axis.x * sin);
        //
        linear.x += -axis.x;
        linear.z += axis.y;

        // info!("{:?}", player.look_dir.x);

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

        if jump && player.state == PlayerState::Grounded {
            linear.y = jump_height;
        }

        // linear.y += gravity * time.delta_secs();

        if linear.y < max_fall {
            linear.y = max_fall;
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

    let (player_transform, player) = qp.single();
    let mut camera_transform = qc.single_mut();

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
    pub player_id: PlayerId,
    pub input: InputMap<NetworkedInput>,
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
                look_dir: Vec2::default(),
            },
            player_id: PlayerId {
                id: lightyear::prelude::ClientId::Local(0),
            },

            input: InputMap::new([(NetworkedInput::Jump, KeyCode::Space)])
                .with_dual_axis(NetworkedInput::Move, VirtualDPad::wasd())
                .with_dual_axis(NetworkedInput::Look, MouseMove::default()),

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
