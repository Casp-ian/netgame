use bevy::{
    input::{ButtonState, keyboard::Key, keyboard::KeyboardInput},
    prelude::*,
};
use leafwing_input_manager::prelude::ActionState;
use lightyear::{prelude::*, shared::replication::components::Controlled};

use crate::protocol::{
    input::NetworkedInput,
    message::{ChatChannel, ChatMessage},
};

use super::ClientGameState;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(ChatState::Closed);

        app.add_systems(Startup, create_chat_window);
        app.add_systems(
            Update,
            (print_chat, read_keys).run_if(in_state(ClientGameState::Game)),
        );

        app.add_systems(OnEnter(ChatState::Opened), open_chat);
        app.add_systems(OnEnter(ChatState::Closed), close_chat);
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, States)]
pub enum ChatState {
    Opened,
    Closed,
}

fn open_chat(
    mut controls: Query<&mut ActionState<NetworkedInput>, With<Controlled>>,
    mut chat: Query<&mut Visibility, Or<(With<ChatField>, With<ChatLog>)>>,
) {
    for mut vis in chat.iter_mut() {
        *vis = Visibility::Visible;
    }
    if let Ok(mut control) = controls.single_mut() {
        // TODO
        // Enable and disable work 90% of the time...
        // it might be system ordering or networking causing it
        ActionState::disable(&mut control);
    }
}

fn close_chat(
    mut controls: Query<&mut ActionState<NetworkedInput>, With<Controlled>>,
    mut chat: Query<&mut Visibility, Or<(With<ChatField>, With<ChatLog>)>>,
) {
    for mut vis in chat.iter_mut() {
        *vis = Visibility::Hidden;
    }
    if let Ok(mut control) = controls.single_mut() {
        ActionState::enable(&mut control);
    }
}

#[derive(Component)]
struct ChatField;
#[derive(Component)]
struct ChatLog;

fn create_chat_window(mut commands: Commands) {
    commands
        .spawn((
            Visibility::Hidden,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ChatField,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::default(),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent.spawn((
                ChatLog,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(90.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::default(),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn print_chat(
    mut chatlog: Query<&mut Text, With<ChatLog>>,
    mut reader: EventReader<ClientReceiveMessage<ChatMessage>>,
) {
    let mut log = chatlog.single_mut().unwrap();
    for event in reader.read() {
        log.push_str(&event.message().text);
        log.push('\n');
    }
}

fn read_keys(
    mut client: ResMut<ClientConnectionManager>,

    state: Res<State<ChatState>>,
    mut next_state: ResMut<NextState<ChatState>>,
    mut chat_field: Query<&mut Text, With<ChatField>>,

    mut buttons: EventReader<KeyboardInput>,
) {
    let mut text = chat_field.single_mut().unwrap();

    for event in buttons.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        if state.get() == &ChatState::Closed {
            if event.logical_key == Key::Enter {
                next_state.set(ChatState::Opened);
            }
            continue;
        }

        match &event.logical_key {
            // Handle pressing Enter to finish the input
            Key::Enter => {
                if text.0.len() > 0 {
                    client
                        .send_message::<ChatChannel, ChatMessage>(&ChatMessage {
                            text: text.0.clone(),
                        })
                        .unwrap();
                }
                text.0.clear();
                next_state.set(ChatState::Closed);
            }
            // Handle pressing Backspace to delete last char
            Key::Backspace => {
                text.0.pop();
            }
            Key::Space => {
                text.0.push(' ');
            }
            // Handle key presses that produce text characters
            Key::Character(input) => {
                // Ignore any input that contains control (special) characters
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                text.0.push_str(&input);
            }
            _ => {}
        } // if button == &Key::Return {}
    }
}
