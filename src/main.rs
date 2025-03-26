use avian3d::PhysicsPlugins;
use bevy::log::{Level, LogPlugin};
use bevy::state::app::StatesPlugin;
use bevy::{
    prelude::*,
    winit::{UpdateMode::Continuous, WinitSettings},
};

use lightyear::prelude::*;

mod protocol;
use protocol::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;

mod shared;
use shared::map::MapPlugin;
use shared::player::PlayerPlugin;

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    eprintln!("Shouldnt have both client and server features!!!");
    // compile_error!("cant have both client and server features at the same time");

    let mut app = App::new();

    #[cfg(feature = "client")]
    app.add_plugins((
        client::ClientPlugins,
        // DefaultPlugins,
    ));

    #[cfg(feature = "server")]
    app.add_plugins((
        server::ServerPlugins,
        // MinimalPlugins,
        // StatesPlugin,
        // LogPlugin::default(),
    ));

    app
        // Makes the server/client update continuously even while unfocused.
        .add_plugins((
            DefaultPlugins,
            ProtocolPlugin,
            PhysicsPlugins::default(),
            PlayerPlugin,
            MapPlugin,
        ));

    app.insert_resource(WinitSettings {
        focused_mode: Continuous,
        unfocused_mode: Continuous,
    });

    app.run();
}
