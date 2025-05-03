use avian3d::prelude::{Collider, Restitution, RigidBody};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub collider: Collider,
    pub visibility: Visibility,
}

// TODO i would make a scene out of this instead of a bundle
impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::new(0.8),
            collider: Collider::sphere(0.25),
            visibility: Visibility::Visible,
        }
    }
}
