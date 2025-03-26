use bevy::app::Plugin;

mod chat;
mod network;

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((network::ClientNetworkPlugin, chat::ChatPlugin));
    }
}
