use bevy::prelude::*;
use lightyear::prelude::*;

use crate::protocol::message::{ChatChannel, ChatMessage};

use super::oneshot::ServerOneshotSystems;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, forward_chat);
    }
}

fn forward_chat(
    mut commands: Commands,
    mut connection_manager: ResMut<lightyear::prelude::server::ConnectionManager>,
    mut events: EventReader<FromClients<ChatMessage>>,
    systems: Res<ServerOneshotSystems>,
) {
    for event in events.read() {
        let text = &event.message().text;
        if text.len() == 0 {
            continue;
        }

        let mut chars = text.chars();
        if chars.next().unwrap() == '/' {
            let command: String = chars.collect();
            let system = systems.list.get(&command);
            if let Some(system) = system {
                commands.run_system(*system);
            }
        }
        connection_manager
            .send_message_to_target::<ChatChannel, ChatMessage>(
                &ChatMessage {
                    text: format!("{}: {}", event.from(), event.message().text).to_string(),
                },
                NetworkTarget::All,
            )
            .unwrap();
    }
}
