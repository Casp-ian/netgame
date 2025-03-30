use bevy::app::Plugin;

mod network;
use network::*;
mod spawn;
use spawn::*;
mod chat;
use chat::*;

pub struct ServerPlugins;

impl Plugin for ServerPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((ServerNetworkPlugin, SpawnPlugin, ChatPlugin));
    }
}
