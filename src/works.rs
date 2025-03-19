use bevy::prelude::*;

pub mod player;

pub struct WorksPlugin;

impl Plugin for WorksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player::spawn_player)
            .add_systems(Update, player::move_camera)
            .add_systems(Update, player::move_player);
    }
}
