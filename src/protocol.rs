use bevy::{app::Plugin, prelude::*};
use serde::{Deserialize, Serialize};

use lightyear::prelude::{client::ComponentSyncMode, *};

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        // app.register_component::<Camera3d>(ChannelDirection::ClientToServer);

        app.register_message::<ChatMessage>(ChannelDirection::Bidirectional);

        // app.add_plugins(InputManagerPlugin::<Inputs>::default());
    }
}

// COMPONENTS ---------------------------------------------------------------------------------------

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId {
    id: u32,
}

// MESSAGES -----------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChatMessage {
    sender: u32,
    text: String,
}

// INPUTS -------------------------------------------------------------------------------------------

// /// The different directions that the player can move the box
// #[derive(Actionlike, Hash, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Reflect)]
// pub struct Direction {
//     pub(crate) up: bool,
//     pub(crate) down: bool,
//     pub(crate) left: bool,
//     pub(crate) right: bool,
// }

// /// The `InputProtocol` needs to be an enum of the various inputs that the client can send to the server.
// #[derive(Actionlike, Hash, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Reflect)]
// pub enum Inputs {
//     Direction(Direction),
//     Jump,
//     /// NOTE: we NEED to provide a None input so that the server can distinguish between lost input packets and 'None' inputs
//     None,
// }
