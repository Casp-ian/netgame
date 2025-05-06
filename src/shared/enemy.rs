use avian3d::prelude::{LinearVelocity, Position, RigidBody};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                enemy_tick,
                // TODO does timer work well on server client??
                // agro.run_if(on_timer(Duration::from_secs(5))),
                agro,
            ),
        );
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(Transform)]
pub struct Enemy {
    goal_pos: Vec3,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub rigid_body: RigidBody,
}

// TODO i would make a scene out of this instead of a bundle
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy {
                goal_pos: Vec3::ZERO,
            },
            rigid_body: RigidBody::Dynamic,
        }
    }
}

fn enemy_tick(
    // a
    mut enemies: Query<(&Enemy, &Position, &mut LinearVelocity)>,
) {
    let enemy_speed = 4.0;

    for (enemy, pos, mut velocity) in enemies.iter_mut() {
        let direction: Vec3 = enemy.goal_pos - pos.0;
        velocity.0 = direction.normalize_or_zero() * enemy_speed;
    }
}

fn agro(
    // a
    players: Query<&Position, With<Player>>,
    mut enemies: Query<(&Position, &mut Enemy)>,
) {
    for (_, mut enemy) in enemies.iter_mut() {
        let closest_player_pos = players.iter().next();
        if let Some(player_pos) = closest_player_pos {
            enemy.goal_pos = player_pos.0;
        }
    }
    // a
}
