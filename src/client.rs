use bevy::{
    app::Plugin,
    prelude::{AppExtStates, States},
};

mod chat;
mod input;
mod menu;
mod network;
mod oneshot;
mod predicted;

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(ClientGameState::MainMenu);
        app.init_resource::<oneshot::ClientOneshotSystems>();
        app.add_plugins((
            chat::ChatPlugin,
            input::InputPlugin,
            menu::MenuPlugin,
            network::ClientNetworkPlugin,
            predicted::PredictedPlugin,
        ));
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, States)]
pub enum ClientGameState {
    MainMenu,
    Game,
}
