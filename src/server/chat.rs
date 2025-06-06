use bevy::prelude::*;
use lightyear::prelude::*;

use crate::protocol::{
    component::PlayerId,
    message::{ChatChannel, ChatMessage},
};

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

    players: Query<&PlayerId>,

    systems: Res<ServerOneshotSystems>,
) {
    for event in events.read() {
        let text: &String = &event.message().text;

        let name = players
            .iter()
            .find(|x| x.id == event.from)
            .expect("sender is no longer in the game")
            .name
            .clone();

        // skip empty
        if text.len() == 0 {
            continue;
        }

        let mut chars = text.chars();
        if chars.next().unwrap() == '/' {
            let command: String = chars.collect();
            let system = systems.list.get(&command);
            if let Some(system) = system {
                commands.run_system(*system);
                do_chat(
                    &mut connection_manager,
                    format!("Command `{}` succeeded", text),
                    NetworkTarget::Single(event.from()),
                );
            } else {
                do_chat(
                    &mut connection_manager,
                    format!("Command `{}` failed", text),
                    NetworkTarget::Single(event.from()),
                );
            }

            return;
        }

        do_chat(
            &mut connection_manager,
            format!("{}: {}", name, text).to_string(),
            NetworkTarget::All,
        );
    }
}

fn do_chat(
    connection_manager: &mut ResMut<lightyear::prelude::server::ConnectionManager>,
    message: String,
    target: NetworkTarget,
) {
    connection_manager
        .send_message_to_target::<ChatChannel, ChatMessage>(&ChatMessage { text: message }, target)
        .unwrap();
}
