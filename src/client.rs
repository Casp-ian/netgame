use bevy::app::Plugin;

mod chat;
mod input;
mod network;
mod render;

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            chat::ChatPlugin,
            input::InputPlugin,
            network::ClientNetworkPlugin,
            render::RenderPlugin,
        ));
    }
}
