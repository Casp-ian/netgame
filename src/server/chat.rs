use bevy::prelude::*;
use lightyear::prelude::*;

use crate::protocol::message::{ChatChannel, ChatMessage};

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, forward_chat);
    }
}

fn forward_chat(
    mut connection_manager: ResMut<lightyear::prelude::server::ConnectionManager>,
    mut events: EventReader<FromClients<ChatMessage>>,
) {
    for event in events.read() {
        connection_manager
            .send_message_to_target::<ChatChannel, ChatMessage>(
                &ChatMessage {
                    text: format!("[{}]: {}", event.from(), event.message().text).to_string(),
                },
                NetworkTarget::All,
            )
            .unwrap();
    }
}
