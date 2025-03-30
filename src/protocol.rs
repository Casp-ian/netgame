use avian3d::prelude::{AngularVelocity, LinearVelocity, Position, Rotation};
use bevy::{app::Plugin, prelude::*};
use client::{ComponentSyncMode, LerpFn};
use serde::{Deserialize, Serialize};

use lightyear::{prelude::*, utils::bevy::TransformLinearInterpolation};

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_message::<ChatMessage>(ChannelDirection::Bidirectional);

        app.register_component::<Controller>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<ProjectileId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(|start, end, t| Position(start.lerp(**end, t)))
            .add_correction_fn(|start, end, t| Position(start.lerp(**end, t)));

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(|start, end, t| Rotation(*start.slerp(*end, t)))
            .add_correction_fn(|start, end, t| Rotation(*start.slerp(*end, t)));

        app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);

        // mesh3d: Mesh3d::default(),
        // mesh_material3d: MeshMaterial3d::default(),

        app.add_channel::<ChatChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}

// COMPONENTS ---------------------------------------------------------------------------------------

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId {
    pub id: ClientId,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectileId {
    pub id: u32,
}

// CHANNELS -----------------------------------------------------------------------------------------

#[derive(Channel)]
pub struct ChatChannel;

// MESSAGES -----------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub text: String,
}

// INPUTS -------------------------------------------------------------------------------------------

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Controller {
    pub whish_dir: Vec2,
    pub jump: bool,
}

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
