use bevy::{
    input::{ButtonState, keyboard::Key, keyboard::KeyboardInput},
    prelude::*,
};
use lightyear::prelude::*;

use crate::protocol::message::{ChatChannel, ChatMessage};

pub struct ChatPlugin;

#[derive(Resource)]
struct Chatting {
    // TODO disable movement, and show chat window
    opened: bool,
    message: String,
}

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Chatting {
            opened: false,
            message: String::new(),
        });
        app.add_systems(Update, (print_chat, read_keys));
    }
}

fn print_chat(mut reader: EventReader<ClientReceiveMessage<ChatMessage>>) {
    for event in reader.read() {
        info!("Received message: {}", event.message().text);
    }
}

fn read_keys(
    mut client: ResMut<ClientConnectionManager>,
    mut chatting: ResMut<Chatting>,
    mut buttons: EventReader<KeyboardInput>,
) {
    for event in buttons.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        if !chatting.opened {
            if event.logical_key == Key::Enter {
                chatting.opened = true;
            }
            continue;
        }

        match &event.logical_key {
            // Handle pressing Enter to finish the input
            Key::Enter => {
                client
                    .send_message::<ChatChannel, ChatMessage>(&ChatMessage {
                        text: chatting.message.clone(),
                    })
                    .unwrap();
                chatting.message.clear();
                chatting.opened = false;
            }
            // Handle pressing Backspace to delete last char
            Key::Backspace => {
                chatting.message.pop();
            }
            Key::Space => {
                chatting.message.push(' ');
            }
            // Handle key presses that produce text characters
            Key::Character(input) => {
                // Ignore any input that contains control (special) characters
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                chatting.message.push_str(&input);
            }
            _ => {}
        } // if button == &Key::Return {}
    }
}
