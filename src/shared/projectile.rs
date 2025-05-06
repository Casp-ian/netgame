use avian3d::prelude::{Collider, CollidingEntities, Restitution, RigidBody};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, despawn_projectile);
    }
}

fn despawn_projectile(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Projectile, &CollidingEntities)>,
) {
    for (entity, mut projectile, collisions) in projectiles.iter_mut() {
        // info!("A: {:?}", collisions);
        projectile.bounces -= collisions.len() as i32;

        if projectile.bounces <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    bounces: i32,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub collisions: CollidingEntities,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub collider: Collider,
}

// TODO i would make a scene out of this instead of a bundle
impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Projectile { bounces: 50 },
            collisions: CollidingEntities::default(),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::new(0.8),
            collider: Collider::sphere(0.25),
        }
    }
}
