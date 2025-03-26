use bevy::app::Plugin;

mod network;
use network::*;
mod spawn;
use spawn::*;

pub struct ServerPlugins;

impl Plugin for ServerPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((ServerNetworkPlugin, SpawnPlugin));
    }
}
