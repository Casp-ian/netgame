use bevy::prelude::*;
use lightyear::prelude::*;

use crate::protocol::ChatMessage;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, print_chat);
    }
}

fn print_chat(mut reader: EventReader<ClientReceiveMessage<ChatMessage>>) {
    for event in reader.read() {
        info!("Received message: {}", event.message().text);
    }
}
