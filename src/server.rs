use bevy::app::Plugin;

mod chat;
mod network;
mod oneshot;
mod spawn;

pub struct ServerPlugins;

impl Plugin for ServerPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<oneshot::ServerOneshotSystems>();
        app.add_plugins((
            network::ServerNetworkPlugin,
            spawn::SpawnPlugin,
            chat::ChatPlugin,
        ));
    }
}
