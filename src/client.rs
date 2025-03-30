use bevy::app::Plugin;

mod chat;
mod network;
mod render;

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            network::ClientNetworkPlugin,
            chat::ChatPlugin,
            render::RenderPlugin,
        ));
    }
}
