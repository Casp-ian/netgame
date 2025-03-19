use avian3d::prelude::*;
use bevy::prelude::*;

pub mod content;
pub mod works;
use content::ContentPlugin;
use works::WorksPlugin;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(ContentPlugin)
        .add_plugins(WorksPlugin)
        // systems
        .run();
}
