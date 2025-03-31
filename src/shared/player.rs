use avian3d::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualDPad};
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
}

#[derive(Component, PartialEq, Serialize, Deserialize)]
#[require(Transform)]
pub struct Head;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player)
            .add_systems(Update, move_camera);
    }
}

pub fn move_player(
    // time: Res<Time>,
    mut qp: Query<
        (
            &ActionState<NetworkedInput>,
            &mut LinearVelocity,
            &ShapeHits,
            &mut Player,
        ),
        Without<Head>,
    >,
) {
    if qp.is_empty() {
        // info!("B");
        return;
    }

    let (action, mut linear, hits, mut player) = qp.single_mut();

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

    let axis;
    if let Some(test) = action.dual_axis_data(&NetworkedInput::Move) {
        axis = test.pair;
    } else {
        axis = Vec2::ZERO;
    }

    let jump;
    if let Some(test) = action.button_data(&NetworkedInput::Jump) {
        jump = test.state.pressed();
    } else {
        jump = false;
    }

    linear.x += axis.x * speed;
    linear.z += axis.y * speed;

    // info!("{:?}", linear);

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
            },
            player_id: PlayerId {
                id: lightyear::prelude::ClientId::Local(0),
            },

            input: InputMap::new([(NetworkedInput::Jump, KeyCode::Space)])
                .with_dual_axis(NetworkedInput::Move, VirtualDPad::wasd()),

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
