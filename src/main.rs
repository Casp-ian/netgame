use avian3d::PhysicsPlugins;
use bevy::log::{Level, LogPlugin};
use bevy::state::app::StatesPlugin;
use bevy::{
    prelude::*,
    winit::{UpdateMode::Continuous, WinitSettings},
};

mod protocol;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;

mod shared;
use shared::SharedPlugins;

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    eprintln!("Shouldnt have both client and server features!!!");
    // compile_error!("cant have both client and server features at the same time");

    let mut app = App::new();

    #[cfg(feature = "client")]
    app.add_plugins((
        client::ClientPlugins,
        // Default
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Client".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }),
    ));

    #[cfg(feature = "server")]
    app.add_plugins((
        server::ServerPlugins,
        // Default
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Server".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        // MinimalPlugins,
        // StatesPlugin,
        // LogPlugin::default(),
    ));

    app.add_plugins((
        protocol::ProtocolPlugin,
        PhysicsPlugins::default(),
        SharedPlugins,
    ));

    // Makes the server/client update continuously even while unfocused.
    app.insert_resource(WinitSettings {
        focused_mode: Continuous,
        unfocused_mode: Continuous,
    });

    app.run();
}
