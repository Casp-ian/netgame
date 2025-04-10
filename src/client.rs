use bevy::{
    app::Plugin,
    prelude::{AppExtStates, States},
};

mod chat;
mod input;
mod menu;
mod network;
mod render;

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.insert_state(ClientGameState::MainMenu);
        app.add_plugins((
            chat::ChatPlugin,
            input::InputPlugin,
            menu::MenuPlugin,
            network::ClientNetworkPlugin,
            render::RenderPlugin,
        ));
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, States)]
pub enum ClientGameState {
    MainMenu,
    Game,
}
