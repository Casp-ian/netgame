use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::client::ComponentSyncMode;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::shared::{casting::Caster, enemy::Enemy, player::Player};

pub fn register_components(app: &mut App) {
    // General positional
    app.register_component::<Transform>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<Position>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<Rotation>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    // Player
    app.register_component::<Player>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<Caster>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<Enemy>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    // Ids
    app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once);

    app.register_component::<EnemyId>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once);

    app.register_component::<ProjectileId>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once);
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId {
    pub id: ClientId,
    pub name: String,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EnemyId {
    pub id: u32,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProjectileId {
    pub id: u32,
}
