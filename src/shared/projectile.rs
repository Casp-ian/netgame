use avian3d::prelude::{Collider, Restitution, RigidBody};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, despawn_projectile);
    }
}

fn despawn_projectile(mut commands: Commands, mut projectiles: Query<(Entity, &mut Projectile)>) {
    for (entity, mut projectile) in projectiles.iter_mut() {
        projectile.bounces -= 1;

        if projectile.bounces <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub bounces: i32,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub collider: Collider,
}

// TODO i would make a scene out of this instead of a bundle
impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Projectile { bounces: 300 },
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::new(0.8),
            collider: Collider::sphere(0.75),
        }
    }
}
