use bevy::prelude::*;

pub mod player;

pub struct WorksPlugin;

impl Plugin for WorksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::move_player)
            .add_systems(Update, player::move_camera);
    }
}
