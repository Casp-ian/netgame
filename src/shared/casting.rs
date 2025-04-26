use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::*;

use crate::protocol::{REPLICATION_GROUP, component::ProjectileId, input::NetworkedInput};

use super::{player::Player, projectile::ProjectileBundle};

pub struct CastingPlugin;

impl Plugin for CastingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (chant, cast).chain());
        // app.add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct Caster {
    spell: bool,
    timer: f32,
}
impl Caster {
    pub fn new() -> Caster {
        return Caster {
            spell: false,
            timer: 0.0,
        };
    }
}

fn chant(mut qp: Query<(&ActionState<NetworkedInput>, &mut Caster)>) {
    for (action, mut caster) in qp.iter_mut() {
        if action.pressed(&NetworkedInput::Fire) {
            caster.spell = true;
            caster.timer = 0.0;
        }
    }
}

fn cast(
    mut commands: Commands,
    mut qp: Query<(&LinearVelocity, &Transform, &Player, &mut Caster), With<LinearVelocity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (vel, pos, player, mut caster) in qp.iter_mut() {
        if caster.spell {
            caster.spell = false;

            let distance = 1.;
            let speed = 5.;
            let up_speed = 5.;

            let player_pos = pos.translation;

            let quat_x = Quat::from_axis_angle(Vec3::Y, player.look_dir.x);
            let quat_y = Quat::from_axis_angle(quat_x.mul_vec3(Vec3::X), player.look_dir.y);

            let pos_diff = quat_x.mul_vec3(Vec3::Z * distance);
            let vel_diff = quat_y.mul_vec3(quat_x.mul_vec3(Vec3::Z * speed + Vec3::Y * up_speed));

            let spawn_pos: Vec3 = player_pos + pos_diff;

            let replicate = ServerReplicate {
                group: REPLICATION_GROUP,
                sync: server::SyncTarget {
                    prediction: NetworkTarget::All,
                    interpolation: NetworkTarget::None,
                },
                ..default()
            };

            commands.spawn((
                replicate,
                PreSpawnedPlayerObject::default(),
                ProjectileId { id: 0 },
                Transform::from_translation(spawn_pos),
                ProjectileBundle { ..default() },
                LinearVelocity(vel_diff + vel.0),
                Mesh3d(meshes.add(Sphere::new(0.25))),
                MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
            ));
        }
        // test
    }
}
