use avian3d::PhysicsPlugins;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::prelude::*;
use bevy::winit::{UpdateMode::Continuous, WinitSettings};

mod protocol;

#[cfg(feature = "client")]
mod client;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
use bevy::{log::LogPlugin, state::app::StatesPlugin};

mod shared;
use shared::SharedPlugins;

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    eprintln!("Shouldnt have both client and server features!!!");
    // compile_error!("cant have both client and server features at the same time");

    let mut app = App::new();

    #[cfg(feature = "client")]
    app.add_plugins((
        // Default
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Client".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        PhysicsPlugins::default(),
        client::ClientPlugins,
    ));

    #[cfg(feature = "server")]
    app.add_plugins((
        // Default
        // DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "Server".to_string(),
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // }),
        MinimalPlugins,
        StatesPlugin,
        LogPlugin::default(),
        DiagnosticsPlugin,
        PhysicsPlugins::default(),
        server::ServerPlugins,
    ));

    app.add_plugins((protocol::ProtocolPlugin, SharedPlugins));

    // Makes the server/client update continuously even while unfocused.
    app.insert_resource(WinitSettings {
        focused_mode: Continuous,
        unfocused_mode: Continuous,
    });

    app.run();
}
