use bevy::prelude::*;
use lightyear::prelude::*;

use crate::protocol::message::{ChatChannel, ChatMessage};

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (print_chat, send_chat));
    }
}

fn print_chat(mut reader: EventReader<ClientReceiveMessage<ChatMessage>>) {
    for event in reader.read() {
        info!("Received message: {}", event.message().text);
    }
}

fn send_chat(keys: Res<ButtonInput<KeyCode>>, mut client: ResMut<ClientConnectionManager>) {
    if keys.pressed(KeyCode::Enter) {
        client
            .send_message::<ChatChannel, ChatMessage>(&ChatMessage {
                text: "Fuck you!!!".to_string(),
            })
            .unwrap();
    }
}
