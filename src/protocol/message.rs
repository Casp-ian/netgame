use bevy::app::App;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub fn register_messages(app: &mut App) {
    app.register_message::<ChatMessage>(ChannelDirection::Bidirectional);
    app.register_message::<RegisterMessage>(ChannelDirection::Bidirectional);

    app.add_channel::<ChatChannel>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        ..Default::default()
    });

    // TODO use
    app.add_channel::<RegisterChannel>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        ..Default::default()
    });
}

// CHANNELS -----------------------------------------------------------------------------------------

#[derive(Channel)]
pub struct ChatChannel;

#[derive(Channel)]
pub struct RegisterChannel;

// MESSAGES -----------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RegisterMessage {
    pub name: String,
}
