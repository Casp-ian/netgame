use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::client::{ComponentSyncMode, LerpFn};
use lightyear::{prelude::*, utils::bevy::TransformLinearInterpolation};
use serde::{Deserialize, Serialize};

pub fn register_components(app: &mut App) {
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

    // app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId {
    pub id: ClientId,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectileId {
    pub id: u32,
}
