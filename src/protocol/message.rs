use bevy::app::App;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub fn register_messages(app: &mut App) {
    app.register_message::<ChatMessage>(ChannelDirection::Bidirectional);

    app.add_channel::<ChatChannel>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        ..Default::default()
    });
}

// CHANNELS -----------------------------------------------------------------------------------------

#[derive(Channel)]
pub struct ChatChannel;

// MESSAGES -----------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub text: String,
}
