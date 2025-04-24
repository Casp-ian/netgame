use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

use crate::client::ClientGameState;

pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (focus_textbox, read_keys).run_if(in_state(ClientGameState::MainMenu)),
        );
    }
}

#[derive(Component)]
#[require(Text)]
pub struct Textbox {
    pub focused: bool,
}

impl Default for Textbox {
    fn default() -> Self {
        Self { focused: false }
    }
}

fn focus_textbox(
    mut interaction_query: Query<(&Interaction, &mut Textbox, Entity), (Changed<Interaction>)>,
) {
    for (interaction, mut textbox, entity) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("focusing {:?}", entity);
                textbox.focused = true;
                // TODO unfocus others
            }
            _ => {
                // textbox.focused = false;
            }
        }
    }
}

fn read_keys(
    mut buttons: EventReader<KeyboardInput>,
    mut textboxes: Query<(&mut Text, &mut Textbox)>,
) {
    for event in buttons.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        // NOTE might be weird behaviour if multiple textboxes are focused at the same time
        let result = textboxes.iter_mut().find(|(_, textbox)| textbox.focused);

        if result.is_none() {
            continue;
        }

        let (mut text, mut textbox) = result.unwrap();

        match &event.logical_key {
            // Handle pressing Enter to finish the input
            Key::Enter | Key::Escape => {
                // TODO this is disabled for as long as there will be only one text box anyways, so why ever unfocus
                // textbox.focused = false;
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
        }
    }
}
